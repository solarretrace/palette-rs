
#[warn(missing_docs)]
pub mod error;

#[warn(missing_docs)]
pub mod element;

#[warn(missing_docs)]
pub mod palette;

#[warn(missing_docs)]
pub mod metadata;

#[warn(missing_docs)]
pub mod operations;

#[warn(missing_docs)]
pub mod format;

pub use palette::palette::Palette;
pub use palette::palette::PaletteBuilder;
pub use palette::format::SMALL_FORMAT;
pub use palette::format::ZPL_FORMAT;
pub use palette::error::{Error, Result};