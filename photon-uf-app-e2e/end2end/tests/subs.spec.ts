import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("e2e.subs", () => {
  test("e2e.subs.index_happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto("/photon/subscriptions", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-subscriptions")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(seeded.fixtures.topic_name).first()).toBeVisible({
      timeout: 60_000,
    });
  });

  test("e2e.subs.detail_happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(
      `/photon/subscriptions/${encodeURIComponent(seeded.fixtures.subscription_id)}`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-subscription-detail")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByText(seeded.fixtures.topic_name).first()).toBeVisible();
  });

  test("e2e.subs.not_found_sad", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/photon/subscriptions/__photon_e2e_no_such_sub__", {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-subscription-detail")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByText("Subscription not found")).toBeVisible({ timeout: 60_000 });
  });
});
