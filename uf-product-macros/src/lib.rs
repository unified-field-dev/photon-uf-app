//! Proc macros for Unified Field product apps (registration, server context, search sources).
//!
//! Design-system macros (`#[component_doc]`, route extraction) live in `orbital-macros`.
//!
//! ## Features
//!
//! - [`uf_app!`] — register a product app (id, name, icon, routes) so the
//!   shell can discover it, and so `uf-codegen`'s build-script scan can find its route component.
//! - `#[server]` ([`macro@server`]) — wraps Leptos's `#[server]` with operation-context plumbing
//!   and an optional permission gate.
//! - `#[derive(UfPermissionManifest)]` ([`derive_uf_permission_manifest`]) — derive
//!   a permission manifest for a crate-local enum/struct.
//! - [`define_search_sources!`] — register one or more backend search sources for the
//!   `/search` command palette.
//!
//! ## Getting started
//!
//! In a product-app crate's `lib.rs`:
//!
//! ```rust,ignore
//! use uf_product_macros::uf_app;
//!
//! uf_app! {
//!     name: "Counter",
//!     id: "counter",
//!     description: "A simple counter application",
//!     icon: "📊",
//!     version: "0.1.0",
//!     routes: CounterRoutes,
//!     route_path: "/counter",
//! }
//! ```
//!
//! ## Where to look next
//!
//! - [`uf_app`] — expands to app metadata + `inventory::submit!` registration.
//! - [`macro@server`] — SSR-side operation context wrapper around `#[leptos::server]`.

use proc_macro::TokenStream;

mod app_definition;
mod permission_manifest_derive;
mod search_sources;
mod server;

/// Register a product app for Unified Field shell discovery.
#[proc_macro]
pub fn uf_app(input: TokenStream) -> TokenStream {
    app_definition::expand_uf_app(input)
}

/// Wrapper around Leptos `#[server]` with operation context and optional permission gate.
#[proc_macro_attribute]
pub fn server(attr: TokenStream, input: TokenStream) -> TokenStream {
    server::expand_server(attr, input)
}

/// Derive helper for crate-local Unified Field permission manifests.
#[proc_macro_derive(UfPermissionManifest, attributes(permission_manifest, permission))]
pub fn derive_uf_permission_manifest(input: TokenStream) -> TokenStream {
    permission_manifest_derive::expand_derive_permission_manifest(input)
}

/// Define search sources and SSR descriptor registrations.
#[proc_macro]
pub fn define_search_sources(input: TokenStream) -> TokenStream {
    search_sources::expand_define_search_sources(input)
}
