//! Higgs-backed SSR helpers replacing template `orbital::ssr` for standalone uf-app repos.

#[cfg(feature = "ssr")]
pub mod ssr {
    use higgs::Higgs;
    use leptos::prelude::ServerFnError;
    use valence::Valence;

    pub use uf_host::{current_operation, with_operation};

    /// Build Valence for the current request actor via Higgs.
    pub async fn valence() -> Result<Valence, ServerFnError> {
        let ctx = Higgs::from_request().await?;
        ctx.valence()
            .map_err(|e| ServerFnError::new(e.to_string()))
    }

    /// Build a system Valence for the current operation context.
    pub async fn system_valence() -> Result<Valence, ServerFnError> {
        let ctx = Higgs::from_request().await?;
        ctx.system_valence()
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
}
