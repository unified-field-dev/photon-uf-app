//! Proc macros for host-product Orbital apps (registration, server context, search sources).
//!
//! Design-system macros (`#[component_doc]`, route extraction) live in `orbital-macros`.
//!
//! ## Features
//!
//! - [`orbital_app!`] — registers an app's metadata (name, id, icon, route) with
//!   [`uf_app_registry`] and optionally generates its top-level route component.
//! - [`#[server]`](macro@server) — wraps Leptos's `#[server]` with operation context and an
//!   optional permission gate.
//! - [`#[derive(OrbitalPermissionManifest)]`](derive@OrbitalPermissionManifest) — derives a
//!   crate-local Orbital permission manifest from attributes on a type.
//! - [`define_search_sources!`] — declares search sources and their SSR descriptor
//!   registrations in one call.
//!
//! ## Getting started
//!
//! Most apps only need [`orbital_app!`] in their crate root:
//!
//! ```rust,ignore
//! use uf_product_macros::orbital_app;
//!
//! orbital_app! {
//!     name: "Photon",
//!     id: "photon",
//!     description: "Event pipeline management",
//!     icon: "💫",
//!     version: "0.1.0",
//!     routes: PhotonRoutes,
//!     route_path: "/photon",
//! }
//! ```
//!
//! ## Where to look next
//!
//! - [`orbital_app!`] — app metadata + route registration.
//! - [`macro@server`] — the `#[server]` attribute macro.
//! - [`derive@OrbitalPermissionManifest`] — the permission manifest derive.
//! - [`define_search_sources!`] — search source declarations.

#![deny(missing_docs)]

use proc_macro::TokenStream;

mod app_definition;
mod permission_manifest_derive;
mod search_sources;
mod server;

/// Register a product app for Orbital shell discovery.
#[proc_macro]
pub fn orbital_app(input: TokenStream) -> TokenStream {
    app_definition::expand_orbital_app(input)
}

/// Wrapper around Leptos `#[server]` with operation context and optional permission gate.
#[proc_macro_attribute]
pub fn server(attr: TokenStream, input: TokenStream) -> TokenStream {
    server::expand_server(attr, input)
}

/// Derive helper for crate-local Orbital permission manifests.
#[proc_macro_derive(OrbitalPermissionManifest, attributes(permission_manifest, permission))]
pub fn derive_orbital_permission_manifest(input: TokenStream) -> TokenStream {
    permission_manifest_derive::expand_derive_permission_manifest(input)
}

/// Define search sources and SSR descriptor registrations.
#[proc_macro]
pub fn define_search_sources(input: TokenStream) -> TokenStream {
    search_sources::expand_define_search_sources(input)
}
