use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{Card, ContentContainer, SpacingSize, Subtitle2, Title3};
use orbital::primitives::*;

use crate::components::{ActiveSubscriptionsTable, EventsTable, PhotonStatsGrid};
use crate::server::{get_dashboard_stats, get_recent_events, get_subscriptions};

/// Photon dashboard: aggregate stats, recent events, and active subscriptions at a glance.
#[component]
pub fn PhotonDashboardPage() -> impl IntoView {
    let navigate = use_navigate();
    let nav_events = navigate.clone();
    let nav_subs = navigate.clone();
    let stats_res = Resource::new(|| (), |_| async move { get_dashboard_stats().await });
    let events_res = Resource::new(|| (), |_| async move { get_recent_events(10).await });
    let subs_res = Resource::new(|| (), |_| async move { get_subscriptions().await });

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .SectionHeader {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 16px;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="photon-dashboard">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Photon Dashboard"</Title3>

                <Suspense fallback=move || view! { <div>"Loading..."</div> }>
                    {move || match stats_res.get() {
                        Some(Ok(stats)) => view! {
                            <PhotonStatsGrid stats=stats />
                        }.into_any(),
                        Some(Err(e)) => view! {
                            <MessageBar intent=MessageBarIntent::Error>"Failed to load stats: " {e.to_string()}</MessageBar>
                        }.into_any(),
                        None => view! { <div>"Loading..."</div> }.into_any(),
                    }}
                </Suspense>

                <div>
                    <div class=class_names.section_header>
                        <Subtitle2>"Recent Events"</Subtitle2>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            on_click=Callback::new(move |_| nav_events(crate::paths::EVENTS, Default::default()))
                        >
                            "View All \u{2192}"
                        </Button>
                    </div>
                    <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                        {move || match events_res.get() {
                            Some(Ok(events)) => view! {
                                <Card>
                                    <EventsTable
                                        events=events
                                        show_event_id=false
                                        show_key=true
                                        show_topic=true
                                        show_seq=true
                                        show_created=true
                                    />
                                </Card>
                            }.into_any(),
                            Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                            None => view! { <Card>"Loading..."</Card> }.into_any(),
                        }}
                    </Suspense>
                </div>

                <div>
                    <div class=class_names.section_header>
                        <Subtitle2>"Active Subscriptions"</Subtitle2>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            on_click=Callback::new(move |_| nav_subs(crate::paths::SUBSCRIPTIONS, Default::default()))
                        >
                            "View All \u{2192}"
                        </Button>
                    </div>
                    <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                        {move || match subs_res.get() {
                            Some(Ok(subs)) => view! {
                                <Card>
                                    <ActiveSubscriptionsTable subs=subs />
                                </Card>
                            }.into_any(),
                            Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                            None => view! { <Card>"Loading..."</Card> }.into_any(),
                        }}
                    </Suspense>
                </div>
            </Flex>
        </ContentContainer>
    }
}
