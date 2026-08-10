//! DTO shapes and typed blank-id errors for Photon ops contracts.

use serde::{Deserialize, Serialize};

/// Summary of a Photon topic for list/detail views: registry metadata plus
/// recent traffic counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopicSummary {
    /// Topic name as registered with Photon.
    pub topic_name: String,
    /// Optional key field the topic is partitioned/keyed by.
    pub keyed_by: Option<String>,
    /// JSON-encoded schema description for the topic's payload.
    pub schema_json: String,
    /// Number of events published to this topic in the last 24 hours.
    pub event_count_24h: u64,
    /// Number of subscriptions currently registered against this topic.
    pub subscription_count: u32,
}

/// Summary of a Photon subscription for list/detail views: configuration
/// plus read-state visibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionSummary {
    /// Unique subscription identifier.
    pub subscription_id: String,
    /// Human-readable subscription name.
    pub subscription_name: String,
    /// Name of the topic this subscription is attached to.
    pub topic_name: String,
    /// Whether the subscription is currently enabled.
    pub enabled: bool,
    /// Dispatch mode (e.g. at-least-once, durable) as a display string.
    pub mode: String,
    /// Optional topic key filter restricting which keys this subscription receives.
    pub topic_key_filter: Option<String>,
    /// Number of events the subscription's checkpoint is behind the topic head.
    pub checkpoint_lag: i64,
    /// Sequence number of the last event this subscription has processed, if any.
    pub last_seq: Option<i64>,
    /// Timestamp of the last successfully processed event, if any.
    pub last_processed_at: Option<String>,
}

/// Summary of a single Photon event for list views and dashboards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventSummary {
    /// Unique event identifier.
    pub event_id: String,
    /// Name of the topic the event was published to.
    pub topic_name: String,
    /// Optional key the event was published under.
    pub topic_key: Option<String>,
    /// Sequence number of the event within its topic.
    pub seq: i64,
    /// RFC3339 timestamp of when the event was created.
    pub created_at: String,
    /// Short, human-readable preview of the event's delivery status/payload.
    pub payload_preview: String,
}

/// Aggregate counters shown on the Photon dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardStats {
    /// Total number of registered topics.
    pub topic_count: u32,
    /// Total number of registered subscriptions.
    pub subscription_count: u32,
    /// Number of events published across all topics in the last 24 hours.
    pub event_count_24h: u64,
}

/// Full detail record for a single event, including payload and actor context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventDetail {
    /// Unique event identifier.
    pub event_id: String,
    /// Name of the topic the event was published to.
    pub topic_name: String,
    /// Optional key the event was published under.
    pub topic_key: Option<String>,
    /// Sequence number of the event within its topic.
    pub seq: i64,
    /// RFC3339 timestamp of when the event was created.
    pub created_at: String,
    /// Delivery status recorded for the event (e.g. delivered, dead-lettered).
    pub delivery_status: String,
    /// Raw event payload JSON, if the transport record is still available.
    pub payload_json: serde_json::Value,
    /// Raw actor JSON captured at publish time, if the transport record is still available.
    pub actor_json: serde_json::Value,
    /// True when the transport record is no longer available for this event id.
    pub transport_expired: bool,
}

/// Blank topic name, subscription id, or event id rejected before Photon lookups.
///
/// Callers map this into Leptos `ServerFnError` (or equivalent) at the `#[server]`
/// boundary; the Display text stays stable for UI and contract tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PhotonIdError {
    /// Topic name was empty or whitespace-only.
    EmptyTopicName,
    /// Subscription id was empty or whitespace-only.
    EmptySubscriptionId,
    /// Event id was empty or whitespace-only.
    EmptyEventId,
}

impl std::fmt::Display for PhotonIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyTopicName => write!(f, "Photon topic name is required"),
            Self::EmptySubscriptionId => write!(f, "Photon subscription id is required"),
            Self::EmptyEventId => write!(f, "Photon event id is required"),
        }
    }
}

impl std::error::Error for PhotonIdError {}
