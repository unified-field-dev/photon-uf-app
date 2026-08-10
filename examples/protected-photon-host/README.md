# protected-photon-host

Axum oneshot host under **`/photon`**: deny without session, allow with
`X-Demo-User`, return the in-memory dashboard KPI shape `photon-backend` builds
for the UI.

Production Leptos hosts mount `PhotonRoutes` at **`/photon`** and gate ops
reads with `PhotonAdmin`. This example proves the same path + auth + dashboard
contract without the SSR/WASM / Orbital graph. The oneshot path `/photon`
matches the Orbital app id/path (`photon` / `/photon`).

| | |
|---|---|
| **When to use** | First smoke of Photon UF app host wiring (auth gate + dashboard API) |
| **Command** | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-photon-uf-app cargo run -p protected-photon-host` |
| **Success** | Stdout: `protected_photon_host: OK — /photon deny/allow + dashboard KPIs` |
| **Look next** | Mount [`PhotonRoutes`](../../photon-app/) ; wire Photon runtime + valence-admin |

**Open first:** [`src/main.rs`](src/main.rs)

## Copy into your host

| File | What to take |
|------|----------------|
| This [`Cargo.toml`](Cargo.toml) | Axum oneshot shape + `photon-backend` (dashboard KPI smoke) |
| Product mount `Cargo.toml` (below) | `photon-app` + `photon-backend` with `ssr` / `hydrate` features |
| [`src/main.rs`](src/main.rs) | Session gate on `/photon`, dashboard JSON, inventory contract names |
| Leptos sketch (below) | `<PhotonRoutes />` under `/photon` |

### Product mount dependencies

```toml
[dependencies]
photon-app = { git = "https://github.com/deathbreakfast/photon-uf-app", package = "photon-app", rev = "REPLACE_WITH_PIN", default-features = false }
photon-backend = { git = "https://github.com/deathbreakfast/photon-uf-app", package = "photon-backend", rev = "REPLACE_WITH_PIN" }
uf-product = { /* your pin */, default-features = false }
uf-integrations = { /* your pin */, default-features = false }

[features]
ssr = [
    "photon-app/ssr",
    "uf-product/ssr",
    "uf-integrations/ssr",
]
hydrate = [
    "photon-app/hydrate",
    "uf-product/hydrate",
    "uf-integrations/hydrate",
]
```

### Leptos mount sketch

```rust,ignore
use photon_app::PhotonRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <PhotonRoutes />
    </Routes>
}
```

Dashboard helpers (Leptos-free):

```rust,ignore
use photon_backend::dashboard_stats;

let stats = dashboard_stats(topic_count, subscription_count, event_count_24h);
```

Inventory names match `photon` / `/photon`. Layout uses `RequireAuthenticated`;
ops `#[server]` fns carry `PhotonAdmin` (manifest
`permissions::PhotonPermission`). Wire a Photon runtime + session extractors in
host bootstrap before mounting the routes.

For shell chrome (layout, fonts, Axum + Leptos boot), copy
[`shell-chrome-host`](https://github.com/deathbreakfast/unified-field-product/tree/main/examples/shell-chrome-host)
from unified-field-product, then mount `PhotonRoutes`.

## Run (documented gate)

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
cargo check -p protected-photon-host
cargo run -p protected-photon-host
```

**Success:** stdout prints `protected_photon_host: OK — /photon deny/allow + dashboard KPIs`.

## Hydrate / browser

Out of gate for this host. Full ops UI needs a product binary with
`cargo-leptos`, `wasm32`, session chrome, Photon runtime, and a working Orbital /
`uf-product` graph. Prefer the oneshot above for local gates; treat `photon-app`
compile failures from broken sibling pins as host-product debt.
