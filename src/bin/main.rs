

extern crate color;
extern crate palette;

use palette::*;
use palette::operation::*;

pub fn main() {
	let mut pal = Palette::new("test", Format::Default, true);

	pal.apply(Box::new(InsertCell::new())).unwrap();
	println!("{}", pal);
	pal.undo().unwrap();

	println!("{}", pal);
}