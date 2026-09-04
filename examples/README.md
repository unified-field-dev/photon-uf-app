# Examples

Runnable teaching hosts for this UF app. Each card: when to use · command ·
success · look next.

## Canonical path

### `protected-photon-host` — auth + `/photon` dashboard

**Teaches:** session auth gate on `/photon` and the in-memory dashboard KPI shape
`photon-backend` builds for the UI. Inventory names: `photon` / `/photon` /
`RequireAuthenticated` / `PhotonAdmin`.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
cargo run -p protected-photon-host
```

**Success:** stdout prints `protected_photon_host: OK — /photon deny/allow + dashboard KPIs`.

**Next step:** Mount `<PhotonRoutes />` in a product host with Photon runtime.

Copy table + product mount `Cargo.toml`:
[`protected-photon-host/README.md`](protected-photon-host/README.md).

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-photon-host`](protected-photon-host/) | Auth + `/photon` dashboard API | `cargo run -p protected-photon-host` | Deny/allow + KPI JSON | Product host with `PhotonRoutes` |
