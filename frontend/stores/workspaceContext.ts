import { defineStore } from "pinia";

export type AppletTileSummary = {
  gridId: string | number;
  appletName: string;
};

export type WorkspaceContextState = {
  workspaceId: number;
  workspaceName: string;
  applets: AppletTileSummary[];
};

export const useWorkspaceContextStore = defineStore("workspaceContext", {
  state: (): WorkspaceContextState => ({
    workspaceId: -1,
    workspaceName: "",
    applets: [],
  }),
  actions: {
    setSession(workspaceId: number, workspaceName: string, applets: AppletTileSummary[]) {
      this.workspaceId = workspaceId;
      this.workspaceName = workspaceName;
      this.applets = applets;
    },
  },
});
