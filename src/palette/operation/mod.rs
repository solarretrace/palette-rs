////////////////////////////////////////////////////////////////////////////////
//!
//! Contains definitions for various palette editing operations.
//!
////////////////////////////////////////////////////////////////////////////////

#[warn(missing_docs)]
pub mod common;

#[warn(missing_docs)]
pub mod arrange;

#[warn(missing_docs)]
pub mod duplicate;

#[warn(missing_docs)]
pub mod ramp;

#[warn(missing_docs)]
pub mod sequence;

#[warn(missing_docs)]
pub mod simple;

#[warn(missing_docs)]
pub mod undo;

pub use palette::operation::common::*;

pub use palette::operation::arrange::*;
pub use palette::operation::duplicate::*;
pub use palette::operation::ramp::*;
pub use palette::operation::sequence::*;
pub use palette::operation::simple::*;
pub use palette::operation::undo::*;
