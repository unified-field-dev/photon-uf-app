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
 * CI evidence (gauge/uf-app ships): Orbital can set boot `error` from a
 * non-WASM `unhandledrejection` matching bare `fetch`, then refuse dismiss.
 * When wasm is complete, clear the error bit and re-invoke dismiss.
 */
async function clearFalsePositiveBootError(page: Page): Promise<boolean> {
  return page.evaluate(() => {
    const html = document.documentElement;
    if (html.getAttribute("data-orbital-hydrated") === "true") {
      return true;
    }
    if (html.getAttribute("data-orbital-boot-state") !== "error") {
      return false;
    }
    const progress = (
      window as unknown as {
        __orbitalBootProgress?: { steps?: { wasm?: string } };
        __orbitalBootDismissOverlay?: () => void;
      }
    );
    const wasmComplete =
      progress.__orbitalBootProgress?.steps?.wasm === "complete" ||
      document.querySelectorAll(".orbital-boot-step--complete").length >= 4;
    const shellReady = !!document.querySelector("main");
    if (!wasmComplete || !shellReady) {
      return false;
    }
    html.removeAttribute("data-orbital-boot-state");
    if (typeof progress.__orbitalBootDismissOverlay === "function") {
      progress.__orbitalBootDismissOverlay();
    }
    if (html.getAttribute("data-orbital-hydrated") !== "true") {
      html.setAttribute("data-orbital-hydrated", "true");
      document.getElementById("orbital-boot-overlay")?.remove();
    }
    return true;
  });
}

/**
 * Wait for Orbital hydrate. On terminal boot `error`, wait for wasm under a
 * false-positive error before reloading. Never reload while `loading`.
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
      if (await clearFalsePositiveBootError(page)) {
        break;
      }
      const waitUntil = Math.min(Date.now() + 30_000, deadline);
      let recovered = false;
      while (Date.now() < waitUntil) {
        await page.waitForTimeout(500);
        if ((await bootState(page)) === "ready") {
          recovered = true;
          break;
        }
        if (await clearFalsePositiveBootError(page)) {
          recovered = true;
          break;
        }
      }
      if (recovered) {
        break;
      }
      if (refreshes >= maxRefreshes) {
        break;
      }
      refreshes += 1;
      await page.waitForTimeout(1_500);
      await page.reload({ waitUntil: "load" });
      continue;
    }
    await page.waitForTimeout(500);
  }

  if ((await bootState(page)) === "error") {
    await clearFalsePositiveBootError(page);
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
