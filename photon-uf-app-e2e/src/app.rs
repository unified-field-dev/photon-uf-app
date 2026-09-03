//! Mount Photon ops pages for Playwright.

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use uf_integrations::{
    provide_shell_auth_menu, HostAuthMenu, ShellAppBar, ShellAuthMenu, UnifiedFieldAppBar,
    UnifiedFieldShellLayout,
};
use uf_product::components::ContentContainer;
use uf_product::primitives::{Body1, Flex, FlexAlign, FlexGap, Link, Title3};
use uf_product::{orbital_shell, OrbitalTemplate};

use crate::gate_demos::E2eAuthProvider;
use crate::harness_auth_menu::HarnessAuthMenu;
use crate::photon_routes_eager::PhotonRoutesEager;

/// SSR document shell.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    orbital_shell(options, || view! { <App/> })
}

/// Root: harness auth + eager Photon routes.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    photon_app::ensure_help_steps_linked();
    uf_help::ensure_linked();
    #[cfg(feature = "ssr")]
    {
        provide_context(crate::e2e_higgs_config());
        provide_context(crate::e2e_photon());
        wire_gauge_permissions_bridge();
    }
    provide_shell_auth_menu(|| view! { <HarnessAuthMenu /> });

    view! {
        <OrbitalTemplate>
            <Stylesheet id="leptos" href="/pkg/photon-uf-app-e2e.css"/>
            <Title text="photon-uf-app e2e"/>
            <E2eAuthProvider>
                <Router>
                    <Routes fallback=|| view! { <p>"Not found"</p> }>
                        <Route path=path!("/") view=HomePage/>
                        <PhotonRoutesEager />
                    </Routes>
                </Router>
            </E2eAuthProvider>
        </OrbitalTemplate>
    }
}

/// Wire Gauge as uf-product permission backend (same pattern as chronon-uf-app-e2e).
#[cfg(feature = "ssr")]
pub fn wire_gauge_permissions_bridge() {
    use std::sync::Arc;

    struct GaugePermissionBackend;

    #[async_trait::async_trait]
    impl uf_product::permissions::PermissionBackend for GaugePermissionBackend {
        async fn has_permission(&self, permission_name: &str) -> Result<bool, ServerFnError> {
            let ctx = higgs::Higgs::from_request().await?;
            let valence = ctx
                .valence()
                .map_err(|e| ServerFnError::new(format!("Failed to build Valence: {e}")))?;
            let _caller =
                gauge::instrumentation::PermissionCheckCallerGuard::new("permission_backend");
            gauge::service::actor_can(&valence, permission_name)
                .await
                .map_err(|e| ServerFnError::new(format!("Failed to check permission: {e}")))
        }
    }

    uf_product::permissions::provide_permission_backend(Arc::new(GaugePermissionBackend));
}

#[cfg(not(feature = "ssr"))]
pub fn wire_gauge_permissions_bridge() {}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <UnifiedFieldShellLayout>
            <ShellAppBar slot>
                <UnifiedFieldAppBar app_name="Photon e2e".to_string()>
                    <ShellAuthMenu slot:auth_menu>
                        <HostAuthMenu />
                    </ShellAuthMenu>
                </UnifiedFieldAppBar>
            </ShellAppBar>
            <ContentContainer max_width="900px" data_testid="photon-e2e-home">
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Start>
                    <Title3>"photon-uf-app e2e"</Title3>
                    <Body1>"PhotonRoutes host for Playwright."</Body1>
                    <Link href="/photon">"Open /photon"</Link>
                </Flex>
            </ContentContainer>
        </UnifiedFieldShellLayout>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
    uf_product::hide_boot_loader();
}
