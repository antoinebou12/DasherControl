import { defineStore } from "pinia";
import {
  applyWorkspaceThemeToDocument,
  defaultWorkspaceAppearance,
  THEME_PRESETS,
  workspaceAppearanceFromApiRow,
  type WorkspaceAppearance,
} from "../utils/workspaceTheme";

export const useWorkspaceAppearanceStore = defineStore("workspaceAppearance", {
  state: (): WorkspaceAppearance => defaultWorkspaceAppearance(),
  actions: {
    preview() {
      applyWorkspaceThemeToDocument(this.$state);
    },
    hydrateFromApiRow(row: Record<string, unknown> | null | undefined) {
      this.$patch(workspaceAppearanceFromApiRow(row));
      this.preview();
    },
    resetToDefaults() {
      const preset = THEME_PRESETS.dracula;
      this.$patch({
        workspace_icon: "",
        theme_preset: "dracula",
        bg_color: preset.bg,
        fg_color: preset.fg,
        accent_color: preset.accent,
        panel_color: preset.panel,
        background_image_url: null,
      });
      this.preview();
    },
  },
});
