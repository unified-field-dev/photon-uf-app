//! Dashboard stats grid showing topic, subscription, and event counts.

use leptos::prelude::*;
use orbital::components::{AutoGrid, SpacingSize, StatCard};

use crate::server::DashboardStats;

/// Renders the three stat cards on the Photon dashboard.
///
/// Uses `AutoGrid` so the cards reflow responsively based on available width.
/// Does **not** apply outer margin -- use `Flex` with `SpacingSize` at the
/// call site to control vertical spacing.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn PhotonStatsGrid(
    /// Dashboard statistics to display.
    stats: DashboardStats,
) -> impl IntoView {
    let topics = stats.topic_count.to_string();
    let subs = stats.subscription_count.to_string();
    let events = stats.event_count_24h.to_string();

    view! {
        <AutoGrid min="250px" gap=SpacingSize::Size160>
            <StatCard label="Topics" value=topics icon=icondata::AiBellOutlined />
            <StatCard label="Subscriptions" value=subs icon=icondata::AiUnorderedListOutlined />
            <StatCard label="Events (24h)" value=events icon=icondata::AiHistoryOutlined />
        </AutoGrid>
    }
}
