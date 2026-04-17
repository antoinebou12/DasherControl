<template>
  <div v-if="token !== ''" class="workspace-selector-container">
    <v-btn
      class="toggle-hide"
      icon
      size="small"
      variant="text"
      aria-label="Toggle workspace list"
      @click="isVisible = !isVisible"
    >
      <v-icon :icon="isVisible ? 'mdi-chevron-up' : 'mdi-chevron-down'" />
    </v-btn>
    <v-expand-transition>
      <v-card v-if="isVisible" class="workspace-card mt-2" variant="outlined" rounded="lg">
        <v-card-title class="text-subtitle-1 py-2">Workspaces</v-card-title>
        <v-divider />
        <v-list density="compact" class="workspace-list py-0" max-height="220" scrollable>
          <v-list-item
            v-for="workspace in workspaces"
            :key="workspace.id"
            :active="selected.id === workspace.id"
            color="primary"
            rounded="md"
            class="mb-1"
            @click="selectWorkspace(workspace)"
          >
            <template #prepend>
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
            <v-list-item-title>{{ workspace.name }}</v-list-item-title>
            <template #append>
              <v-btn
                icon
                size="x-small"
                variant="text"
                color="error"
                aria-label="Delete workspace"
                @click.stop="deleteWorkspace(workspace.id)"
              >
                <v-icon icon="mdi-close" size="small" />
              </v-btn>
            </template>
          </v-list-item>
        </v-list>
        <v-card-text class="pt-2 pb-3">
          <v-text-field
            v-model="workspaceName"
            placeholder="Press Enter to save name"
            density="compact"
            variant="outlined"
            hide-details
            rounded="pill"
            @keyup.enter="editWorkspaceName"
          />
        </v-card-text>
      </v-card>
    </v-expand-transition>
  </div>
</template>

<script>
import axios from "axios";
import { mapState } from "pinia";
import { useUserStore } from "../stores/user";
import { createNotification } from "../utils";
import { boxiconClassList, isBoxiconClass } from "../utils/workspaceTheme";

export default {
  name: "WorkspaceSelector",
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
  top: 8px;
  right: 4px;
  z-index: 1;
}
.workspace-card {
  background: var(--ui-chrome-bg) !important;
  border-color: var(--ui-chrome-border) !important;
  color: var(--ui-chrome-fg) !important;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.35);
}
.workspace-list {
  background: transparent;
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
