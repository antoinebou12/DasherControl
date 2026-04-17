import { defineStore } from "pinia";

/** Responsive shell: optional right tools rail (left reserved for future nav). */
export const useUiShellStore = defineStore("uiShell", {
  state: () => ({
    rightPanelOpen: false,
  }),
  actions: {
    toggleRightPanel() {
      this.rightPanelOpen = !this.rightPanelOpen;
    },
    setRightPanel(open: boolean) {
      this.rightPanelOpen = open;
    },
  },
});
