

use super::element::Element;
use super::color::Color;

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;


use std::collections::BTreeMap;


pub struct Palette<'p> {
	elements: Vec<Rc<RefCell<Element<'p>>>>
}

impl<'p> Palette<'p> {
	pub fn new() -> Palette<'p> {
		Palette {elements: Vec::new()}
	}

	pub fn get_color(&self, index: usize) -> Color {
		self.elements[index].borrow().get_color()
	}

	pub fn add_element(&mut self, element: Element<'p>) {
		self.elements.push(Rc::new(RefCell::new(element)));
	}

	pub fn get_element(&self, index: usize) -> Rc<RefCell<Element<'p>>> {
		self.elements[index].clone()
	}
}

impl<'p> fmt::Debug for Palette<'p> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(write!(f, "Palette:\n"));
		for ref elem in &self.elements {
			try!(write!(f, "\t{:?} Color = {:?}\n", elem, elem.borrow().get_color()))
		}
		Ok(())
	}
}