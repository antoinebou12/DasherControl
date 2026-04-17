import { defineStore } from "pinia";
import axios from "axios";

export const useUserStore = defineStore("user", {
  state: () => ({
    username: "" as string,
    email: "" as string,
    token: "" as string,
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
      if (this.token !== "") return;
      try {
        const { data } = await axios.get<string>("/tenants/api/token");
        if (data) this.setToken(data);
        else this.setToken("");
      } catch {
        this.setToken("");
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
