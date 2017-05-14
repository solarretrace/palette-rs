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
//! Defines operations for duplicating elements.
//!
////////////////////////////////////////////////////////////////////////////////
use super::{
	PaletteOperation, 
	HistoryEntry, 
	OperationInfo, 
	set_target, 
	get_source
};
use palette::{Result, Error};
use palette::data::PaletteOperationData;
use palette::element::ColorElement;
use palette::operation::Undo;
use address::Address;



////////////////////////////////////////////////////////////////////////////////
// CopyColor
////////////////////////////////////////////////////////////////////////////////
/// Inserts a copy of another color into the palette.
/// 
/// # Example
///
/// ```rust
/// use rampeditor::*;
/// 
/// let mut pal = Palette::new("Example", Format::Default, true);
///
/// pal.apply(Box::new(InsertColor::new(Color::new(12, 50, 78)))).unwrap();
/// pal.apply(Box::new(CopyColor::new(Address::new(0, 0, 0)))).unwrap();
///
/// assert_eq!(
/// 	pal.get_color(Address::new(0, 0, 0)), 
/// 	pal.get_color(Address::new(0, 0, 1))
/// );
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct CopyColor {
	/// The address of the element to watch.
	copying: Address,
	/// The location to start placing the colors.
	location: Option<Address>,
	/// Whether to overwrite existing elements when generating new ones.
	overwrite: bool,
}


impl CopyColor {
	/// Creates a new CopyColor operation.
	#[inline]
	pub fn new(copying: Address) -> CopyColor {
		CopyColor {
			copying: copying,
			location: None,
			overwrite: false,
		}
	}

	/// Sets the location to place the color.
	pub fn located_at(mut self, location: Address) -> CopyColor {
		self.location = Some(location);
		self
	}

	/// Configures the operation to overwrite existing elements when inserted.
	pub fn overwrite(mut self, overwrite: bool) -> CopyColor {
		self.overwrite = overwrite;
		self
	}
}


impl PaletteOperation for CopyColor {
	fn get_info(&self) -> OperationInfo {
		OperationInfo {
			name: "Copy Color",
			details: Some(format!("{:?}", self))
		}
	}

	fn apply(&mut self, data: &mut PaletteOperationData) -> Result<HistoryEntry> {
		// Get starting address.
		let starting_address = if let Some(address) = self.location {
			address
		} else {
			data.first_free_address_after(Default::default())?
		};

		// Get targets.
		let target = data.find_targets(
			1, 
			starting_address,
			self.overwrite,
			None
		)?[0];

		// Create new color.
		let new_element = ColorElement::Pure {
			color: data
				.get_slot(self.copying)
				.and_then(|slot| slot.get_color())
				.ok_or(Error::EmptyAddress(self.copying))?
		};

		// Set target.
		let mut undo = Undo::new_for(self);
		set_target(data, target, new_element, &mut undo)?;
		
		Ok(HistoryEntry {
			info: self.get_info(),
			undo: Box::new(undo),
		})
	}
}



////////////////////////////////////////////////////////////////////////////////
// InsertWatcher
////////////////////////////////////////////////////////////////////////////////
/// Inserts a 'watcher' into the palette, a first-order element which will have
/// the same color as another element in the palette.
/// 
/// # Example
///
/// ```rust
/// use rampeditor::*;
/// 
/// let mut pal = Palette::new("Example", Format::Default, true);
///
/// pal.apply(Box::new(InsertColor::new(Color::new(12, 50, 78)))).unwrap();
/// pal.apply(Box::new(InsertWatcher::new(Address::new(0, 0, 0)))).unwrap();
///
/// assert_eq!(
/// 	pal.get_color(Address::new(0, 0, 0)),
/// 	pal.get_color(Address::new(0, 0, 1))
/// );
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct InsertWatcher {
	/// The address of the element to watch.
	watching: Address,
	/// The location to start placing the colors.
	location: Option<Address>,
	/// Whether to overwrite existing elements when generating new ones.
	overwrite: bool,
	/// Whether to generate a placeholder slot when an invalid reference is
	/// given.
	make_sources: bool,
}


impl InsertWatcher {
	/// Creates a new InsertWatcher operation.
	#[inline]
	pub fn new(watching: Address) -> InsertWatcher {
		InsertWatcher {
			watching: watching,
			location: None,
			overwrite: false,
			make_sources: false,
		}
	}

	/// Sets the location to place the watcher.
	pub fn located_at(mut self, location: Address) -> InsertWatcher {
		self.location = Some(location);
		self
	}

	/// Configures the operation to overwrite existing elements when inserted.
	pub fn overwrite(mut self, overwrite: bool) -> InsertWatcher {
		self.overwrite = overwrite;
		self
	}

	/// Configures the operation to generate a placeholder color instead of 
	/// producing an error when empty address is provided. 
	pub fn make_sources(
		mut self, 
		make_sources: bool) 
		-> InsertWatcher 
	{
		self.make_sources = make_sources;
		self
	}
}


impl PaletteOperation for InsertWatcher {
	fn get_info(&self) -> OperationInfo {
		OperationInfo {
			name: "Insert Watcher",
			details: Some(format!("{:?}", self))
		}
	}

	fn apply(&mut self, data: &mut PaletteOperationData) -> Result<HistoryEntry> {
		// Get starting address.
		let starting_address = if let Some(address) = self.location {
			address
		} else {
			data.first_free_address_after(Default::default())?
		};

		// Get targets.
		let target = data.find_targets(
			1, 
			starting_address,
			self.overwrite,
			Some(vec![self.watching])
		)?[0];
		
		// Get source slot.
		let mut undo = Undo::new_for(self);
		let src = get_source(
			data, 
			self.watching, 
			self.make_sources, 
			&mut undo
		)?;
				
		// Generate watcher element.
		let new_element = ColorElement::Mixed {
			mix: Box::new(move |colors| colors[0]),
			sources: vec![src.clone()]
		};

		set_target(data, target, new_element, &mut undo)?;


		Ok(HistoryEntry {
			info: self.get_info(),
			undo: Box::new(undo)
		})
	}
}