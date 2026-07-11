//! Proc macros for host-product Orbital apps (registration, server context, search sources).
//!
//! Design-system macros (`#[component_doc]`, route extraction) live in `orbital-macros`.

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
