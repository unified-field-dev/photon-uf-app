import { test as base, expect, type Page } from "@playwright/test";

export type SeedAuthKind = "anonymous" | "admin" | "outsider" | "unverified";

export type SeedFixtures = {
  topic_name: string;
  subscription_id: string;
  event_id: string;
};

export async function seedAuth(page: Page, auth: SeedAuthKind) {
  const res = await page.request.post("/api/test/seed-data", {
    data: { auth },
  });
  expect(res.ok()).toBeTruthy();
  return res.json() as Promise<{
    ok: boolean;
    auth: string;
    fixtures: SeedFixtures;
  }>;
}

async function bootState(page: Page): Promise<"ready" | "error" | "loading"> {
  return page.evaluate(() => {
    const html = document.documentElement;
    if (html.getAttribute("data-orbital-hydrated") === "true") {
      return "ready";
    }
    if (html.getAttribute("data-orbital-boot-state") === "error") {
      return "error";
    }
    return "loading";
  });
}

/**
 * Wait for Orbital hydrate. On terminal boot `error`, pause then reload — do not
 * thrash navigations (that aborts in-flight `.wasm`). Never reload while `loading`.
 */
export async function waitForHydrated(page: Page, timeoutMs = 180_000) {
  const deadline = Date.now() + timeoutMs;
  let refreshes = 0;
  const maxRefreshes = 3;

  while (Date.now() < deadline) {
    const state = await bootState(page);
    if (state === "ready") {
      break;
    }
    if (state === "error") {
      if (refreshes >= maxRefreshes) {
        break;
      }
      refreshes += 1;
      // Let Chromium release a failed compile before retrying the ~50–100MiB wasm.
      await page.waitForTimeout(1_500);
      await page.reload({ waitUntil: "load" });
      continue;
    }
    await page.waitForTimeout(500);
  }

  await expect
    .poll(async () => bootState(page), { timeout: 10_000 })
    .toBe("ready");
  await expect(page.getByTestId("orbital-boot-overlay")).toHaveCount(0, {
    timeout: 60_000,
  });
  await expect(page.getByTestId("e2e-auth-bootstrap")).toBeAttached({
    timeout: 30_000,
  });
}

/** Higgs / server-fn deny surfaces as an Orbital error MessageBar. */
export async function expectMutationDenied(page: Page) {
  await expect(page.locator(".orbital-message-bar--error").first()).toBeVisible({
    timeout: 60_000,
  });
}

export const test = base;
export { expect };
