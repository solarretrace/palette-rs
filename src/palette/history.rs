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
//! Provides definitions relevant to maintaining an operation history.
//!
////////////////////////////////////////////////////////////////////////////////
use palette::operation::PaletteOperation;


////////////////////////////////////////////////////////////////////////////////
// OperationHistory
////////////////////////////////////////////////////////////////////////////////
/// Maintains a history of operations applied to a palette and their associated
/// undo operations.
#[derive(Debug)]
pub struct OperationHistory {
	/// The record of applied operations an undo operations.
	records: Vec<HistoryEntry>,
}


impl OperationHistory {
	/// Creates a new, empty OperationHistory
	pub fn new() -> OperationHistory {
		OperationHistory {records: Vec::new()}
	}

	/// Pushes a new operation history entry onto the operation stack.
	pub fn push(&mut self, entry: HistoryEntry)	{
		self.records.push(entry);
	}

	/// pops the last operation history entry off the operation stack and 
	/// returns it. Returns None if the history is empty.
	pub fn pop(&mut self) -> Option<HistoryEntry> {
		self.records.pop()
	}
}



////////////////////////////////////////////////////////////////////////////////
// HistoryEntry
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates a single entry in the operation history.
#[derive(Debug)]
pub struct HistoryEntry {
	/// Information about the operation that was applied to the palette.
	pub info: EntryInfo,
	/// The operation that undoes the applied operation.
	pub undo: Box<PaletteOperation>,
}



////////////////////////////////////////////////////////////////////////////////
// EntryInfo
////////////////////////////////////////////////////////////////////////////////
/// Encapsulates information about the operation that was performed.
#[derive(Debug)]
pub enum EntryInfo {
	/// An undo uperation was applied.
	Undo,
	/// The given operation was applied.
	Apply {
		/// The operation that was applied.
		operation: Box<PaletteOperation>,
	}
}