
#[warn(missing_docs)]
pub mod palette;
#[warn(missing_docs)]
pub mod address;
#[warn(missing_docs)]
pub mod utilities;
#[warn(missing_docs)]
pub mod interval;
#[warn(missing_docs)]
pub mod color;
#[warn(missing_docs)]
pub mod gui;


pub use color::{Color, Cmyk, Hsl, Hsv, Rgb, Xyz};
pub use palette::Palette;
// pub use palette::{Palette, PaletteExtensions, BasicPalette, ZplPalette};
pub use address::{Address, Group, Selection};
pub use interval::{Bound, Interval};
pub use palette::operation::{
	InsertColor,
	RemoveElement,
	CopyColor,
	InsertWatcher,
	SequenceOperation,
	RepeatOperation,
	InsertRamp,
};
