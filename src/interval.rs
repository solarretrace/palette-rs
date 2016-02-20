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
//! Provides a basic bounded interval type for doing complex set selections.
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
    /// The boundary includes the point.
    Include(T),
    /// The boundary excludes the point.
    Exclude(T),
}

impl<T> Boundary<T> {
    /// Returns whether the boundary includes its point.
    #[inline]
    pub fn is_inclusive(&self) -> bool {
        match self {
            &Boundary::Include(..) => true,
            &Boundary::Exclude(..) => false
        }
    }

    /// Returns whether the boundary excludes its point.
    #[inline]
    pub fn is_exclusive(&self) -> bool {
        !self.is_inclusive()
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
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Interval<T> where T: PartialOrd + Clone {
    /// The start of the range.
    start: Boundary<T>,
    /// The end of the range.
    end: Option<Boundary<T>>
}

impl <T> Interval<T> where T: PartialOrd + Clone  {
    /// Creates a new interval from the given boundaries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::{Boundary, Interval};
    ///
    /// let l = Boundary::Include(12);
    /// let r = Boundary::Include(16);
    /// let int = Interval::new(l, Some(r));
    /// 
    /// assert_eq!(int.left_point(), 12);
    /// assert_eq!(int.right_point(), 16);
    /// ```
    ///
    /// If the arguments are out of order, they will be swapped:
    ///
    /// ```rust
    /// # use rampeditor::{Boundary, Interval};
    /// let l = Boundary::Include(12);
    /// let r = Boundary::Include(16);
    /// let int = Interval::new(r, Some(l));
    /// 
    /// assert_eq!(int.left_point(), 12);
    /// assert_eq!(int.right_point(), 16);
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
    ///
    /// ```rust
    /// # use rampeditor::Interval;
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(!int.left_bound().is_inclusive());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(!int.right_bound().is_inclusive());
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
    ///
    /// ```rust
    /// # use rampeditor::Interval;
    /// let int = Interval::closed(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(int.left_bound().is_inclusive());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(int.right_bound().is_inclusive());
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
    ///
    /// ```rust
    /// # use rampeditor::Interval;
    /// let int = Interval::left_open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(!int.left_bound().is_inclusive());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(int.right_bound().is_inclusive());
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
    ///
    /// ```rust
    /// # use rampeditor::Interval;
    ///
    /// let int = Interval::right_open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(int.left_bound().is_inclusive());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(!int.right_bound().is_inclusive());
    /// ```
    pub fn right_open(start: T, end: T) -> Self {
        Interval::new(
            Boundary::Include(start),
            Some(Boundary::Exclude(end))
        )
    }

    /// Returns the leftmost (least) boundary point of the interval. Note that 
    /// this point may not be in the interval if the interval is left-open.
    #[inline]
    pub fn left_point(&self) -> T {
        (*self.start).clone()
    }

    /// Returns the rightmost (greatest) boundary point of the interval. Note 
    /// that this point may not be in the interval if the interval is 
    /// right-open.
    #[inline]
    pub fn right_point(&self) -> T {
        if let Some(ref end_bound) = self.end {
            (**end_bound).clone()
        } else {
            self.left_point()
        }
    }

    /// Returns the left (least) boundary of the interval.
    #[inline]
    pub fn left_bound(&self) -> Boundary<T> {
        self.start.clone()
    }

    /// Returns the right (greatest) boundary of the interval.
    #[inline]
    pub fn right_bound(&self) -> Boundary<T> {
        if let Some(ref end_bound) = self.end {
            end_bound.clone()
        } else {
            self.left_bound()
        }
    }

    /// Returns whether a given interval is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rampeditor::{Interval, Boundary};
    /// let int = Interval::right_open(0, 2);
    /// assert!(!int.is_empty());
    /// ```
    ///
    /// An open interval with two of the same points is empty:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Boundary};
    /// let int = Interval::open(0, 0);
    /// assert!(int.is_empty());
    /// ```
    ///
    /// A half-open interval with two of the same points is not:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Boundary};
    /// let int = Interval::left_open(0, 0);
    /// assert!(!int.is_empty());
    /// ```
    ///
    /// A single-point interval is empty only if that point is excluded:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Boundary};
    /// let int_a = Interval::new(Boundary::Exclude(0), None);
    /// let int_b = Interval::new(Boundary::Include(0), None);
    /// assert!(int_a.is_empty());
    /// assert!(!int_b.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.left_bound() == self.right_bound() 
            && self.left_bound().is_exclusive()
    }

    /// Returns whether the given point is included in the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::{Interval, Boundary};
    /// let int = Interval::right_open(0.0, 2.0);
    /// assert!(!int.contains(&-1.34));
    /// assert!(!int.contains(&-0.001));
    /// assert!(int.contains(&0.0));
    /// assert!(int.contains(&0.001));
    /// assert!(int.contains(&1.0));
    /// assert!(int.contains(&1.9999));
    /// assert!(!int.contains(&2.0));
    /// ```
    #[inline]
    pub fn contains(&self, point: &T) -> bool {
        *point > self.left_point() && *point < self.right_point()
            || *point == self.left_point() && self.left_bound().is_inclusive()
            || *point == self.right_point() && self.right_bound().is_inclusive()

        // if let Some(ref end_bound) = self.end {
        //     (*point >= *self.start && *point < **end_bound) || 
        //     (end_bound.is_inclusive() && *point == **end_bound)
        // } else {
        //     self.start.is_inclusive() && *point == *self.start
        // }
    }

    /// Returns the set union of the interval with the given interval. Note that
    /// since an interval requires contiguous points, a union of disjoint 
    /// intervals will fail to produce an interval and None will be returned.
    pub fn union(&self, other: &Self) -> Option<Self> {
        unimplemented!()
    }

    /// Returns the set intersection of the interval with the given interval,
    /// or None if the intervals do not overlap.
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        unimplemented!()
    }

    /// Returns the interval with all the points in the intersection with the 
    /// given interval removed.
    pub fn minus(&self, other: &Self) -> Option<Self> {
        unimplemented!()
    }

    /// Returns the smallest interval containing both of the given intervals.
    pub fn connect(&self, other: &Self) -> Option<Self> {
        unimplemented!()
    }

    /// Transforms a collection of intervals by combining any intervals that 
    /// overlap or touch and removing any that are empty.
    pub fn normalize(intervals: Vec<Self>) -> Vec<Self> {
        unimplemented!()
    }
}

impl <'a, T> Interval<T> where T: PartialOrd + Clone + 'a, &'a T: Sub  {
    /// Returns the width of the interval.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rampeditor::{Interval, Boundary};
    /// let int = Interval::open(0.0, 2.2);
    ///
    /// assert_eq!(int.width(), 2.2);
    /// ```
    ///
    /// If the interval is empty, a default value is returned:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Boundary};
    /// let int = Interval::open(0.0, 0.0);
    ///
    /// assert_eq!(int.width(), 0.0);
    /// ```
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
