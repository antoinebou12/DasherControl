import { createDiscreteApi, darkTheme } from "naive-ui";

const { notification } = createDiscreteApi(["notification"], {
  configProviderProps: { theme: darkTheme },
});

export function createNotification(
  _self: unknown,
  title: string,
  description: unknown,
  color = "dark",
) {
  const content = String(description ?? "");
  if (color === "danger") notification.error({ title, content, duration: 4500 });
  else if (color === "success") notification.success({ title, content, duration: 2500 });
  else notification.create({ title, content, duration: 3500 });
}
