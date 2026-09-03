import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("e2e.events", () => {
  test("e2e.events.index_happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto("/photon/events", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    const events = page.getByTestId("photon-events");
    await expect(events).toBeVisible({ timeout: 60_000 });
    // Topic also appears as a hidden <option>; assert the table cell instead.
    await expect(
      events.getByRole("cell", { name: seeded.fixtures.topic_name }),
    ).toBeVisible({ timeout: 60_000 });
  });

  test("e2e.events.detail_happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(
      `/photon/events/${encodeURIComponent(seeded.fixtures.event_id)}`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-event-detail")).toBeVisible({ timeout: 60_000 });
    await expect(
      page.getByTestId("photon-event-detail").getByText(seeded.fixtures.topic_name).first(),
    ).toBeVisible();
  });

  test("e2e.events.not_found_sad", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/photon/events/__photon_e2e_no_such_event__", {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-event-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText("Event not found")).toBeVisible({ timeout: 60_000 });
  });
});
