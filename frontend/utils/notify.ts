import { pushSnackbar, type SnackbarTone } from "./snackbarState";

export function createNotification(
  _self: unknown,
  title: string,
  description: unknown,
  color = "dark",
) {
  const content = String(description ?? "");
  let tone: SnackbarTone = "info";
  let ms = 3500;
  if (color === "danger") {
    tone = "error";
    ms = 4500;
  } else if (color === "success") {
    tone = "success";
    ms = 2500;
  } else if (color === "primary") {
    tone = "info";
    ms = 3500;
  }
  pushSnackbar(title, content, tone, ms);
}
