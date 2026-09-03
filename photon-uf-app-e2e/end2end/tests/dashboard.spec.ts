import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("e2e.dashboard", () => {
  test("e2e.dashboard.load_happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto("/photon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    const dash = page.getByTestId("photon-dashboard");
    await expect(dash).toBeVisible({ timeout: 60_000 });
    // Scope to the dashboard: bare getByText("Topics") hits the collapsed nav link first.
    await expect(dash.getByText("Topics", { exact: true })).toBeVisible();
    await expect(dash.getByText("Subscriptions", { exact: true })).toBeVisible();
    await expect(dash.getByText("Events (24h)", { exact: true })).toBeVisible();
    await expect(
      dash.getByRole("cell", { name: seeded.fixtures.topic_name }).first(),
    ).toBeVisible({ timeout: 60_000 });
  });
});
