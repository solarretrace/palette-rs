
#[warn(missing_docs)]
pub mod palette;

#[warn(missing_docs)]
pub mod address;

#[warn(missing_docs)]
pub mod utilities;

#[warn(missing_docs)]
pub mod interval;

pub mod color;



pub use color::Color;
pub use palette::{Palette, DefaultPalette, ZplPalette};
pub use address::{Address, Group};
pub use interval::{Boundary, Interval};