import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("e2e.topics", () => {
  test("e2e.topics.index_happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto("/photon/topics", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-topics")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(seeded.fixtures.topic_name).first()).toBeVisible({
      timeout: 60_000,
    });
  });

  test("e2e.topics.detail_happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(
      `/photon/topics/${encodeURIComponent(seeded.fixtures.topic_name)}`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-topic-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(seeded.fixtures.topic_name).first()).toBeVisible();
  });

  test("e2e.topics.not_found_sad", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/photon/topics/__photon_e2e_no_such_topic__", {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-topic-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText("Topic not found")).toBeVisible({ timeout: 60_000 });
  });
});
