import { expect, test } from "@playwright/test";

test.describe("Dark dashboard shell", () => {
  test("document exposes dark chrome and accent tokens", async ({ page }) => {
    await page.goto("/");
    const chromeBg = await page.evaluate(() =>
      getComputedStyle(document.documentElement).getPropertyValue("--ui-chrome-bg").trim(),
    );
    expect(chromeBg).toBe("#1e1e26");
    const accent = await page.evaluate(() =>
      getComputedStyle(document.documentElement).getPropertyValue("--ui-accent").trim(),
    );
    expect(accent).toMatch(/^#/);
    const createPanel = await page.evaluate(() =>
      getComputedStyle(document.documentElement).getPropertyValue("--create-panel-bg").trim(),
    );
    expect(createPanel).toBe("#2c2e3a");
  });

  test("navbar is not white chrome", async ({ page }) => {
    await page.goto("/");
    const nav = page.locator(".dc-navbar");
    await expect(nav).toBeVisible();
    const bg = await nav.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(bg).not.toBe("rgb(255, 255, 255)");
  });

  test("guest bar is visible when logged out", async ({ page }) => {
    await page.goto("/");
    const bar = page.getByRole("region", { name: "Account" });
    await expect(bar).toBeVisible();
    await expect(bar.getByRole("button", { name: "Log in" })).toBeVisible();
  });
});
