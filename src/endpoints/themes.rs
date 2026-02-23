//! # Themes Endpoint
//!
//! Fluent interface for the IGDB `/themes` endpoint.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`ThemesEndpoint`] | `/themes` | [`Theme`] |

use crate::models::themes::Theme;

define_endpoint! {
    /// `/themes` - searchable.
    pub struct ThemesEndpoint => "themes", Theme, searchable
}
