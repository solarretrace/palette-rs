
extern crate ramptools;

use ramptools::core::color::Color;
use ramptools::core::palette::Palette;
use ramptools::core::element::Element;


fn main() {
	
	let c1 = Color(34, 68, 122);
	let c2 = Color(188, 35, 123);
	let c3 = Color(0, 0, 0);
	let mut pal = Palette::new();
	pal.add_element(Element::ZerothOrder {color: c1});
	pal.add_element(Element::ZerothOrder {color: c2});
	pal.add_element(Element::ZerothOrder {color: c3});
	let e = Element::FirstOrder {
		parent: pal.get_element(0),
		build: Box::new(|elem| {
			let c = elem.get_color();
			Color(c.0 + 1, c.1 + 2, c.2 + 3)
		})
	};
	pal.add_element(e);
	
	println!("{:?}", pal);
}
