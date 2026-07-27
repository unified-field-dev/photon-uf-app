# Photon UF App

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Leptos admin UI for Photon topics, subscriptions, and events — mounted under `/photon`.

```toml
[dependencies]
photon-app = { git = "https://github.com/deathbreakfast/photon-uf-app", package = "photon-app", branch = "main" }
```

```rust
use photon_app::PhotonRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <PhotonRoutes />
    </Routes>
}
```

## About

- Dashboard for topic/subscription/event activity
- Topic and subscription detail (schemas, checkpoints)
- Event browse with payload and actor context

Host must supply a Photon runtime and auth guard context. Enable `ssr` / hydrate features to match your host. See the `photon-app` crate rustdocs for the full Concern → route → server fn table.

## Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-photon-host`](examples/protected-photon-host/) | Auth + `/photon` dashboard API | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-photon-uf-app cargo run -p protected-photon-host` | Deny/allow + KPI JSON | Product host with `PhotonRoutes` |

Full ladder: [`examples/README.md`](examples/README.md).

## Workspace

| Crate | Role |
|-------|------|
| `photon-app` | Photon admin UI |
| `photon-backend` | Pure topic/subscription/event contracts (no Leptos) |
| `uf-*` (top-level `uf-app-registry`, `uf-integrations`, `uf-product-macros`, `uf-ssr`) | Not workspace members and not depended on — the workspace's real `uf-*` crates come from `L3-products-zones-hosts` (see `[workspace.dependencies]` in `Cargo.toml`). These local trees are unused leftovers; do not treat them as source of truth. |

## Verify

See [`docs/VERIFICATION.md`](docs/VERIFICATION.md). Preferred backend CI:

```bash
export CARGO_BUILD_JOBS=1
cargo clippy -p photon-backend --all-targets -- -D warnings
cargo test -p photon-backend
```

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
