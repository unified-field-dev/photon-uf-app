//! Lightweight Unified Field shell integrations for exported app crates.

use leptos::prelude::*;
use orbital::components::{
    AppBar, AppBarDensity, AppBarLeading, AppBarMaterial, AppBarPosition, Layout, LayoutHeader,
    LayoutMain, LayoutSidebar, LayoutSidebarToggle, MaterialCorners, MaterialElevation,
    MaterialVariant,
};
use orbital::primitives::*;

/// When true, [`UnifiedFieldAppBar`] should render the sidebar toggle.
#[derive(Clone, Copy)]
pub struct ShellSidebarToggle(pub bool);

/// Slot for the app bar region of [`UnifiedFieldShellLayout`].
#[slot]
pub struct ShellAppBar {
    pub children: Children,
}

/// Slot for the left navigation column of [`UnifiedFieldShellLayout`].
#[slot]
pub struct ShellLeftNav {
    pub children: Children,
}

/// Minimal product shell layout for standalone exported apps.
#[component]
pub fn UnifiedFieldShellLayout(
    #[prop(optional)] shell_app_bar: Option<ShellAppBar>,
    #[prop(optional)] shell_left_nav: Option<ShellLeftNav>,
    children: Children,
) -> impl IntoView {
    let sidebar_open = RwSignal::new(true);
    let show_sidebar_toggle = shell_left_nav.is_some();
    provide_context(ShellSidebarToggle(show_sidebar_toggle));

    view! {
        {match (shell_app_bar, shell_left_nav) {
            (Some(ShellAppBar { children: header_children }), Some(ShellLeftNav { children: sidebar_children })) => {
                view! {
                    <Layout
                        overlay_header=true
                        data_testid="unified-field-shell-layout"
                        sidebar_open=sidebar_open
                        layout_header=LayoutHeader { children: header_children }
                        layout_sidebar=LayoutSidebar { children: sidebar_children }
                        layout_main=LayoutMain { children }
                    />
                }.into_any()
            }
            (Some(ShellAppBar { children: header_children }), None) => {
                view! {
                    <Layout
                        overlay_header=true
                        data_testid="unified-field-shell-layout"
                        layout_header=LayoutHeader { children: header_children }
                        layout_main=LayoutMain { children }
                    />
                }.into_any()
            }
            (None, Some(ShellLeftNav { children: sidebar_children })) => {
                view! {
                    <Layout
                        overlay_header=true
                        data_testid="unified-field-shell-layout"
                        sidebar_open=sidebar_open
                        layout_sidebar=LayoutSidebar { children: sidebar_children }
                        layout_main=LayoutMain { children }
                    />
                }.into_any()
            }
            (None, None) => {
                view! {
                    <Layout
                        overlay_header=true
                        data_testid="unified-field-shell-layout"
                        layout_main=LayoutMain { children }
                    />
                }.into_any()
            }
        }}
    }
}

#[derive(Clone, PartialEq)]
pub struct BreadcrumbLink {
    pub title: String,
    pub url: String,
}

impl BreadcrumbLink {
    pub fn new(title: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            url: url.into(),
        }
    }
}

fn product_avatar_letter(app_id: &str) -> char {
    app_id
        .chars()
        .find(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .unwrap_or('?')
}

#[component]
fn AppBarBranding(app_name: String, avatar_letter: String, homepage_url: String) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Logo {
            display: flex;
            align-items: center;
            gap: 12px;
            font-weight: 600;
            font-size: 16px;
            color: var(--orb-color-text-primary);
        }

        .LogoLink {
            display: flex;
            align-items: center;
            gap: 12px;
            text-decoration: none;
            color: inherit;
            transition: opacity 0.2s ease;
        }

        .LogoLink:hover {
            opacity: 0.8;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <a href=homepage_url class=class_names.logo_link>
            <div class=class_names.logo>
                <Avatar config=AvatarConfig {
                    initials: Some(avatar_letter),
                    name: Some(app_name.clone()),
                    shape: AvatarShape::Square,
                    size: Some(32),
                    color: AvatarColor::Brand,
                    ..Default::default()
                } />
                <span>{app_name}</span>
            </div>
        </a>
    }
}

#[component]
fn AppBarBreadcrumbs(breadcrumbs: Vec<BreadcrumbLink>) -> impl IntoView {
    if breadcrumbs.is_empty() {
        return view! { <></> }.into_any();
    }

    view! {
        <Breadcrumb>
            {breadcrumbs.into_iter().map(|breadcrumb| {
                view! {
                    <BreadcrumbItem>
                        <a href=breadcrumb.url.clone() style="text-decoration: none; color: inherit;">
                            <BreadcrumbButton>{breadcrumb.title}</BreadcrumbButton>
                        </a>
                    </BreadcrumbItem>
                }
            }).collect::<Vec<_>>()}
        </Breadcrumb>
    }
    .into_any()
}

/// Minimal app bar for exported app shells.
#[component]
pub fn UnifiedFieldAppBar(
    app_name: String,
    #[prop(optional)] app_id: Option<&'static str>,
    #[prop(optional)] app_logo_initial: Option<String>,
    #[prop(optional)] homepage_url: Option<String>,
    #[prop(optional)] breadcrumbs: Option<Vec<BreadcrumbLink>>,
    #[prop(default = true)] interactive: bool,
    #[prop(default = true)] show_notifications: bool,
    #[prop(optional)] show_sidebar_toggle: Option<bool>,
) -> impl IntoView {
    let _ = show_notifications;
    let avatar_letter = app_id
        .map(|id| product_avatar_letter(id).to_string())
        .or(app_logo_initial)
        .unwrap_or_else(|| {
            app_name
                .chars()
                .next()
                .map(|c| c.to_uppercase().to_string())
                .unwrap_or_else(|| "?".to_string())
        });
    let homepage_url = homepage_url.unwrap_or_else(|| "/".to_string());
    let breadcrumbs = breadcrumbs.unwrap_or_default();
    let show_sidebar_toggle = show_sidebar_toggle
        .or_else(|| use_context::<ShellSidebarToggle>().map(|ctx| ctx.0))
        .unwrap_or(false);

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .LeadingRow {
            display: flex;
            align-items: center;
            gap: 12px;
            min-width: 0;
        }

        .ChromeLocked {
            pointer-events: none;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div class=move || if interactive { String::new() } else { class_names.chrome_locked.to_string() }>
            <AppBar position=AppBarPosition::Sticky density=AppBarDensity::Compact>
                <AppBarMaterial
                    variant=MaterialVariant::Frost
                    elevation=MaterialElevation::Flat
                    corners=MaterialCorners::Square
                    slot
                />
                <AppBarLeading slot>
                    <div class=class_names.leading_row>
                        {show_sidebar_toggle.then(|| view! { <LayoutSidebarToggle /> })}
                        <AppBarBranding
                            app_name=app_name
                            avatar_letter=avatar_letter
                            homepage_url=homepage_url
                        />
                        <AppBarBreadcrumbs breadcrumbs=breadcrumbs />
                    </div>
                </AppBarLeading>
            </AppBar>
        </div>
    }
}
