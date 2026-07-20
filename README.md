# Photon UF App

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Official Unified Field admin UI for Photon (Leptos).

```toml
[dependencies]
photon-app = { git = "https://github.com/deathbreakfast/photon-uf-app", package = "photon-app", branch = "main" }
```

Mount the Photon admin routes from your host shell (SSR + hydrate features as required by your Leptos setup).

## Workspace

| Crate | Role |
|-------|------|
| `photon-app` | Photon admin UI (topics, subscriptions, ops views) |
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
