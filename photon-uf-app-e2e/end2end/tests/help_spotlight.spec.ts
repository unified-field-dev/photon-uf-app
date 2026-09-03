import { test, expect, seedAuth, waitForHydrated } from "./fixtures";
import type { Page } from "@playwright/test";

async function completeVisibleTour(page: Page) {
  const footer = page.locator('[data-testid="spotlight-footer"]:visible');
  const next = footer.getByTestId("spotlight-tour-next");
  await expect(footer).toBeVisible({ timeout: 60_000 });
  for (let i = 0; i < 24; i++) {
    if ((await footer.count()) === 0) {
      break;
    }
    // Spotlight panels can sit partially off-screen; DOM click avoids Playwright
    // viewport hit-testing failures that still occur with { force: true }.
    await next.evaluate((el: HTMLElement) => el.click());
    try {
      await expect(footer).toHaveCount(0, { timeout: 2_000 });
      break;
    } catch {
      /* more steps */
    }
  }
  await expect(footer).toHaveCount(0, { timeout: 30_000 });
}

test.describe("help-spotlight", () => {
  test("help-spotlight-skips-when-seeded", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/photon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("photon-dashboard")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("help-step-photon-intro")).toHaveCount(0);
    await expect(page.locator('[data-testid="spotlight-footer"]:visible')).toHaveCount(0);
  });

  test("help-spotlight-skips-auth-gate", async ({ page }) => {
    await seedAuth(page, "anonymous", { help_tour: true });
    await page.goto("/photon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("help-step-photon-intro")).toHaveCount(0);
    await expect(page.locator('[data-testid="spotlight-footer"]:visible')).toHaveCount(0);
  });

  test("help-spotlight-dashboard-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/photon", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-photon-intro")).toBeVisible({ timeout: 60_000 });
    await completeVisibleTour(page);
    await expect(page.getByTestId("help-step-photon-intro")).toHaveCount(0);

    await page.reload({ waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-photon-intro")).toHaveCount(0);
  });

  test("help-spotlight-topics-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/photon/topics", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-photon-topics-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-topic-detail-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const topic = seeded.fixtures.topic_name;
    await page.goto(`/photon/topics/${encodeURIComponent(topic)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-photon-topic-meta")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-subs-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/photon/subscriptions", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-photon-subs-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-sub-detail-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const id = seeded.fixtures.subscription_id;
    await page.goto(`/photon/subscriptions/${encodeURIComponent(id)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-photon-sub-meta")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-events-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/photon/events", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-photon-events-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-event-detail-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const id = seeded.fixtures.event_id;
    await page.goto(`/photon/events/${encodeURIComponent(id)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-photon-event-meta")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });
});
