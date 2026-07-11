use leptos::prelude::*;
use orbital::primitives::*;

/// Badge for subscription enabled/disabled status.
#[component]
pub fn SubscriptionStatusBadge(
    /// Whether the subscription is enabled
    enabled: bool,
) -> impl IntoView {
    let (label, appearance, color) = if enabled {
        ("ON", BadgeAppearance::Filled, BadgeColor::Success)
    } else {
        ("OFF", BadgeAppearance::Outline, BadgeColor::Warning)
    };

    view! {
        <Badge appearance=appearance color=color>{label}</Badge>
    }
}
