import { test as base, expect, type Page } from "@playwright/test";

export type SeedAuthKind = "anonymous" | "admin" | "outsider" | "unverified";

export type SeedFixtures = {
  topic_name: string;
  subscription_id: string;
  event_id: string;
};

const HELP_STORAGE_KEY = "uf.help.tour_steps";

export async function seedAuth(
  page: Page,
  auth: SeedAuthKind,
  opts?: { help_tour?: boolean },
) {
  const helpTour = opts?.help_tour ?? false;

  const res = await page.request.post("/api/test/seed-data", {
    data: {
      auth,
      help_tour: helpTour,
    },
  });
  expect(res.ok()).toBeTruthy();
  const body = (await res.json()) as {
    ok: boolean;
    auth: string;
    fixtures: SeedFixtures;
    help_seen_json?: string | null;
  };

  const seenJson = body.help_seen_json ?? "[]";

  // Seed / clear before any document loads so WASM `read_local_visits` sees the key.
  await page.addInitScript(
    ({ enableTour, seen, key }) => {
      try {
        if (enableTour) {
          if (!sessionStorage.getItem("uf.help.e2e_tour_cleared")) {
            localStorage.removeItem(key);
            sessionStorage.setItem("uf.help.e2e_tour_cleared", "1");
          }
          return;
        }
        localStorage.setItem(key, seen);
      } catch {
        /* ignore */
      }
    },
    { enableTour: helpTour, seen: seenJson, key: HELP_STORAGE_KEY },
  );

  // Also write on the origin now (covers the first goto after seed).
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await page.evaluate(
    ({ enableTour, seen, key }) => {
      if (enableTour) {
        if (!sessionStorage.getItem("uf.help.e2e_tour_cleared")) {
          localStorage.removeItem(key);
          sessionStorage.setItem("uf.help.e2e_tour_cleared", "1");
        }
        return;
      }
      localStorage.setItem(key, seen);
    },
    { enableTour: helpTour, seen: seenJson, key: HELP_STORAGE_KEY },
  );

  return body;
}

/**
 * Wait for Orbital boot overlay to finish and hydrate to mark the document ready.
 */
export async function waitForHydrated(page: Page, timeoutMs = 240_000) {
  await expect
    .poll(
      async () =>
        page.evaluate(() => {
          const html = document.documentElement;
          if (html.getAttribute("data-orbital-boot-state") === "error") {
            return "error";
          }
          if (html.getAttribute("data-orbital-hydrated") === "true") {
            return "ready";
          }
          return "loading";
        }),
      { timeout: timeoutMs },
    )
    .not.toBe("error");
  await expect
    .poll(
      async () =>
        page.evaluate(
          () => document.documentElement.getAttribute("data-orbital-hydrated") === "true",
        ),
      { timeout: timeoutMs },
    )
    .toBe(true);
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
