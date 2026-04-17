import { defineStore } from "pinia";
import axios from "axios";

export const useUserStore = defineStore("user", {
  state: () => ({
    username: "" as string,
    email: "" as string,
    token: "" as string,
    /** True after the first `getToken()` attempt finishes (avoids flashing guest UI while session is loading). */
    sessionResolved: false as boolean,
  }),
  actions: {
    setUser(payload: { username: string; email: string }) {
      this.username = payload.username;
      this.email = payload.email;
    },
    setToken(token: string) {
      this.token = token;
    },
    async getToken() {
      if (this.token !== "") {
        this.sessionResolved = true;
        return;
      }
      try {
        const { data } = await axios.get<string>("/tenants/api/token");
        if (data) this.setToken(data);
        else this.setToken("");
      } catch {
        this.setToken("");
      } finally {
        this.sessionResolved = true;
      }
    },
    async refreshProfile() {
      try {
        const { data } = await axios.get<{ email: string; username: string }>("/tenants/api/profile");
        this.setUser({ email: data.email, username: data.username });
      } catch {
        /* no session */
      }
    },
  },
});
