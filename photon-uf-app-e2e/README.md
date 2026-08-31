# photon-uf-app-e2e

Leptos lab host + Playwright for [`photon-app`](../photon-app/) `PhotonRoutes`.

Mounts the same pages a product host would under `/photon`, with lab-only mem
Valence, session injection, and an in-process mem Photon runtime.
**Do not copy this boot into a production host.**

## Run

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
# From the photon-uf-app workspace root.
cargo leptos end-to-end --project photon-uf-app-e2e
```

Do not interrupt the end-to-end run. It stops on its own when Playwright finishes.

Site: `http://127.0.0.1:3190` · seed: `POST /api/test/seed-data`

Boundary integration (no browser):

```bash
cargo test -p photon-uf-app-e2e --features ssr --test boundary_contract
```

## Scenarios

| ID | Asserts |
|----|---------|
| `e2e.auth.anonymous_gate` | Anon gated; no dashboard |
| `e2e.auth.no_admin` | Outsider authenticated; PhotonAdmin reads denied |
| `e2e.auth.admin_dashboard` / `e2e.dashboard.load_happy` | Admin sees KPIs and seeded topic |
| `e2e.topics.*` | Index→detail; unknown topic |
| `e2e.subs.*` | Index→detail; unknown subscription |
| `e2e.events.*` | Index→detail; unknown event |
