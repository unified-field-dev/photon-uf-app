use leptos::prelude::*;
use leptos_router::components::Outlet;
use lepton_shell::AppBarUserMenu;
use orbital::components::{
    Navigation, NavigationBody, NavigationConfig, NavigationLink, NavigationMaterial,
};
use uf_integrations::{
    ShellAppBar, ShellAuthMenu, ShellLeftNav, UnifiedFieldAppBar, UnifiedFieldShellLayout,
};

use crate::paths;
use crate::AppMetadata;

/// Photon's shell layout: app bar, left navigation, and a router [`Outlet`] for the
/// currently active page.
///
/// Wraps every route declared in [`crate::PhotonRoutes`] and is only rendered once auth has
/// been checked by the caller (see `PhotonAuthGuard` in the crate root).
#[component]
pub fn PhotonLayout() -> impl IntoView {
    let app_name = AppMetadata::name().to_string();
    let selected_value = RwSignal::new(None::<String>);
    let open_categories = RwSignal::new(Vec::<String>::new());

    view! {
        <div data-testid="photon-app-root">
        <UnifiedFieldShellLayout>
            <ShellAppBar slot>
                <UnifiedFieldAppBar
                    app_name=app_name
                    app_id=AppMetadata::id()
                    homepage_url="/".to_string()
                >
                    <ShellAuthMenu slot:auth_menu>
                        <AppBarUserMenu />
                    </ShellAuthMenu>
                </UnifiedFieldAppBar>
            </ShellAppBar>
            <ShellLeftNav slot>
                <Navigation config=NavigationConfig::new().with_selected_value(selected_value).with_open_categories(open_categories)>
                    <NavigationMaterial slot />
                    <NavigationBody slot>
                        <NavigationLink path=paths::ROOT value=paths::ROOT icon=icondata::AiHomeOutlined exact=true test_id="nav-photon-dashboard">"Dashboard"</NavigationLink>
                        <NavigationLink path=paths::TOPICS value=paths::TOPICS icon=icondata::AiBellOutlined test_id="nav-photon-topics">"Topics"</NavigationLink>
                        <NavigationLink path=paths::SUBSCRIPTIONS value=paths::SUBSCRIPTIONS icon=icondata::AiUnorderedListOutlined test_id="nav-photon-subscriptions">"Subscriptions"</NavigationLink>
                        <NavigationLink path=paths::EVENTS value=paths::EVENTS icon=icondata::AiHistoryOutlined test_id="nav-photon-events">"Events"</NavigationLink>
                    </NavigationBody>
                </Navigation>
            </ShellLeftNav>
            <Outlet />
        </UnifiedFieldShellLayout>
        </div>
    }
}
