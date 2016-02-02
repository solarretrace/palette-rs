
use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord, Copy, Clone)]
struct Address {
	pub page: u16,
	pub line: u8,
	pub column: u8,
}

impl Address {
	fn new(page: u16, line: u8, column: u8) -> Self {
		Address {
			page: page,
			line: line,
			column: column,
		}
	}

	fn next(&self, lines_per_page: u8, columns_per_line: u8) {
		let mut next = Address::new(
			self.page,
			self.line,
			self.column.wrapping_add(1)
		);
		if next.column % columns_per_line == 0 { 
			next.column = 0;
			next.line.wrapping_add(1);
			if next.line % lines_per_page == 0 {
				next.line = 0;
				next.page.wrapping_add(1);
				if next.page == 0 {
					panic!("Address.next called on maximum Address.");
				}
			}
		}
	}
}

impl fmt::Display for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, 
			"{}:{}:{}",
			self.page,
			self.line,
			self.column
		)
	}
}