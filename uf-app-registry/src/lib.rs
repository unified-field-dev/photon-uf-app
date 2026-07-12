//! Standalone app registration collected via `inventory` (replaces `orbital::inventory`).

/// Metadata submitted by `#[uf_product_macros::orbital_app]` on SSR builds.
#[derive(Debug, Clone)]
pub struct AppRegistration {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub route_path: &'static str,
    pub brand_seed: Option<&'static str>,
    pub permission_manifest: Option<&'static str>,
}

inventory::collect!(AppRegistration);
