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

Crate-root rustdoc owns Organized-by-task, Owns / does not own, the route table,
and the Examples ladder. Mapping helpers live in `photon-backend`.

Compose into a host that supplies a Photon runtime and the auth/context
extractors the app expects. Enable `ssr` / hydrate to match your host.
