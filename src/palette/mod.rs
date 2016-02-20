
#[warn(missing_docs)]
pub mod error;

#[warn(missing_docs)]
pub mod element;

#[warn(missing_docs)]
pub mod data;

#[warn(missing_docs)]
pub mod metadata;

#[warn(missing_docs)]
pub mod operations;

#[warn(missing_docs)]
pub mod format;

#[warn(missing_docs)]
pub mod formats;

pub use palette::data::PaletteData;
pub use palette::format::Palette;
pub use palette::formats::ZplFormat;
pub use palette::error::{Error, Result};