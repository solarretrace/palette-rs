//! Provides a set of interfaces and implementations for converting palettes 
//! between different formats.

#[warn(missing_docs)]
pub mod format;

#[warn(missing_docs)]
#[allow(dead_code)]
pub mod small;

#[warn(missing_docs)]
#[allow(dead_code)]
pub mod zpl;



pub use palette::format::format::*;
pub use palette::format::small::*;
pub use palette::format::zpl::*;