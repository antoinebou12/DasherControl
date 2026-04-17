<template>
  <div v-if="token !== ''" class="workspace-selector-container">
    <span class="toggle-hide" @click="isVisible = !isVisible">
      <i :class="isVisible ? 'bx bx-up-arrow' : 'bx bx-down-arrow'" class="bx" />
    </span>
    <n-card v-if="isVisible" size="small" class="workspace-card" title="Workspaces">
      <n-scrollbar style="max-height: 220px">
        <n-list hoverable clickable>
          <n-list-item
            v-for="workspace in workspaces"
            :key="workspace.id"
            :class="{ selected: selected.id === workspace.id }"
            @click="selectWorkspace(workspace)"
          >
            <n-thing :title="workspace.name">
              <template #avatar>
                <span
                  v-if="workspace.workspace_icon && !isBoxicon(workspace.workspace_icon)"
                  class="ws-avatar ws-avatar--emoji"
                >{{ workspace.workspace_icon }}</span>
                <i
                  v-else-if="workspace.workspace_icon"
                  class="ws-avatar bx"
                  :class="boxiconClasses(workspace.workspace_icon)"
                />
                <i v-else class="ws-avatar bx bx-grid-alt ws-avatar--muted" />
              </template>
              <template #header-extra>
                <n-button quaternary size="tiny" @click.stop="deleteWorkspace(workspace.id)">
                  <i class="bx bx-x" />
                </n-button>
              </template>
            </n-thing>
          </n-list-item>
        </n-list>
      </n-scrollbar>
      <n-input
        v-model:value="workspaceName"
        class="workspace-name-input"
        placeholder="Press Enter to save name"
        @keyup.enter="editWorkspaceName"
      />
    </n-card>
  </div>
</template>

<script>
import axios from "axios";
import { NButton, NCard, NInput, NList, NListItem, NScrollbar, NThing } from "naive-ui";
import { mapState } from "pinia";
import { useUserStore } from "../stores/user";
import { createNotification } from "../utils";
import { boxiconClassList, isBoxiconClass } from "../utils/workspaceTheme";

export default {
  name: "WorkspaceSelector",
  components: { NButton, NCard, NInput, NList, NListItem, NScrollbar, NThing },
  data() {
    return {
      selected: { name: "", id: -1 },
      workspaces: [],
      isVisible: true,
      workspaceName: "",
    };
  },
  computed: {
    ...mapState(useUserStore, ["token"]),
  },
  watch: {
    isVisible() {
      this.get_workspaces();
    },
    selected(newVal) {
      if (!newVal || newVal.id == null) {
        this.workspaceName = "";
        return;
      }
      this.workspaceName = newVal.name;
      this.$emit("selected");
    },
  },
  created() {
    if (this.token) this.get_workspaces();
  },
  mounted() {
    if (this.workspaces.length && this.token) {
      this.selected = this.workspaces[0];
    }
  },
  methods: {
    isBoxicon(icon) {
      return isBoxiconClass(icon || "");
    },
    boxiconClasses(icon) {
      return boxiconClassList(icon || "");
    },
    selectWorkspace(workspace) {
      this.selected = workspace;
    },
    get_workspaces() {
      axios({
        method: "get",
        url: "/workspaces/api/list",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${this.token}`,
        },
      })
        .then((response) => {
          this.workspaces = response.data;
          if (!this.workspaces.length) {
            this.selected = { name: "", id: -1 };
            return;
          }
          const still = this.workspaces.some((w) => w.id === this.selected.id);
          if (this.selected.id === -1 || !still) {
            this.selected = this.workspaces[0];
          }
        })
        .catch(() => {
          this.workspaces = [];
        });
    },
    editWorkspaceName() {
      this.$emit("updateName");
    },
    deleteWorkspace(id) {
      axios({
        method: "post",
        url: `/workspaces/api/delete/${id}`,
        headers: {
          Authorization: `Bearer ${this.token}`,
        },
      })
        .then(() => {
          createNotification(null, "Workspace deleted", "", "primary");
          this.get_workspaces();
        })
        .catch(() => {
          createNotification(null, "Error deleting workspace", "", "danger");
          this.get_workspaces();
        });
    },
  },
};
</script>

<style scoped lang="scss">
.workspace-selector-container {
  color: var(--fg);
  position: relative;
}
.toggle-hide {
  position: absolute;
  top: 56px;
  right: 24px;
  cursor: pointer;
}
.workspace-card {
  margin-top: 8px;
  background: rgba(0, 0, 0, 0.25);
}
.selected {
  background: rgba(98, 114, 164, 0.25);
}
.workspace-name-input {
  margin-top: 8px;
}
.ws-avatar {
  font-size: 1.25rem;
  line-height: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 2rem;
  height: 2rem;
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.08);
}
.ws-avatar--emoji {
  font-size: 1.1rem;
}
.ws-avatar--muted {
  opacity: 0.45;
}
</style>
