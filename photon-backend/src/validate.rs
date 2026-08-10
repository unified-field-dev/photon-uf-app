//! Blank-id rejection and event-list limit capping.

use crate::types::PhotonIdError;

/// Maximum events returned by a single `get_events` / `get_recent_events` call.
pub const MAX_EVENT_LIST_LIMIT: u32 = 100;

/// Rejects blank topic names before registry / detail lookups.
///
/// # Errors
///
/// Returns [`PhotonIdError::EmptyTopicName`] when `topic_name` is empty or whitespace-only.
pub fn validate_topic_name(topic_name: &str) -> Result<(), PhotonIdError> {
    if topic_name.trim().is_empty() {
        Err(PhotonIdError::EmptyTopicName)
    } else {
        Ok(())
    }
}

/// Rejects blank subscription ids before detail lookups.
///
/// # Errors
///
/// Returns [`PhotonIdError::EmptySubscriptionId`] when `id` is empty or whitespace-only.
pub fn validate_subscription_id(id: &str) -> Result<(), PhotonIdError> {
    if id.trim().is_empty() {
        Err(PhotonIdError::EmptySubscriptionId)
    } else {
        Ok(())
    }
}

/// Rejects blank event ids before detail lookups.
///
/// # Errors
///
/// Returns [`PhotonIdError::EmptyEventId`] when `id` is empty or whitespace-only.
pub fn validate_event_id(id: &str) -> Result<(), PhotonIdError> {
    if id.trim().is_empty() {
        Err(PhotonIdError::EmptyEventId)
    } else {
        Ok(())
    }
}

/// Cap requested event list limits to [`MAX_EVENT_LIST_LIMIT`].
#[must_use]
pub fn clamp_event_list_limit(limit: u32) -> usize {
    limit.min(MAX_EVENT_LIST_LIMIT) as usize
}
