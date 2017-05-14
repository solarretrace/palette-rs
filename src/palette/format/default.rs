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
//! Provides components for interacting with the default palette format.
//!
////////////////////////////////////////////////////////////////////////////////
use palette::Palette;
use palette::operation::PaletteOperation;
use palette;


/// Applies the given operation to the palette.
pub fn apply_operation(
	palette: &mut Palette, 
	mut operation: Box<PaletteOperation>) 
	-> palette::Result<()> 
{
	let data = &mut palette.data;
	let history = &mut palette.operation_history;
	// Apply operation.
	let entry = operation.apply(data)?;
	// Add history entry if history is enabled.
	if let &mut Some(ref mut history) = history {
		history.undo_entries.push(entry);
		history.redo_entries.clear();
	}
	Ok(())
}


/// Reverses the most recently applied operation.
pub fn undo(palette: &mut Palette) -> palette::Result<()> {
	let data = &mut palette.data;
	let history = &mut palette.operation_history;
	// Check if history is enable.
	if let &mut Some(ref mut history) = history {
		// Check for history entry.
		if let Some(mut entry) = history.undo_entries.pop() {
			let redo = entry.undo.apply(data)?;
			history.redo_entries.push(redo);
		}
		Ok(())
	} else {
		panic!("undo not supported")
	}
}


/// Reverses the most recently applied undo operation.
pub fn redo(palette: &mut Palette) -> palette::Result<()> {
	let data = &mut palette.data;
	let history = &mut palette.operation_history;
	// Check if history is enable.
	if let &mut Some(ref mut history) = history {
		// Check for history entry.
		if let Some(mut entry) = history.redo_entries.pop() {
			let undo = entry.undo.apply(data)?;
			history.undo_entries.push(undo);
		}
		Ok(())
	} else {
		panic!("undo not supported")
	}
}