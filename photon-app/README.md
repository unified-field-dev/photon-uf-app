# photon-app

Leptos operations UI for Photon: topics, subscriptions, and event streams under `/photon`.

```toml
# Pin tag or rev — do not use branch = "main".
photon-app = { git = "https://github.com/unified-field-dev/photon-uf-app", package = "photon-app", rev = "REPLACE_WITH_PIN", default-features = false }
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

Crate-root rustdoc owns task-oriented sections, the route table, and examples.
Mapping helpers and ops IO live in `photon-backend`.

## Module map

| Area | Path | Role |
|------|------|------|
| Routes + registration | `lib.rs` | `PhotonRoutes`, `uf_app!` inventory |
| Server functions | `server.rs` | Higgs `#[server]` wrappers over `photon_backend::ops` |
| Pages | `pages/` | Dashboard, topics, subscriptions, events |
| Components | `components/` | Orbital tables, cards, toolbars |
| Permissions | `permissions.rs` | `PhotonAdmin` manifest |

## Develop and verify

Compose into a host that supplies a Photon runtime and the auth/context
extractors the app expects. Enable `ssr` / hydrate to match your host.

Local and CI gates (fmt, clippy, backend contracts, Playwright e2e):
[`../docs/VERIFICATION.md`](../docs/VERIFICATION.md).

```bash
export CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-photon-uf-app
cargo clippy -p photon-app --features ssr --all-targets -- -D warnings
cargo leptos end-to-end --project photon-uf-app-e2e
```
