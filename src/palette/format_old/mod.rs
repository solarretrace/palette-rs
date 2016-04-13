//! Provides a set of interfaces and implementations for converting palettes 
//! between different formats.

#[warn(missing_docs)]
pub mod basic;
#[warn(missing_docs)]
#[allow(dead_code)]
pub mod zpl;

pub use palette::format::basic::BasicPalette;
pub use palette::format::zpl::ZplPalette;

use palette::operation::PaletteOperation;
use palette;
use address::Address;
use color::Rgb;

use std::fmt;
use std::io::{Result, Write, Read};
use std::io;

////////////////////////////////////////////////////////////////////////////////
// Palette
////////////////////////////////////////////////////////////////////////////////
/// Specifies the interface for a palette with convenience operations.
pub trait PaletteExtensions : Palette {
	/// Reverses the most recently applied operation.
	fn undo(&mut self) -> palette::Result<()>;

	/// Reverses the most recently applied undo operation.
	fn redo(&mut self) -> palette::Result<()>;

	/// Applies the given operation to the palette.
	fn apply<O>(&mut self, operation: O) -> palette::Result<()>
		where O: PaletteOperation + 'static
	{
		// Defer to the Palette implementation, but box the operation for 
		// convenience.
		self.apply_operation(Box::new(operation))
	}
}


////////////////////////////////////////////////////////////////////////////////
// Palette
////////////////////////////////////////////////////////////////////////////////
/// Specifies the interface for using a specific palette format.
pub trait Palette : fmt::Debug {
	/// Creates a new palette with the given name.
	fn new<S>(name: S) -> Self where S: Into<String>;

	/// Returns the color at the given address, or None if the slot is empty.
	fn get_color(&self, address: Address) -> Option<Rgb>;

	/// Returns the number of elements in the palette.
	fn len(&self) -> usize;

	/// Applies the given operation to the palette. Usually, this will just 
	/// defer to the PaletteOperation's apply method, but this could also 
	/// provide extra functionality such as undo/redo and format-specific 
	/// checks.
	fn apply_operation(
		&mut self, 
		mut operation: Box<PaletteOperation>) 
		-> palette::Result<()>;

	/// Writes the palette to the given buffer.
	#[allow(unused_variables)]
	fn write_palette<W>(&self, out_buf: &mut W) -> io::Result<()> 
		where W: io::Write
	{
		unimplemented!()
	}

	/// Reads a palette from the given buffer.
	#[allow(unused_variables)]
	fn read_palette<R>(in_buf: &R) -> io::Result<Self>
		where R: io::Read
	{
		unimplemented!()
	}
}