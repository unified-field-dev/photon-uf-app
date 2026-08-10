//! Subscription card used in the subscriptions index page.

use leptos::ev;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use orbital::components::{Body1Strong, Caption1, Card, CardButtonArea, CardContent};
use orbital::primitives::{Flex, FlexGap, MaterialElevation, MaterialVariant};

use crate::components::SubscriptionStatusBadge;
use crate::server::SubscriptionSummary;

/// Renders a single subscription card with name, status badge, topic, mode,
/// key filter, checkpoint lag, and last processed time.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn SubscriptionCard(
    /// Subscription data to display.
    sub: SubscriptionSummary,
) -> impl IntoView {
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let href = photon_backend::photon_subscription_path(&sub.subscription_id);
    let sub_name = sub.subscription_name.clone();
    let test_id = format!("sub-{sub_name}");
    let topic = sub.topic_name.clone();
    let mode = sub.mode.clone();
    let key_filter = sub
        .topic_key_filter
        .clone()
        .unwrap_or_else(|| "(all)".to_string());
    let lag = sub.checkpoint_lag;
    let en = sub.enabled;
    let last_view = sub.last_processed_at.map(|t| {
        view! {
            <Caption1 block=true>"Last processed: " {t}</Caption1>
        }
    });

    view! {
        <Card
            attr:data-testid=test_id
            variant=MaterialVariant::Outlined
            elevation=MaterialElevation::Flat
            gap=FlexGap::Size(0)
        >
            <CardButtonArea on_click=Callback::new({
                let nav = nav_store.with_value(|n: &_| n.clone());
                let href = href.clone();
                move |_: ev::MouseEvent| nav(&href, NavigateOptions::default())
            })>
                <CardContent>
                    <Flex vertical=true gap=FlexGap::Size(4)>
                        <Body1Strong block=true>
                            {sub_name}
                            " "
                            <SubscriptionStatusBadge enabled=en />
                        </Body1Strong>
                        <Caption1 block=true>"Topic: " {topic}</Caption1>
                        <Caption1 block=true>"Mode: " {mode}</Caption1>
                        <Caption1 block=true>"Key filter: " {key_filter}</Caption1>
                        <Caption1 block=true>"Checkpoint lag: " {lag}</Caption1>
                        {last_view}
                    </Flex>
                </CardContent>
            </CardButtonArea>
        </Card>
    }
}
