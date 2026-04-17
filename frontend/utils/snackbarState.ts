import { reactive } from "vue";

export type SnackbarTone = "error" | "success" | "info";

export const snackbarState = reactive({
  open: false,
  title: "",
  message: "",
  color: "error" as SnackbarTone,
  timeout: 4500,
});

export function pushSnackbar(
  title: string,
  message: string,
  tone: SnackbarTone = "info",
  timeout = 4500,
) {
  snackbarState.title = title;
  snackbarState.message = message;
  snackbarState.color = tone;
  snackbarState.timeout = timeout;
  snackbarState.open = true;
}
