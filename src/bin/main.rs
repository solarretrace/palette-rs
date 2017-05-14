

extern crate color;
extern crate palette;

use palette::*;

pub fn main() {
	let mut pal = Palette::new("test", Format::Default, true);

	pal.apply(Box::new(InsertColor::new(Color::new(0, 12, 13)))).unwrap();
	pal.apply(Box::new(InsertWatcher::new(Address::new(0, 0, 0)))).unwrap();
	pal.apply(Box::new(InsertColor::new(Color::new(100, 12, 13))
		.located_at(Address::new(0, 0, 0))
		.overwrite(true)
		)).unwrap();
	// pal.undo().unwrap();

	println!("{}", pal);
}