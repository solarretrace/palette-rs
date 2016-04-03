
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



pub use color::{Rgb, Hsl};
pub use palette::{Palette, PaletteExtensions, BasicPalette, ZplPalette};
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
