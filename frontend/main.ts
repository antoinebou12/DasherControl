import { createApp } from "vue";
import { createPinia } from "pinia";
import "./components/applets/extensions";
import App from "./App.vue";
import vuetify from "./plugins/vuetify";

const app = createApp(App);
app.use(createPinia());
app.use(vuetify);
app.mount("#app");
