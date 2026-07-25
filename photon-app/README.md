# photon-app

Leptos operations UI for Photon: topics, subscriptions, and event streams under `/photon`.

```toml
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

## Routes

Mounted under `/photon` (auth-gated):

- Dashboard — aggregate topic/subscription/event activity
- Topics — index and detail
- Subscriptions — index and detail
- Events — index and detail with payload previews

## Integration

Compose into a host that supplies a Photon runtime and the auth/context extractors the app expects. See crate rustdocs for SSR / hydrate feature flags.
