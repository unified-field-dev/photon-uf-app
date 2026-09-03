use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use orbital::components::{Card, ContentContainer, SpacingSize, Subtitle2, Title3};
use orbital::primitives::{Button, ButtonAppearance, Flex, MessageBar, MessageBarIntent, Space};

use crate::components::{
    ActiveSubscriptionsTable, EventsTable, EventsTableColumns, PhotonStatsGrid,
};
use crate::server::{get_dashboard_stats, get_recent_events, get_subscriptions};

/// Photon dashboard: aggregate stats, recent events, and active subscriptions at a glance.
#[component]
pub fn PhotonDashboardPage() -> impl IntoView {
    let navigate = use_navigate();
    let nav_events = navigate.clone();
    let nav_subs = navigate.clone();
    let stats_res = Resource::new(|| (), |()| async move { get_dashboard_stats().await });
    let events_res = Resource::new(|| (), |()| async move { get_recent_events(10).await });
    let subs_res = Resource::new(|| (), |()| async move { get_subscriptions().await });

    view! {
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

                <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                    <Space>
                        <Subtitle2>"Recent Events"</Subtitle2>
                        <div id="photon-dashboard-view-events">
                            <Button
                                appearance=ButtonAppearance::Subtle
                                on_click=Callback::new(move |_| nav_events(crate::paths::EVENTS, NavigateOptions::default()))
                            >
                                "View All \u{2192}"
                            </Button>
                        </div>
                    </Space>
                    <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                        {move || match events_res.get() {
                            Some(Ok(events)) => view! {
                                <div id="photon-dashboard-recent-events">
                                    <Card>
                                        <EventsTable
                                            events=events
                                            columns=EventsTableColumns {
                                                show_event_id: false,
                                                show_key: true,
                                                show_topic: true,
                                                show_seq: true,
                                                show_created: true,
                                            }
                                        />
                                    </Card>
                                </div>
                            }.into_any(),
                            Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                            None => view! { <Card>"Loading..."</Card> }.into_any(),
                        }}
                    </Suspense>
                </Flex>

                <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                    <Space>
                        <Subtitle2>"Active Subscriptions"</Subtitle2>
                        <div id="photon-dashboard-view-subs">
                            <Button
                                appearance=ButtonAppearance::Subtle
                                on_click=Callback::new(move |_| nav_subs(crate::paths::SUBSCRIPTIONS, NavigateOptions::default()))
                            >
                                "View All \u{2192}"
                            </Button>
                        </div>
                    </Space>
                    <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                        {move || match subs_res.get() {
                            Some(Ok(subs)) => view! {
                                <div id="photon-dashboard-active-subs">
                                    <Card>
                                        <ActiveSubscriptionsTable subs=subs />
                                    </Card>
                                </div>
                            }.into_any(),
                            Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                            None => view! { <Card>"Loading..."</Card> }.into_any(),
                        }}
                    </Suspense>
                </Flex>
            </Flex>
        </ContentContainer>
    }
}
