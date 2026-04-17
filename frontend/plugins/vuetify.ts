import "vuetify/styles";
import "@mdi/font/css/materialdesignicons.css";
import { createVuetify } from "vuetify";

export default createVuetify({
  theme: {
    defaultTheme: "dcDark",
    themes: {
      dcDark: {
        dark: true,
        colors: {
          background: "#282a36",
          surface: "#1e1e26",
          "surface-variant": "#2c2e3a",
          primary: "#2563eb",
          secondary: "#6272a4",
          error: "#ef4444",
          onBackground: "#f4f4f5",
          onSurface: "#f4f4f5",
        },
      },
    },
  },
});
