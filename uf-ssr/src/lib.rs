//! Higgs-backed SSR helpers replacing the template `orbital::ssr` module for standalone
//! `uf-app` repos.
//!
//! ## Features
//!
//! - **Request-scoped [`Valence`]** — [`ssr::valence`] resolves the current request's actor
//!   (via [`higgs::Higgs`]) into a permission-checked [`Valence`] handle, so server functions
//!   never have to parse claims themselves.
//! - **System-scoped [`Valence`]** — [`ssr::system_valence`] does the same for background work
//!   that runs outside a single request (startup jobs, sweeps, health checks).
//! - **Operation context passthrough** — [`ssr::current_operation`] / [`ssr::with_operation`]
//!   are re-exported from [`uf_host`] so callers only need to depend on this crate.
//!
//! *This crate has no auth model of its own* — it only adapts [`higgs::Higgs`] request context
//! into the [`Valence`] type your server functions already expect.
//!
//! ## Getting started
//!
//! Call [`ssr::valence`] from any `#[server]` function to get a session scoped to the caller:
//!
//! ```rust,ignore
//! use leptos::prelude::ServerFnError;
//!
//! #[leptos::server]
//! pub async fn my_endpoint() -> Result<String, ServerFnError> {
//!     let db = uf_ssr::ssr::valence().await?;
//!     // ... use `db` for permission-checked reads/writes ...
//!     Ok("ok".into())
//! }
//! ```
//!
//! This crate is only compiled with an `ssr` module when the `ssr` feature is enabled; on the
//! client (`hydrate`/`csr` builds) it exposes nothing.
//!
//! ## Where to look next
//!
//! - [`ssr`] — the SSR-only module with [`ssr::valence`] and [`ssr::system_valence`].

#![deny(missing_docs)]

/// SSR-only helpers for resolving a request or background operation into a [`Valence`] session.
///
/// Only compiled when the `ssr` feature is enabled; client (WASM) builds see no items from this
/// module at all.
#[cfg(feature = "ssr")]
pub mod ssr {
    use higgs::Higgs;
    use leptos::prelude::ServerFnError;
    use valence::Valence;

    pub use uf_host::{current_operation, with_operation};

    /// Build a [`Valence`] for the current request's actor via [`Higgs`].
    ///
    /// Resolves the calling actor from the active request context and returns a
    /// permission-checked session scoped to that actor. Returns [`ServerFnError`] if there is
    /// no active request context or the actor cannot be resolved.
    pub async fn valence() -> Result<Valence, ServerFnError> {
        let ctx = Higgs::from_request().await?;
        ctx.valence()
            .map_err(|e| ServerFnError::new(e.to_string()))
    }

    /// Build a system-scoped [`Valence`] for the current operation context.
    ///
    /// Use this for background work (retention sweeps, startup jobs, scheduled tasks, …) that
    /// runs outside any single subscribed request but still needs a permission-checked session.
    pub async fn system_valence() -> Result<Valence, ServerFnError> {
        let ctx = Higgs::from_request().await?;
        ctx.system_valence()
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
}
