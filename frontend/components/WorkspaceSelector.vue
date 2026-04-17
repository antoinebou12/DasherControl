<template>
  <div v-if="token !== ''" class="workspace-selector-container">
    <span class="toggle-hide" @click="isVisible = !isVisible">
      <i :class="isVisible ? 'bx bx-up-arrow' : 'bx bx-down-arrow'" class="bx" />
    </span>
    <n-card v-if="isVisible" size="small" class="workspace-card workspace-card--dark" title="Workspaces">
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
                <n-button
                  quaternary
                  size="tiny"
                  class="ws-delete-btn"
                  aria-label="Delete workspace"
                  @click.stop="deleteWorkspace(workspace.id)"
                >
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
        round
        size="small"
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
  color: var(--ui-chrome-fg);
  position: relative;
}
.toggle-hide {
  position: absolute;
  top: 56px;
  right: 24px;
  cursor: pointer;
  color: var(--ui-chrome-fg);
}
.workspace-card {
  margin-top: 8px;
  background: var(--ui-chrome-bg) !important;
  border: 1px solid var(--ui-chrome-border) !important;
  color: var(--ui-chrome-fg) !important;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.35);
  overflow: hidden;
}
.workspace-card--dark :deep(.n-card-header) {
  background: rgba(0, 0, 0, 0.22) !important;
  padding: 10px 14px !important;
  border-bottom: 1px solid var(--ui-chrome-border) !important;
}
.workspace-card :deep(.n-card-header) {
  color: var(--ui-chrome-fg) !important;
  font-weight: 600;
}
.workspace-card :deep(.n-card__content) {
  padding: 8px 10px 0 !important;
}
.selected {
  background: color-mix(in srgb, var(--ui-accent) 16%, transparent) !important;
  outline: 1px solid color-mix(in srgb, var(--ui-accent) 45%, transparent);
  border-radius: 6px;
}
.workspace-name-input {
  margin-top: 10px;
  margin-bottom: 4px;
}
.workspace-name-input :deep(.n-input) {
  --n-color: rgba(255, 255, 255, 0.06) !important;
  --n-color-focus: rgba(255, 255, 255, 0.09) !important;
  --n-text-color: var(--ui-chrome-fg) !important;
  --n-placeholder-color: rgba(244, 244, 245, 0.45) !important;
  --n-border: 1px solid var(--ui-chrome-border) !important;
  --n-border-hover: 1px solid color-mix(in srgb, var(--ui-accent) 35%, var(--ui-chrome-border)) !important;
  --n-border-focus: 1px solid var(--ui-accent) !important;
  --n-height: 36px !important;
}
.workspace-card :deep(.ws-delete-btn:hover) {
  --n-text-color: var(--ui-danger) !important;
  color: var(--ui-danger) !important;
}
.workspace-card :deep(.ws-delete-btn .bx) {
  font-size: 1.1rem;
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
  border: 1px solid var(--ui-chrome-border);
  background: var(--ui-chrome-bg);
  color: var(--ui-chrome-fg);
}
.ws-avatar--emoji {
  font-size: 1.1rem;
}
.ws-avatar--muted {
  opacity: 1;
}
</style>
