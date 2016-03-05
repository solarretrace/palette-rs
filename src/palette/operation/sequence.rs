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
//! Defines operations for combining multiple operations together.
//!
////////////////////////////////////////////////////////////////////////////////
use super::common::{PaletteOperation, HistoryEntry, OperationInfo};
use palette::Result;
use palette::data::PaletteData;

use std::mem;
// Sequence
// Repeat



////////////////////////////////////////////////////////////////////////////////
// SequenceOperation
////////////////////////////////////////////////////////////////////////////////
/// Applies a sequence of operations to the palette.
/// 
/// # Example
///
/// ```rust
/// use rampeditor::*;
/// 
/// let mut pal = BasicPalette::new("Example");
/// ```
#[derive(Debug)]
pub struct SequenceOperation {
	operations: Vec<Box<PaletteOperation>>
}


impl SequenceOperation {
	/// Creates a new SequenceOperation from the given operation vector.
	#[inline]
	pub fn new(operations: Vec<Box<PaletteOperation>>) -> SequenceOperation {
		SequenceOperation {operations: operations}
	}
}


impl PaletteOperation for SequenceOperation {
	fn get_info(&self) -> OperationInfo {
		OperationInfo {
			name: "Sequence",
			details: Some(format!("{:?}", self))
		}
	}

	fn apply(&mut self, data: &mut PaletteData) -> Result<HistoryEntry> {
		let mut undo_sequence: Vec<Box<PaletteOperation>> = Vec::new();

		let operations = mem::replace(&mut self.operations, Vec::new());
		for mut operation in operations {
			let entry = try!(operation.apply(data));
			undo_sequence.push(entry.undo);
		}
		
		Ok(HistoryEntry {
			info: self.get_info(),
			undo: Box::new(SequenceOperation::new(undo_sequence)),
		})
	}
}


////////////////////////////////////////////////////////////////////////////////
// RepeatOperation
////////////////////////////////////////////////////////////////////////////////
/// Applies a sequence of operations to the palette.
/// 
/// # Example
///
/// ```rust
/// use rampeditor::*;
/// 
/// let mut pal = BasicPalette::new("Example");
/// ```
#[derive(Debug)]
pub struct RepeatOperation {
	repeat_count: usize,
	operation: Box<PaletteOperation>,
}


impl RepeatOperation {
	/// Creates a new RepeatOperation from the given operation vector.
	#[inline]
	pub fn new(operation: Box<PaletteOperation>) -> RepeatOperation {
		RepeatOperation {
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


impl PaletteOperation for RepeatOperation {
	fn get_info(&self) -> OperationInfo {
		OperationInfo {
			name: "Repeat",
			details: Some(format!("{:?}", self))
		}
	}
	
	fn apply(&mut self, data: &mut PaletteData) -> Result<HistoryEntry> {
		let mut undo_sequence: Vec<Box<PaletteOperation>> = Vec::new();

		for _ in 0..self.repeat_count {
			let entry = try!(self.operation.apply(data));
			undo_sequence.push(entry.undo);
		}
		
		Ok(HistoryEntry {
			info: self.get_info(),
			undo: Box::new(SequenceOperation::new(undo_sequence)),
		})
	}
}
