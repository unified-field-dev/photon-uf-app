//! Page components for the Photon ops UI.

/// Dashboard overview page.
pub mod dashboard;
/// Event detail page.
pub mod event_detail;
/// Events index page.
pub mod events;
/// Subscription detail page.
pub mod subscription_detail;
/// Subscriptions index page.
pub mod subscriptions;
/// Topic detail page.
pub mod topic_detail;
/// Topics index page.
pub mod topics;

pub use dashboard::PhotonDashboardPage;
pub use event_detail::PhotonEventDetailPage;
pub use events::PhotonEventsIndexPage;
pub use subscription_detail::PhotonSubscriptionDetailPage;
pub use subscriptions::PhotonSubscriptionsIndexPage;
pub use topic_detail::PhotonTopicDetailPage;
pub use topics::PhotonTopicsIndexPage;
