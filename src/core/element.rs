
use super::color::Color;

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

pub enum Element<'p> {
	ZerothOrder {
		color: Color
	},
	FirstOrder {
		parent: Rc<RefCell<Element<'p>>>, 
		build: Box<Fn(&Element) -> Color>,
	}
}

impl<'p> Element<'p> {
	pub fn get_color(&self) -> Color {
		match *self {
			Element::ZerothOrder {color} 
				=> color,
			Element::FirstOrder {ref parent, ref build} 
				=> build(&*parent.borrow()),
		}
	}
}

impl<'p> fmt::Debug for Element<'p> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match *self {
			Element::ZerothOrder {ref color} 
				=> write!(f, "Element::ZerothOrder({:?})", color),
			Element::FirstOrder {ref parent, ref build} 
				=> write!(f, "Element::FirstOrder({:?})", parent),
		}
	}
}