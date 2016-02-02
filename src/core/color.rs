
use super::utilities::lerp;

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);



pub fn lerp_color(start: Color, end: Color, amount: f32) -> Color {
	Color(
		lerp(start.0, end.0, amount),
		lerp(start.1, end.1, amount),
		lerp(start.2, end.2, amount)
	)
}