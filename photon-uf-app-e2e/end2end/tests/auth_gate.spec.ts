import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("e2e.auth", () => {
  test("e2e.auth.anonymous_gate", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto("/photon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("photon-dashboard")).toHaveCount(0);
  });

  test("e2e.auth.no_admin", async ({ page }) => {
    await seedAuth(page, "outsider");
    await page.goto("/photon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-app-root")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("photon-dashboard")).toBeVisible({ timeout: 60_000 });
    // Authenticated without PhotonAdmin: server fns fail; KPI grid never loads.
    await expect(page.getByText(/Failed to load stats|Permission denied|permission/i).first()).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.locator("#photon-dashboard-stats")).toHaveCount(0);
  });

  test("e2e.auth.admin_dashboard", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/photon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-app-root")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("photon-dashboard")).toBeVisible({ timeout: 60_000 });
  });
});
