//! # Language Models
//!
//! Models for language definitions and per-game language-support records.
//!
//! | Module | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`language`] | `/languages` | Language definition (name, locale, native name) |
//! | [`language_support`] | `/language_supports` | Links a game ↔ language ↔ support type |
//! | [`language_support_type`] | `/language_support_types` | Support-type reference (Audio / Subtitles / Interface) |

pub mod language;
pub mod language_support;
pub mod language_support_type;

pub use language::Language;
pub use language_support::LanguageSupport;
pub use language_support_type::LanguageSupportType;
