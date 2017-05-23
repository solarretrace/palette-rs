// The MIT License (MIT)
//
// Copyright (c) 2016 Skylor R. Schermer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
////////////////////////////////////////////////////////////////////////////////
//!
//! Operations for combining multiple operations together.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use data::Data;
use operation::{
	HistoryEntry,
	OperationInfo,
	PaletteOperation,
};
use result::Result;

// Standard imports.
use std::mem;



////////////////////////////////////////////////////////////////////////////////
// Sequence
////////////////////////////////////////////////////////////////////////////////
/// Applies a sequence of operations to the palette.
///
/// # Example
///
/// ```rust
/// use palette::*;
///
/// let mut pal = Palette::new("Example", Format::Default, true);
///
/// pal.apply(Box::new(
/// 	Sequence::new(vec![
///			Box::new(InsertColor::new(Color::new(10, 10, 10))),
///			Box::new(InsertColor::new(Color::new(20, 20, 20)))
///		])
/// )).unwrap();
///
/// assert_eq!(pal.get_color(Address::new(0, 0, 0)), Some(Color::new(10, 10, 10)));
/// assert_eq!(pal.get_color(Address::new(0, 0, 1)), Some(Color::new(20, 20, 20)));
/// ```
#[derive(Debug)]
pub struct Sequence {
	operations: Vec<Box<PaletteOperation>>
}


impl Sequence {
	/// Creates a new Sequence from the given operation vector.
	#[inline]
	pub fn new(operations: Vec<Box<PaletteOperation>>) -> Sequence {
		Sequence {operations: operations}
	}
}


impl PaletteOperation for Sequence {
	fn get_info(&self) -> OperationInfo {
		OperationInfo {
			name: "Sequence",
			details: Some(format!("{:?}", self))
		}
	}

	fn apply(&mut self, data: &mut Data) -> Result<HistoryEntry> {
		let mut undo_sequence: Vec<Box<PaletteOperation>> = Vec::new();

		let operations = mem::replace(&mut self.operations, Vec::new());
		for mut operation in operations {
			let entry = operation.apply(data)?;
			undo_sequence.push(entry.undo);
		}

		Ok(HistoryEntry {
			info: self.get_info(),
			undo: Box::new(Sequence::new(undo_sequence)),
		})
	}
}


////////////////////////////////////////////////////////////////////////////////
// Repeat
////////////////////////////////////////////////////////////////////////////////
/// Applies a sequence of operations to the palette.
///
/// # Example
///
/// ```rust
/// use palette::*;
///
/// let mut pal = Palette::new("Example", Format::Default, true);
///
/// pal.apply(Box::new(
/// 	Repeat::new(Box::new(
///			InsertColor::new(Color::new(50, 50, 78))
///		)).repeat(3)
/// )).unwrap();
///
/// assert_eq!(pal.get_color(Address::new(0, 0, 0)), Some(Color::new(50, 50, 78)));
/// assert_eq!(pal.get_color(Address::new(0, 0, 1)), Some(Color::new(50, 50, 78)));
/// assert_eq!(pal.get_color(Address::new(0, 0, 2)), Some(Color::new(50, 50, 78)));
/// ```
#[derive(Debug)]
pub struct Repeat {
	repeat_count: usize,
	operation: Box<PaletteOperation>,
}


impl Repeat {
	/// Creates a new Repeat from the given operation vector.
	#[inline]
	pub fn new(operation: Box<PaletteOperation>) -> Repeat {
		Repeat {
			repeat_count: 2,
			operation: operation,
		}
	}

	/// Sets the number of times to repeat the operation.
	#[inline]
	pub fn repeat(mut self, repeat_count: usize) -> Self {
		self.repeat_count = repeat_count;
		self
	}
}


impl PaletteOperation for Repeat {
	fn get_info(&self) -> OperationInfo {
		OperationInfo {
			name: "Repeat",
			details: Some(format!("{:?}", self))
		}
	}

	fn apply(&mut self, data: &mut Data) -> Result<HistoryEntry> {
		let mut undo_sequence: Vec<Box<PaletteOperation>> = Vec::new();

		for _ in 0..self.repeat_count {
			let entry = self.operation.apply(data)?;
			undo_sequence.push(entry.undo);
		}

		Ok(HistoryEntry {
			info: self.get_info(),
			undo: Box::new(Sequence::new(undo_sequence)),
		})
	}
}