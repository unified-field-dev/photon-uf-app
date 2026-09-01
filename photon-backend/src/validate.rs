//! Blank-id rejection, unsafe-id rejection, event-list limit capping, and
//! path-segment encoding for ops UI hrefs.

use crate::types::PhotonIdError;

/// Maximum events returned by a single `get_events` / `get_recent_events` call.
pub const MAX_EVENT_LIST_LIMIT: u32 = 100;

/// Maximum Unicode scalar count for topic names, subscription ids, and event ids
/// accepted by ops detail lookups.
pub const MAX_PHOTON_ID_CHARS: usize = 256;

const fn is_unsafe_ops_id_char(c: char) -> bool {
    c.is_control() || c == '/' || c == '\\'
}

fn check_ops_id(raw: &str) -> Result<&str, PhotonIdErrorKind> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(PhotonIdErrorKind::Empty);
    }
    if trimmed.chars().count() > MAX_PHOTON_ID_CHARS {
        return Err(PhotonIdErrorKind::TooLong);
    }
    if trimmed == "." || trimmed == ".." {
        return Err(PhotonIdErrorKind::Unsafe);
    }
    if trimmed.chars().any(is_unsafe_ops_id_char) {
        return Err(PhotonIdErrorKind::Unsafe);
    }
    Ok(trimmed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PhotonIdErrorKind {
    Empty,
    TooLong,
    Unsafe,
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` topic names
/// before registry / detail lookups.
///
/// # Errors
///
/// Returns a [`PhotonIdError`] variant when the name is empty/whitespace-only,
/// longer than [`MAX_PHOTON_ID_CHARS`], contains `/` `\` or ASCII controls, or is
/// exactly `.` / `..`.
pub fn validate_topic_name(topic_name: &str) -> Result<(), PhotonIdError> {
    match check_ops_id(topic_name) {
        Ok(_) => Ok(()),
        Err(PhotonIdErrorKind::Empty) => Err(PhotonIdError::EmptyTopicName),
        Err(PhotonIdErrorKind::TooLong) => Err(PhotonIdError::TopicNameTooLong),
        Err(PhotonIdErrorKind::Unsafe) => Err(PhotonIdError::UnsafeTopicName),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` subscription
/// ids before detail lookups.
///
/// # Errors
///
/// Returns a [`PhotonIdError`] variant when the id fails the same rules as
/// [`validate_topic_name`].
pub fn validate_subscription_id(id: &str) -> Result<(), PhotonIdError> {
    match check_ops_id(id) {
        Ok(_) => Ok(()),
        Err(PhotonIdErrorKind::Empty) => Err(PhotonIdError::EmptySubscriptionId),
        Err(PhotonIdErrorKind::TooLong) => Err(PhotonIdError::SubscriptionIdTooLong),
        Err(PhotonIdErrorKind::Unsafe) => Err(PhotonIdError::UnsafeSubscriptionId),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` event ids
/// before detail lookups.
///
/// # Errors
///
/// Returns a [`PhotonIdError`] variant when the id fails the same rules as
/// [`validate_topic_name`].
pub fn validate_event_id(id: &str) -> Result<(), PhotonIdError> {
    match check_ops_id(id) {
        Ok(_) => Ok(()),
        Err(PhotonIdErrorKind::Empty) => Err(PhotonIdError::EmptyEventId),
        Err(PhotonIdErrorKind::TooLong) => Err(PhotonIdError::EventIdTooLong),
        Err(PhotonIdErrorKind::Unsafe) => Err(PhotonIdError::UnsafeEventId),
    }
}

/// Cap requested event list limits to [`MAX_EVENT_LIST_LIMIT`].
#[must_use]
pub fn clamp_event_list_limit(limit: u32) -> usize {
    limit.min(MAX_EVENT_LIST_LIMIT) as usize
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

/// Percent-encode a single path segment for `/photon/...` hrefs.
///
/// Leaves RFC 3986 unreserved characters alone (`ALPHA` / `DIGIT` / `-` `.` `_`
/// `~`). Encodes `/`, `\`, controls, spaces, and other bytes so Orbital
/// `paths::*` format strings cannot smuggle extra path segments.
#[must_use]
pub fn encode_ops_path_segment(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for &b in raw.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char);
            }
            _ => {
                out.push('%');
                out.push(HEX[(b >> 4) as usize] as char);
                out.push(HEX[(b & 0x0f) as usize] as char);
            }
        }
    }
    out
}

/// `/photon/topics/{encoded}` detail href.
#[must_use]
pub fn photon_topic_path(topic_name: &str) -> String {
    format!("/photon/topics/{}", encode_ops_path_segment(topic_name))
}

/// `/photon/subscriptions/{encoded}` detail href.
#[must_use]
pub fn photon_subscription_path(subscription_id: &str) -> String {
    format!(
        "/photon/subscriptions/{}",
        encode_ops_path_segment(subscription_id)
    )
}

/// `/photon/events/{encoded}` detail href.
#[must_use]
pub fn photon_event_path(event_id: &str) -> String {
    format!("/photon/events/{}", encode_ops_path_segment(event_id))
}
