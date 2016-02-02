use std::f32;

pub fn nearly_equal(a: f32, b: f32) -> bool {
	let abs_a = a.abs();
	let abs_b = b.abs();
	let diff = (a - b).abs();

	if a == b { // shortcut, handles infinities
		true
	} else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
		// a or b is zero or both are extremely close to it
		// relative error is less meaningful here
		diff < (f32::EPSILON * f32::MIN_POSITIVE)
	} else { // use relative error
		(diff / f32::min(abs_a + abs_b, f32::MAX)) < f32::EPSILON
	}
}

pub fn clamped(val: f32, lb: f32, ub: f32) -> f32 {
	if val < lb {
		lb
	} else if val > ub {
		ub
	} else {
		val
	}
}

pub fn lerp(start: u8, end:u8, amount: f32) -> u8 {
	let a = if start > end {
		1.0 - clamped(amount, 0.0, 1.0)
	} else {
		clamped(amount, 0.0, 1.0)
	};

	let s = if start > end {end} else {start};
	let e = if start > end {start} else {end};
	(((e-s) as f32) * a) as u8 + s
}
