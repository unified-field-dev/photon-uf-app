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

Host must supply a Photon runtime and auth guard context. Enable `ssr` / hydrate features to match your host.

## Workspace

| Crate | Role |
|-------|------|
| `photon-app` | Photon admin UI |
| `uf-*` | Thin shell / registry helpers shared with other uf-app repos |

## Verify

```bash
export CARGO_BUILD_JOBS=1
cargo check --workspace
cargo check -p photon-app --features ssr
cargo test -p photon-app --features ssr
```

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
