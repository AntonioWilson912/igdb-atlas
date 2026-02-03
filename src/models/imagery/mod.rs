//! # Imagery Models
//!
//! All image-bearing models from the IGDB API, regardless of the
//! entity they belong to.  Each struct provides a `url(size)` helper
//! that constructs a full IGDB image URL from its `image_id`.
//!
//! | Module | Endpoint | Attached to |
//! |--------|----------|-------------|
//! | [`artwork`] | `/artworks` | Game |
//! | [`artwork_type`] | `/artwork_types` | Artwork |
//! | [`character_mug_shot`] | `/character_mug_shots` | Character |
//! | [`company_logo`] | `/company_logos` | Company |
//! | [`cover`] | `/covers` | Game / Game Localization |
//! | [`event_logo`] | `/event_logos` | Event |
//! | [`game_engine_logo`] | `/game_engine_logos` | Game Engine |
//! | [`platform_logo`] | `/platform_logos` | Platform |
//! | [`screenshot`] | `/screenshots` | Game |

pub mod artwork;
pub mod artwork_type;
pub mod character_mug_shot;
pub mod company_logo;
pub mod cover;
pub mod event_logo;
pub mod game_engine_logo;
pub mod platform_logo;
pub mod screenshot;

pub use artwork::Artwork;
pub use artwork_type::ArtworkType;
pub use character_mug_shot::CharacterMugShot;
pub use company_logo::CompanyLogo;
pub use cover::Cover;
pub use event_logo::EventLogo;
pub use game_engine_logo::GameEngineLogo;
pub use platform_logo::PlatformLogo;
pub use screenshot::Screenshot;

pub trait UrlFor {
    fn image_id(&self) -> Option<&str>;

    fn url(&self, size: &str) -> Option<String> {
        self.image_id().map(|img_id| {
            format!(
                "//images.igdb.com/igdb/image/upload/t_{}/{}.jpg",
                size, img_id
            )
        })
    }
}

macro_rules! impl_url_for {
    ($($t:ty),*) => {
        $(
            impl UrlFor for $t {
                fn image_id(&self) -> Option<&str> {
                    self.image_id.as_deref()
                }
            }
        )*
    };
}

impl_url_for!(
    Artwork,
    CharacterMugShot,
    CompanyLogo,
    Cover,
    EventLogo,
    GameEngineLogo,
    PlatformLogo,
    Screenshot
);
