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

pub use palette::operations::common::*;

pub use palette::operations::arrange::*;
pub use palette::operations::duplicate::*;
pub use palette::operations::ramp::*;
pub use palette::operations::sequence::*;
pub use palette::operations::simple::*;
pub use palette::operations::undo::*;
