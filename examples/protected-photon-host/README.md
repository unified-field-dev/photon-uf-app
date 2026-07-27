# protected-photon-host

Axum oneshot host under **`/photon`**: deny without session, allow with `X-Demo-User`, return the in-memory dashboard KPI shape `photon-backend` builds for the UI.

Production Leptos hosts mount `<PhotonRoutes />` (auth-gated). This example proves the same path + auth + dashboard contract without the full SSR/WASM graph.

| | |
|---|---|
| **When to use** | First smoke of Photon UF app host wiring (auth gate + dashboard API) |
| **Command** | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-photon-uf-app cargo run -p protected-photon-host` |
| **Success** | Stdout: `protected_photon_host: OK — /photon deny/allow + dashboard KPIs` |
| **Look next** | Mount [`PhotonRoutes`](../../photon-app/) in a product host; wire Photon runtime |

**Open first:** [`src/main.rs`](src/main.rs)

Compile-check:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
cargo check -p protected-photon-host
```
