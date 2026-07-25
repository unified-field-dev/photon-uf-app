//! Standalone app registration collected via [`inventory`] (replaces `orbital::inventory`).
//!
//! ## Features
//!
//! - **Zero-boilerplate discovery** — apps register themselves at link time via
//!   [`inventory::collect!`] instead of a central hand-maintained list.
//! - **Shell-agnostic metadata** — [`AppRegistration`] carries just enough static metadata
//!   (id, name, icon, route, optional brand seed / permission manifest) for a host shell to
//!   render a launcher and mount routes without depending on the app crate's UI types.
//!
//! ## Getting started
//!
//! Apps don't construct [`AppRegistration`] directly; the
//! `#[uf_product_macros::uf_app]` macro submits one on SSR builds. Hosts that want to
//! enumerate registered apps iterate the inventory:
//!
//! ```rust
//! use uf_app_registry::AppRegistration;
//!
//! for app in inventory::iter::<AppRegistration> {
//!     println!("{} -> {}", app.name, app.route_path);
//! }
//! ```
//!
//! ## Where to look next
//!
//! - [`AppRegistration`] — the metadata record itself.

#![deny(missing_docs)]

/// Metadata submitted by `#[uf_product_macros::uf_app]` on SSR builds.
///
/// One instance is registered per app crate via [`inventory::collect!`]; host shells iterate
/// `inventory::iter::<AppRegistration>()` to discover installed apps without a compile-time
/// dependency on each app's UI crate.
#[derive(Debug, Clone)]
pub struct AppRegistration {
    /// Stable, unique identifier for the app (e.g. `"photon"`).
    pub id: &'static str,
    /// Human-readable display name (e.g. `"Photon"`).
    pub name: &'static str,
    /// Short description shown in app launchers.
    pub description: &'static str,
    /// Icon glyph or emoji shown next to the app name.
    pub icon: &'static str,
    /// Root route path the app's nested routes are mounted under (e.g. `"/photon"`).
    pub route_path: &'static str,
    /// Optional brand seed used to derive app-specific accent theming.
    pub brand_seed: Option<&'static str>,
    /// Optional serialized permission manifest describing the app's guarded operations.
    pub permission_manifest: Option<&'static str>,
}

inventory::collect!(AppRegistration);
