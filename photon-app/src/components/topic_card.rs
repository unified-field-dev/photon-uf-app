//! Topic card used in the topics index page.

use leptos::ev;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use orbital::components::{Body1Strong, Caption1, Card, CardButtonArea, CardContent, CardFooter};
use orbital::primitives::{
    Button, ButtonAppearance, ButtonSize, Flex, FlexGap, MaterialElevation, MaterialVariant,
};

use crate::server::TopicSummary;

/// Renders a single topic card with name, keyed-by, schema, event/subscription
/// counts, and action buttons.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn TopicCard(
    /// Topic data to display.
    topic: TopicSummary,
) -> impl IntoView {
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let name = topic.topic_name.clone();
    let keyed = topic.keyed_by.is_some();
    let test_id = format!("topic-{}", &name);
    let name_click = name.clone();
    let name_btn = name.clone();
    let keyed_by_view = topic.keyed_by.clone().map(|k| {
        view! {
            <Caption1 block=true>"Keyed by: " {k}</Caption1>
        }
    });
    let schema = topic.schema_json.clone();
    let event_count = topic.event_count_24h;
    let sub_count = topic.subscription_count;

    view! {
        <Card
            attr:data-testid=test_id
            variant=MaterialVariant::Outlined
            elevation=MaterialElevation::Flat
            gap=FlexGap::Size(0)
        >
            <CardButtonArea on_click=Callback::new({
                let nav = nav_store.with_value(|n: &_| n.clone());
                let name_click = name_click.clone();
                move |_: ev::MouseEvent| nav(&crate::paths::topic(&name_click), NavigateOptions::default())
            })>
                <CardContent>
                    <Flex vertical=true gap=FlexGap::Size(4)>
                        <Body1Strong block=true>{name.clone()}{if keyed { " KEYED" } else { "" }}</Body1Strong>
                        {keyed_by_view}
                        <Caption1 block=true>"Schema: " {schema}</Caption1>
                        <Caption1 block=true>"Events (24h): " {event_count} " | Subscriptions: " {sub_count}</Caption1>
                    </Flex>
                </CardContent>
            </CardButtonArea>
            <CardFooter>
                <Flex gap=FlexGap::Small>
                    <Button size=ButtonSize::Small appearance=ButtonAppearance::Subtle on_click=Callback::new({
                        let nav = nav_store.with_value(|n: &_| n.clone());
                        move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                            nav(&crate::paths::topic(&name_btn), NavigateOptions::default());
                        }
                    })>"View"</Button>
                    <Button size=ButtonSize::Small appearance=ButtonAppearance::Subtle on_click=Callback::new({
                        let nav = nav_store.with_value(|n: &_| n.clone());
                        move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                            nav(crate::paths::EVENTS, NavigateOptions::default());
                        }
                    })>"View Events"</Button>
                    <Button size=ButtonSize::Small appearance=ButtonAppearance::Subtle on_click=Callback::new({
                        let nav = nav_store.with_value(|n: &_| n.clone());
                        move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                            nav(crate::paths::SUBSCRIPTIONS, NavigateOptions::default());
                        }
                    })>"View Subscriptions"</Button>
                </Flex>
            </CardFooter>
        </Card>
    }
}
