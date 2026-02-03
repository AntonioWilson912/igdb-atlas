//! # Timestamp Formatting
//!
//! Utilities for formatting Unix timestamps returned by the IGDB API
//! into human-readable date strings.
//!
//! # Examples
//!
//! ```rust
//! use igdb_atlas::models::timestamp::format_timestamp;
//!
//! assert_eq!(format_timestamp(Some(1432166400)), Some("2015-05-21".to_string()));
//! assert_eq!(format_timestamp(None), None);
//! ```

use chrono::{DateTime, Utc};

/// Formats a Unix timestamp as `YYYY-MM-DD`.
///
/// Returns `None` if the input is `None` or if the timestamp is invalid.
///
/// # Examples
///
/// ```rust
/// use igdb_atlas::models::timestamp::format_timestamp;
///
/// assert_eq!(format_timestamp(Some(0)), Some("1970-01-01".to_string()));
/// assert_eq!(format_timestamp(Some(1432166400)), Some("2015-05-21".to_string()));
/// assert_eq!(format_timestamp(None), None);
/// ```
pub fn format_timestamp(ts: Option<i64>) -> Option<String> {
    ts.and_then(|t| DateTime::from_timestamp(t, 0).map(|dt| dt.format("%Y-%m-%d").to_string()))
}

/// Formats a Unix timestamp as `YYYY-MM-DD HH:MM:SS UTC`.
///
/// Returns `None` if the input is `None` or if the timestamp is invalid.
///
/// # Examples
///
/// ```rust
/// use igdb_atlas::models::timestamp::format_timestamp_full;
///
/// assert_eq!(
///     format_timestamp_full(Some(1432166400)),
///     Some("2015-05-21 00:00:00 UTC".to_string())
/// );
/// assert_eq!(format_timestamp_full(None), None);
/// ```
pub fn format_timestamp_full(ts: Option<i64>) -> Option<String> {
    ts.and_then(|t| {
        DateTime::from_timestamp(t, 0).map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
    })
}

/// Formats a Unix timestamp as a human-readable date (e.g., "May 21, 2015").
///
/// Returns `None` if the input is `None` or if the timestamp is invalid.
///
/// # Examples
///
/// ```rust
/// use igdb_atlas::models::timestamp::format_timestamp_pretty;
///
/// assert_eq!(
///     format_timestamp_pretty(Some(1432166400)),
///     Some("May 21, 2015".to_string())
/// );
/// assert_eq!(format_timestamp_pretty(None), None);
/// ```
pub fn format_timestamp_pretty(ts: Option<i64>) -> Option<String> {
    ts.and_then(|t| DateTime::from_timestamp(t, 0).map(|dt| dt.format("%B %d, %Y").to_string()))
}

/// Formats a Unix timestamp as a relative time description.
///
/// Returns `None` if the input is `None` or if the timestamp is invalid.
///
/// # Examples
///
/// ```rust
/// use igdb_atlas::models::timestamp::format_timestamp_relative;
///
/// // Note: This function returns a relative time description
/// // like "X years ago" based on the current time, so exact
/// // assertions are difficult. This example shows the signature.
/// let result = format_timestamp_relative(Some(1432166400));
/// assert!(result.is_some());
/// ```
pub fn format_timestamp_relative(ts: Option<i64>) -> Option<String> {
    ts.and_then(|t| {
        let dt = DateTime::from_timestamp(t, 0)?;
        let now = Utc::now();
        let duration = now.signed_duration_since(dt);

        if duration.num_days() < 1 {
            Some("today".to_string())
        } else if duration.num_days() < 30 {
            Some(format!("{} days ago", duration.num_days()))
        } else if duration.num_days() < 365 {
            let months = duration.num_days() / 30;
            Some(format!("{} months ago", months))
        } else {
            let years = duration.num_days() / 365;
            Some(format!("{} years ago", years))
        }
    })
}
