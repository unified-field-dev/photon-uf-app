# Photon UF App

[![CI](https://github.com/unified-field-dev/photon-uf-app/actions/workflows/ci.yml/badge.svg)](https://github.com/unified-field-dev/photon-uf-app/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[GitHub](https://github.com/unified-field-dev/photon-uf-app) · `cargo doc -p photon-backend --open`

## About

Photon UF App is the Unified Field **operations UI** for Photon topics,
subscriptions, and events under `/photon`. Photon itself has no built-in UI;
hosts mount this crate so operators can inspect runtime activity.

- **UI (`photon-app`)** — pages, Higgs `#[server]` wrappers, `PhotonRoutes`,
  `uf_app!` registration
- **Backend (`photon-backend`)** — pure topic/subscription/event/dashboard
  helpers (no Leptos); primary CI surface

Reads Photon's runtime directly (`admin_snapshot`, list/get event APIs). Hosts
supply a Photon runtime and auth guard context. Enable `ssr` / hydrate to match
your host. Crate-root rustdoc owns Concern → route → server fn tables; prefer
`cargo doc -p photon-backend --open` for the mapping contract. UI rustdoc is
pin-dependent on Orbital / host graphs.

## Getting started

```toml
[dependencies]
# Pin tag or rev — do not use branch = "main".
photon-app = { git = "https://github.com/unified-field-dev/photon-uf-app", package = "photon-app", rev = "REPLACE_WITH_PIN", default-features = false }
photon-backend = { git = "https://github.com/unified-field-dev/photon-uf-app", package = "photon-backend", rev = "REPLACE_WITH_PIN" }
```

```rust,ignore
use photon_app::PhotonRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <PhotonRoutes />
    </Routes>
}
```

Wire Photon runtime + session extractors in host bootstrap, then mount the
routes above. Full Leptos SSR hosts live outside this repository; use the local
teaching host for the auth + dashboard contract.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
cargo test -p photon-backend
```

## Workspace

| Crate | Role |
|-------|------|
| [`photon-app`](photon-app/) | Leptos ops UI + `PhotonRoutes` + app registration |
| [`photon-backend`](photon-backend/) | Pure DTO/mapping helpers for topic/sub/event/dashboard |
| [`protected-photon-host`](examples/protected-photon-host/) | Teaching host: deny/allow + dashboard KPIs |

## Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-photon-host`](examples/protected-photon-host/) | Auth + `/photon` dashboard API | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-photon-uf-app cargo run -p protected-photon-host` | Deny/allow + KPI JSON | Mount `PhotonRoutes` |

Copy table + product mount `Cargo.toml`:
[`examples/protected-photon-host/README.md`](examples/protected-photon-host/README.md).
More examples: [`examples/README.md`](examples/README.md).

## Security

Auth-gated `/photon` routes and private vulnerability reporting:
[`SECURITY.md`](SECURITY.md). Report vulnerabilities privately — do not open a
public issue for security-sensitive reports.

## Verify

GitHub Actions (`.github/workflows/ci.yml`) runs the CI subset from
[`docs/VERIFICATION.md`](docs/VERIFICATION.md): fmt, clippy `-D warnings` on
`photon-backend` (+ teaching host), contract tests, `protected-photon-host`
check/run, and photon-backend rustdoc with broken-intra-doc-link deny.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
cargo fmt -p photon-backend -p photon-app -p protected-photon-host -- --check
cargo clippy -p photon-backend --all-targets -- -D warnings
cargo clippy -p protected-photon-host --all-targets -- -D warnings
cargo test -p photon-backend --test workspace_members --test product_surface
cargo test -p photon-backend
cargo check -p protected-photon-host
cargo run -p protected-photon-host
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p photon-backend --no-deps
```

Teaching host success line:
`protected_photon_host: OK — /photon deny/allow + dashboard KPIs`.
Full command block: [`docs/VERIFICATION.md`](docs/VERIFICATION.md). Contribute:
[`CONTRIBUTING.md`](CONTRIBUTING.md).

## FAQ

**Is this a standalone Photon server?** No. `photon-app` mounts under a host
`<Routes>` tree. Photon transport and persistence live in the Photon core crates.

**Why is there a separate `photon-backend` crate?** So topic/subscription/event
and dashboard helpers stay unit-testable without the Leptos/UI dependency graph.
`photon-app` `#[server]` fns are thin wrappers over those helpers.

**Do routes mutate Photon state?** No. The UI is read-only today (list/detail +
dashboard). Create/edit flows are out of scope for this surface.

**Where does Photon core fit?** Event pipeline, brokers, and IsolatedLab
contracts live in [photon](https://github.com/unified-field-dev/photon). This
repo maps runtime admin/list/get APIs into UF ops pages.

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md),
[SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
