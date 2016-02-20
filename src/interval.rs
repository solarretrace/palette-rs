// The MIT License (MIT)
// 
// Copyright (c) 2016 Skylor R. Schermer
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in 
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides a basic interval type for doing complex set selections.
//!
////////////////////////////////////////////////////////////////////////////////
use std::ops::{Deref, Sub};
use std::cmp::Ord;


////////////////////////////////////////////////////////////////////////////////
// Boundary
////////////////////////////////////////////////////////////////////////////////
///
/// Determines the type of an interval's boundary.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Boundary<T> {
    /// The boundary is includes the point.
    Include(T),
    /// The boundary is excludes the point.
    Exclude(T),
}

impl<T> Boundary<T> {
    /// Returns whether the boundary includes its point.
    pub fn is_inclusive(&self) -> bool {
        match self {
            &Boundary::Include(..) => true,
            &Boundary::Exclude(..) => false
        }
    }
}

// Implemented to prevent having to match on the Boundary enum to use its 
// contents.
impl<T> Deref for Boundary<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match *self {
            Boundary::Include(ref bound) => bound,
            Boundary::Exclude(ref bound) => bound
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Interval<T>
////////////////////////////////////////////////////////////////////////////////
///
/// A contiguous range of the type T, which may include or exclude either 
/// boundary.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Interval<T> where T: PartialOrd {
    /// The start of the range.
    pub start: Boundary<T>,
    /// The end of the range.
    pub end: Option<Boundary<T>>
}

impl <T> Interval<T> where T: PartialOrd {
    /// Creates a new interval from the given boundaries.
    ///
    /// # Examples
    /// ```rust
    /// use rampeditor::{Boundary, Interval};
    ///
    /// let l = Boundary::Include(12i32);
    /// let r = Boundary::Include(16i32);
    /// let int = Interval::new(l, Some(r));
    /// 
    /// assert_eq!(*int.start, 12);
    /// assert_eq!(*int.end.unwrap(), 16);
    ///
    /// // If the arguments are out of order, they will be swapped:
    /// let int2 = Interval::new(r, Some(l));
    /// 
    /// assert_eq!(*int.start, 12);
    /// assert_eq!(*int.end.unwrap(), 16);
    /// ```
    pub fn new(start: Boundary<T>, end: Option<Boundary<T>>) -> Self {
        if let Some(end_bound) = end {
            if *end_bound <= *start {
                Interval {start: end_bound, end: Some(start)}
            } else {
                Interval {start: start, end: Some(end_bound)}
            }
        } else {
            Interval {start: start, end: end}
        }
    }

    /// Creates a new open interval from the given values.
    ///
    /// # Example
    /// ```rust
    /// # use rampeditor::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(*int.start, 0);
    /// assert!(!int.start.is_inclusive());
    /// assert_eq!(*int.end.unwrap(), 2);
    /// assert!(!int.end.unwrap().is_inclusive());
    /// ```
    pub fn open(start: T, end: T) -> Self {
        Interval::new(
            Boundary::Exclude(start),
            Some(Boundary::Exclude(end))
        )
    }

    /// Creates a new closed interval from the given values.
    ///
    /// # Example
    /// ```rust
    /// # use rampeditor::Interval;
    ///
    /// let int = Interval::closed(0, 2);
    /// 
    /// assert_eq!(*int.start, 0);
    /// assert!(int.start.is_inclusive());
    /// assert_eq!(*int.end.unwrap(), 2);
    /// assert!(int.end.unwrap().is_inclusive());
    /// ```
    pub fn closed(start: T, end: T) -> Self {
        Interval::new(
            Boundary::Include(start),
            Some(Boundary::Include(end))
        )
    }

    /// Creates a new left-open interval from the given values.
    ///
    /// # Example
    /// ```rust
    /// # use rampeditor::Interval;
    ///
    /// let int = Interval::left_open(0, 2);
    /// 
    /// assert_eq!(*int.start, 0);
    /// assert!(!int.start.is_inclusive());
    /// assert_eq!(*int.end.unwrap(), 2);
    /// assert!(int.end.unwrap().is_inclusive());
    /// ```
    pub fn left_open(start: T, end: T) -> Self {
        Interval::new(
            Boundary::Exclude(start),
            Some(Boundary::Include(end))
        )
    }

    /// Creates a new right-open interval from the given values.
    ///
    /// # Example
    /// ```rust
    /// # use rampeditor::Interval;
    ///
    /// let int = Interval::right_open(0, 2);
    /// 
    /// assert_eq!(*int.start, 0);
    /// assert!(int.start.is_inclusive());
    /// assert_eq!(*int.end.unwrap(), 2);
    /// assert!(!int.end.unwrap().is_inclusive());
    /// ```
    pub fn right_open(start: T, end: T) -> Self {
        Interval::new(
            Boundary::Include(start),
            Some(Boundary::Exclude(end))
        )
    }

    pub fn is_empty(&self) -> bool {
        !if let Some(ref end_bound) = self.end {
            self.start.is_inclusive() ||
            end_bound.is_inclusive() ||
            *self.start != **end_bound
        } else {
            self.start.is_inclusive()
        }
    }
}

impl <'a, T> Interval<T> where T: PartialOrd + 'a, &'a T: Sub  {
    /// Returns the width of the interval.
    pub fn width(&'a self) -> <&'a T as Sub>::Output 
        where T: PartialOrd, <&'a T as Sub>::Output: Default 
    {
        if let Some(ref end_bound) = self.end {
            &**end_bound - &*self.start
        } else {
            Default::default()
        }
    }
}

// /// Trait for converting a position to an interval.
// pub trait ToInterval {
//     fn to(self: Self, end: Self) -> Interval<Self> where Self: Sized;

//     fn as_interval(self: Self) -> Interval<Self> where Self: Sized {
//         Interval {start: self, end: None}
//     }

//     fn enclosing<I>(
//         items: I) 
//         -> Option<Interval<Self>>
//         where I: Iterator<Item=Self>, Self: Sized + Ord + Clone + Copy
//     {
//         let mut lowest = None;
//         let mut greatest = None;
//         for item in items {
//             lowest = lowest.map_or(Some(item), |l| 
//                 if item.cmp(&l) == Ordering::Less {
//                     Some(item)
//                 } else {
//                     Some(l)
//                 }
//             );

//             greatest = greatest.map_or(Some(item), |g| 
//                 if item.cmp(&g) == Ordering::Greater {
//                     Some(item)
//                 } else {
//                     Some(g)
//                 }
//             );
//         }

//         match (lowest, greatest) {
//             (Some(l), Some(g)) => Some(l.to(g)),
//             (Some(l), None) => Some(l.as_interval()),
//             (None, Some(g)) => Some(g.as_interval()),
//             (None, None) => None
//         }
//     }
// }
