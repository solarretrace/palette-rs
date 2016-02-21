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

use std::mem;

////////////////////////////////////////////////////////////////////////////////
// Bound
////////////////////////////////////////////////////////////////////////////////
///
/// Determines the type of an interval's boundary.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Bound<T> where T: PartialOrd + PartialEq + Clone {
    /// The boundary includes the point.
    Included(T),
    /// The boundary excludes the point.
    Excluded(T),
}

impl<T> Bound<T> where T: PartialOrd + PartialEq + Clone {
    /// Returns whether the boundary includes its point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(1);
    /// 
    /// assert!(b1.is_closed());
    /// assert!(!b2.is_closed());
    /// ```
    #[inline]
    pub fn is_closed(&self) -> bool {
        match self {
            &Bound::Included(..) => true,
            &Bound::Excluded(..) => false
        }
    }

    /// Returns whether the boundary excludes its point. 
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(1);
    /// 
    /// assert!(!b1.is_open());
    /// assert!(b2.is_open());
    /// ```
    #[inline]
    pub fn is_open(&self) -> bool {
        !self.is_closed()
    }

    /// Returns the intersect of the given boundaries, or the lowest one if they
    /// are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(0);
    /// 
    /// assert_eq!(b1.intersect_or_least(&b2), b2);
    /// ```
    pub fn intersect_or_least(&self, other: &Self) -> Self {
        if **self == **other {
            if self.is_closed() && other.is_closed() {
                self.clone()
            } else {
                Bound::Excluded((**self).clone())
            }
        } else if **self < **other {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Returns the intersect of the given boundaries, or the greatest one if 
    /// they are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(0);
    /// 
    /// assert_eq!(b1.intersect_or_greatest(&b2), b2);
    /// ```
    pub fn intersect_or_greatest(&self, other: &Self) -> Self {
        if **self == **other {
            if self.is_closed() && other.is_closed() {
                self.clone()
            } else {
                Bound::Excluded((**self).clone())
            }
        } else if **self > **other {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Returns the union of the given boundaries, or the lowest one if they are
    /// not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(0);
    /// 
    /// assert_eq!(b1.union_or_least(&b2), b1);
    /// ```
    pub fn union_or_least(&self, other: &Self) -> Self {
        if **self == **other {
            if self.is_open() && other.is_open() {
                self.clone()
            } else {
                Bound::Included((**self).clone())
            }
        } else if **self < **other {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Returns the union of the given boundaries, or the greatest one if they 
    /// are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(0);
    /// 
    /// assert_eq!(b1.union_or_greatest(&b2), b1);
    /// ```
    pub fn union_or_greatest(&self, other: &Self) -> Self {
        if **self == **other {
            if self.is_open() && other.is_open() {
                self.clone()
            } else {
                Bound::Included((**self).clone())
            }
        } else if **self > **other {
            self.clone()
        } else {
            other.clone()
        }
    }
}

// Implemented to prevent having to match on the Bound enum to use its 
// contents.
impl<T> Deref for Bound<T> where T: PartialOrd + PartialEq + Clone {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match *self {
            Bound::Included(ref bound) => bound,
            Bound::Excluded(ref bound) => bound
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
pub struct Interval<T> where T: PartialOrd + PartialEq + Clone {
    /// The start of the range.
    start: Bound<T>,
    /// The end of the range.
    end: Bound<T>
}

impl <T> Interval<T> where T: PartialOrd + PartialEq + Clone  {
    /// Creates a new interval from the given boundaries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::{Bound, Interval};
    ///
    /// let l = Bound::Included(12);
    /// let r = Bound::Included(16);
    /// let int = Interval::new(l, Some(r));
    /// 
    /// assert_eq!(int.left_point(), 12);
    /// assert_eq!(int.right_point(), 16);
    /// ```
    ///
    /// If the arguments are out of order, they will be swapped:
    ///
    /// ```rust
    /// use rampeditor::{Bound, Interval};
    ///
    /// let l = Bound::Included(12);
    /// let r = Bound::Included(16);
    /// let int = Interval::new(r, Some(l));
    /// 
    /// assert_eq!(int.left_point(), 12);
    /// assert_eq!(int.right_point(), 16);
    /// ```
    pub fn new(start: Bound<T>, end: Option<Bound<T>>) -> Self {
        if let Some(end_bound) = end {
            Interval {
                start: start.union_or_least(&end_bound), 
                end: start.union_or_greatest(&end_bound)
            }
        } else {
            Interval {start: start.clone(), end: start}
        }
    }

    /// Creates a new open interval from the given values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(!int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(!int.right_bound().is_closed());
    /// ```
    pub fn open(start: T, end: T) -> Self {
        Interval::new(
            Bound::Excluded(start),
            Some(Bound::Excluded(end))
        )
    }

    /// Creates a new closed interval from the given values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::closed(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(int.right_bound().is_closed());
    /// ```
    pub fn closed(start: T, end: T) -> Self {
        Interval::new(
            Bound::Included(start),
            Some(Bound::Included(end))
        )
    }

    /// Creates a new left-open interval from the given values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::left_open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(!int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(int.right_bound().is_closed());
    /// ```
    pub fn left_open(start: T, end: T) -> Self {
        Interval::new(
            Bound::Excluded(start),
            Some(Bound::Included(end))
        )
    }

    /// Creates a new right-open interval from the given values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::right_open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(!int.right_bound().is_closed());
    /// ```
    pub fn right_open(start: T, end: T) -> Self {
        Interval::new(
            Bound::Included(start),
            Some(Bound::Excluded(end))
        )
    }

    /// Returns the leftmost (least) boundary point of the interval. Note that 
    /// this point may not be in the interval if the interval is left-open.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// ```
    #[inline]
    pub fn left_point(&self) -> T {
        (*self.start).clone()
    }

    /// Returns the rightmost (greatest) boundary point of the interval. Note 
    /// that this point may not be in the interval if the interval is 
    /// right-open.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.right_point(), 2);
    /// ```
    #[inline]
    pub fn right_point(&self) -> T {
        (*self.end).clone()
    }

    /// Returns the left (least) boundary of the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::{Interval, Bound};
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.left_bound(), Bound::Excluded(0));
    /// ```
    #[inline]
    pub fn left_bound(&self) -> Bound<T> {
        self.start.clone()
    }

    /// Returns the right (greatest) boundary of the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::{Interval, Bound};
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.right_bound(), Bound::Excluded(2));
    /// ```
    #[inline]
    pub fn right_bound(&self) -> Bound<T> {
        self.end.clone()
    }

    /// Returns whether a given interval is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rampeditor::{Interval, Bound};
    /// let int = Interval::right_open(0, 2);
    /// assert!(!int.is_empty());
    /// ```
    ///
    /// An open interval with two of the same points is empty:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Bound};
    /// let int = Interval::open(0, 0);
    /// assert!(int.is_empty());
    /// ```
    ///
    /// A half-open interval with two of the same points is not:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Bound};
    /// let int = Interval::left_open(0, 0);
    /// assert!(!int.is_empty());
    /// ```
    ///
    /// A single-point interval is empty only if that point is excluded:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Bound};
    /// let int_a = Interval::new(Bound::Excluded(0), None);
    /// let int_b = Interval::new(Bound::Included(0), None);
    /// assert!(int_a.is_empty());
    /// assert!(!int_b.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.left_bound() == self.right_bound() 
            && self.left_bound().is_open()
    }

    /// Converts the interval into an Option, returning None if it is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// assert!(Interval::open(0, 0).into_non_empty().is_none());
    ///
    /// let int = Interval::open(0, 1);
    /// assert_eq!(int.into_non_empty().unwrap(), int);
    /// ```
    ///
    pub fn into_non_empty(self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    /// Returns whether the given point is included in the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    /// let int = Interval::right_open(0.0, 2.0);
    /// assert!(int.contains(&0.0));
    /// assert!(int.contains(&1.0));
    /// assert!(!int.contains(&2.0));
    /// ```
    #[inline]
    pub fn contains(&self, point: &T) -> bool {
        *point > self.left_point() && *point < self.right_point()
            || *point == self.left_point() && self.left_bound().is_closed()
            || *point == self.right_point() && self.right_bound().is_closed()
    }

    /// Returns the set intersection of the interval with the given interval,
    /// or None if the intervals do not overlap.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    /// let a = Interval::right_open(0.0, 2.0);
    /// let b = Interval::closed(1.0, 3.0);
    /// 
    /// assert_eq!(a.intersect(&b), Some(Interval::right_open(1.0, 2.0)));
    /// ```
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        // Check if either one is empty.
        if self.is_empty() || other.is_empty() {
            return None;
        }

        // Check if they're the same set.
        if self == other {
            return Some(self.clone());
        }

        // Choose orientation for intervals.
        let (a, b) = if self.left_point() <= other.left_point() {
            (self, other)
        } else {
            (other, self)
        };
        
        if a.right_point() < b.left_point() {
            // Not overlapping.
            None
        } else if a.right_point() == b.left_point() {
            // Overlapping at one point. 
            if a.right_bound().is_closed() && b.left_bound().is_closed() {
                // Both are closed.
                Some(Interval::new(
                    Bound::Included(a.right_point()), 
                    None
                ))
            } else {
                // At least one is open.
                None
            }
        } else {
            // Overlapping.
            Some(Interval::new(
                 a.left_bound().intersect_or_greatest(&b.left_bound()),
                 Some(a.right_bound().intersect_or_least(&b.right_bound()))
            ))
        }
    }

    /// Returns the set union of the interval with the given interval. Note that
    /// since an interval requires contiguous points, a union of disjoint 
    /// intervals will fail to produce an interval and None will be returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    /// let a = Interval::left_open(0.0, 2.0);
    /// let b = Interval::closed(1.0, 3.0);
    /// 
    /// assert_eq!(a.union(&b), Some(Interval::left_open(0.0, 3.0)));
    /// ```
    pub fn union(&self, other: &Self) -> Option<Self> {
        // Check for empty unions.
        if self.is_empty() && other.is_empty() {
            return None;
        } else if self.is_empty() {
            return Some(other.clone())
        } else if other.is_empty() {
            return Some(self.clone())
        }

        // Check if they're the same set.
        if self == other {
            return Some(self.clone());
        }

        // Choose orientation for intervals.
        let (a, b) = if self.left_point() <= other.left_point() {
            (self, other)
        } else {
            (other, self)
        };
        
        if a.right_point() < b.left_point() ||
            (a.right_point() == b.left_point() &&
            a.right_bound().is_open() && 
            b.left_bound().is_open())
        {
            // Not overlapping, or overlapping at one open point.
            None
        } else {
            // Overlapping.
            Some(Interval::new(
                 a.left_bound().union_or_least(&b.left_bound()),
                 Some(a.right_bound().union_or_greatest(&b.right_bound()))
            ))
        }
    }

    /// Returns the smallest interval containing both of the given intervals.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    /// let a = Interval::closed(0.0, 0.0);
    /// let b = Interval::open(2.0, 3.0);
    /// 
    /// assert_eq!(a.connect(&b), Some(Interval::right_open(0.0, 3.0)));
    /// ```
    pub fn connect(&self, other: &Self) -> Option<Self> {
        if self.is_empty() && other.is_empty() {
            None
        } else if self.is_empty() {
            Some(other.clone())
        } else if other.is_empty() {
            Some(self.clone())
        } else {
            Some(Interval::new(
                self.left_bound()
                    .union_or_least(&other.left_bound()),
                Some(self.right_bound()
                    .union_or_greatest(&other.right_bound())
                )
            ))
        }
    }

    /// Reduces a collection of intervals to a smaller set by removing redundant
    /// intervals through unions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    /// let ints = Interval::normalize(vec![
    ///     Interval::open(1.0, 2.0),
    ///     Interval::open(2.0, 3.0),
    ///     Interval::open(2.5, 3.5),
    ///     Interval::open(3.0, 3.0),
    ///     Interval::open(0.0, 1.5),
    /// ].into_iter());
    /// 
    /// assert_eq!(ints[0], Interval::open(0.0, 2.0));
    /// assert_eq!(ints[1], Interval::open(2.0, 3.5));
    /// ```
    pub fn normalize<I>(intervals: I) -> Vec<Interval<T>> 
        where I: IntoIterator<Item=Interval<T>>
    {   
        // Remove empty intervals.
        let mut it = intervals
            .into_iter()
            .filter(|int| !int.is_empty());

        // Get first interval.
        let start = it.next().unwrap();

        it.fold(vec![start], |mut prev, int| {
            let mut append = false;
            for item in prev.iter_mut() {
                if let Some(val) = item.union(&int) {
                    // Union with int succeeded.
                    mem::replace(item, val);
                } else {
                    // Union failed; append int to prev list.
                    append = true;
                }
            }
            if append {prev.push(int);}
            prev
        })
    }
}

impl <'a, T> Interval<T> 
    where 
        T: PartialOrd + PartialEq + Clone + 'a, 
        &'a T: Sub  
{
    /// Returns the width of the interval.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rampeditor::{Interval, Bound};
    /// let int = Interval::open(0.0, 2.2);
    ///
    /// assert_eq!(int.width(), 2.2);
    /// ```
    ///
    /// If the interval is empty, a default value is returned:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Bound};
    /// let int = Interval::open(0.0, 0.0);
    ///
    /// assert_eq!(int.width(), 0.0);
    /// ```
    pub fn width(&'a self) -> <&'a T as Sub>::Output 
        where <&'a T as Sub>::Output: Default 
    {
        &*self.end - &*self.start
    }
}



////////////////////////////////////////////////////////////////////////////////
// Test Module
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Interval;

    /// Tests the Interval constructors for points.
    #[test]
    fn interval_point_constructors() {
        let o: fn(f32, f32) -> Interval<f32> = Interval::open;
        let c: fn(f32, f32) -> Interval<f32> = Interval::closed;
        let lo: fn(f32, f32) -> Interval<f32> = Interval::left_open;
        let ro: fn(f32, f32) -> Interval<f32> = Interval::right_open;

        // Point constructors.
        assert!(o(0.5, 0.5).is_empty());
        assert_eq!(lo(0.5, 0.5), c(0.5, 0.5));
        assert_eq!(ro(0.5, 0.5), c(0.5, 0.5));
        assert_eq!(c(0.5, 0.5), c(0.5, 0.5));
    }

    /// Tests the Interval::intersect function.
    #[test]
    fn interval_contains() {
        let int = Interval::right_open(0.0, 2.0);
        assert!(!int.contains(&-1.34));
        assert!(!int.contains(&-0.001));
        assert!(int.contains(&0.0));
        assert!(int.contains(&0.001));
        assert!(int.contains(&1.0));
        assert!(int.contains(&1.9999));
        assert!(!int.contains(&2.0));
    }

    /// Tests the Interval::intersect function.
    #[test]
    fn interval_intersect() {
        let o: fn(f32, f32) -> Interval<f32> = Interval::open;
        let c: fn(f32, f32) -> Interval<f32> = Interval::closed;
        let lo: fn(f32, f32) -> Interval<f32> = Interval::left_open;
        let ro: fn(f32, f32) -> Interval<f32> = Interval::right_open;

        // Open overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(1.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(&lo(1.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(&ro(1.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(& c(1.0, 2.0)), Some( o(1.0, 2.0)));

        // Closed overlapping.
        assert_eq!( c(1.0, 2.0).intersect(& o(1.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(&lo(1.0, 2.0)), Some(lo(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(&ro(1.0, 2.0)), Some(ro(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(& c(1.0, 2.0)), Some( c(1.0, 2.0)));
        
        // Open left-half overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(1.0, 1.5)), Some( o(1.0, 1.5)));
        assert_eq!( o(1.0, 2.0).intersect(&lo(1.0, 1.5)), Some(lo(1.0, 1.5)));
        assert_eq!( o(1.0, 2.0).intersect(&ro(1.0, 1.5)), Some( o(1.0, 1.5)));
        assert_eq!( o(1.0, 2.0).intersect(& c(1.0, 1.5)), Some(lo(1.0, 1.5)));

        // Close left-half overlapping.
        assert_eq!( c(1.0, 2.0).intersect(& o(1.0, 1.5)), Some( o(1.0, 1.5)));
        assert_eq!( c(1.0, 2.0).intersect(&lo(1.0, 1.5)), Some(lo(1.0, 1.5)));
        assert_eq!( c(1.0, 2.0).intersect(&ro(1.0, 1.5)), Some(ro(1.0, 1.5)));
        assert_eq!( c(1.0, 2.0).intersect(& c(1.0, 1.5)), Some( c(1.0, 1.5)));

        // Open right-half overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(1.5, 2.0)), Some( o(1.5, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(&lo(1.5, 2.0)), Some( o(1.5, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(&ro(1.5, 2.0)), Some(ro(1.5, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(& c(1.5, 2.0)), Some(ro(1.5, 2.0)));

        // Closed right-half overlapping.
        assert_eq!( c(1.0, 2.0).intersect(& o(1.5, 2.0)), Some( o(1.5, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(&lo(1.5, 2.0)), Some(lo(1.5, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(&ro(1.5, 2.0)), Some(ro(1.5, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(& c(1.5, 2.0)), Some( c(1.5, 2.0)));

        // Open Subset overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(1.2, 1.8)), Some( o(1.2, 1.8)));
        assert_eq!( o(1.0, 2.0).intersect(&lo(1.2, 1.8)), Some(lo(1.2, 1.8)));
        assert_eq!( o(1.0, 2.0).intersect(&ro(1.2, 1.8)), Some(ro(1.2, 1.8)));
        assert_eq!( o(1.0, 2.0).intersect(& c(1.2, 1.8)), Some( c(1.2, 1.8)));

        // Closed Subset overlapping.
        assert_eq!( c(1.0, 2.0).intersect(& o(1.2, 1.8)), Some( o(1.2, 1.8)));
        assert_eq!( c(1.0, 2.0).intersect(&lo(1.2, 1.8)), Some(lo(1.2, 1.8)));
        assert_eq!( c(1.0, 2.0).intersect(&ro(1.2, 1.8)), Some(ro(1.2, 1.8)));
        assert_eq!( c(1.0, 2.0).intersect(& c(1.2, 1.8)), Some( c(1.2, 1.8)));

        // Right non-overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(2.0, 3.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(&lo(2.0, 3.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(&ro(2.0, 3.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(& c(2.0, 3.0)), None);

        // Left non-overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(0.0, 1.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(&lo(0.0, 1.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(&ro(0.0, 1.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(& c(0.0, 1.0)), None);

        // Center Point intersections.
        assert_eq!( o(1.0, 2.0).intersect(& o(1.5, 1.5)), None);
        assert_eq!( o(1.0, 2.0).intersect(&lo(1.5, 1.5)), Some( c(1.5, 1.5)));
        assert_eq!( o(1.0, 2.0).intersect(&ro(1.5, 1.5)), Some( c(1.5, 1.5)));
        assert_eq!( o(1.0, 2.0).intersect(& c(1.5, 1.5)), Some( c(1.5, 1.5)));

        // Left Point intersections.
        assert_eq!( c(1.0, 2.0).intersect(& o(1.0, 1.0)), None);
        assert_eq!( c(1.0, 2.0).intersect(&lo(1.0, 1.0)), Some( c(1.0, 1.0)));
        assert_eq!( c(1.0, 2.0).intersect(&ro(1.0, 1.0)), Some( c(1.0, 1.0)));
        assert_eq!( c(1.0, 2.0).intersect(& c(1.0, 1.0)), Some( c(1.0, 1.0)));

        // Left Point intersections.
        assert_eq!( c(1.0, 2.0).intersect(& o(2.0, 2.0)), None);
        assert_eq!( c(1.0, 2.0).intersect(&lo(2.0, 2.0)), Some( c(2.0, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(&ro(2.0, 2.0)), Some( c(2.0, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(& c(2.0, 2.0)), Some( c(2.0, 2.0)));
    }


    /// Tests the Interval::union function.
    #[test]
    fn interval_union() {
        let o: fn(f32, f32) -> Interval<f32> = Interval::open;
        let c: fn(f32, f32) -> Interval<f32> = Interval::closed;
        let lo: fn(f32, f32) -> Interval<f32> = Interval::left_open;
        let ro: fn(f32, f32) -> Interval<f32> = Interval::right_open;

        // Open overlapping.
        assert_eq!( o(1.0, 2.0).union(& o(1.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&lo(1.0, 2.0)), Some(lo(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&ro(1.0, 2.0)), Some(ro(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(& c(1.0, 2.0)), Some( c(1.0, 2.0)));

        // Closed overlapping.
        assert_eq!( c(1.0, 2.0).union(& o(1.0, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(&lo(1.0, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(&ro(1.0, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(& c(1.0, 2.0)), Some( c(1.0, 2.0)));
        
        // Open left-half overlapping.
        assert_eq!( o(1.0, 2.0).union(& o(1.0, 1.5)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&lo(1.0, 1.5)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&ro(1.0, 1.5)), Some(ro(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(& c(1.0, 1.5)), Some(ro(1.0, 2.0)));

        // Close left-half overlapping.
        assert_eq!( c(1.0, 2.0).union(& o(1.0, 1.5)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(&lo(1.0, 1.5)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(&ro(1.0, 1.5)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(& c(1.0, 1.5)), Some( c(1.0, 2.0)));

        // Open right-half overlapping.
        assert_eq!( o(1.0, 2.0).union(& o(1.5, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&lo(1.5, 2.0)), Some(lo(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&ro(1.5, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(& c(1.5, 2.0)), Some(lo(1.0, 2.0)));

        // Closed right-half overlapping.
        assert_eq!( c(1.0, 2.0).union(& o(1.5, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(&lo(1.5, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(&ro(1.5, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(& c(1.5, 2.0)), Some( c(1.0, 2.0)));

        // Open Subset overlapping.
        assert_eq!( o(1.0, 2.0).union(& o(1.2, 1.8)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&lo(1.2, 1.8)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&ro(1.2, 1.8)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(& c(1.2, 1.8)), Some( o(1.0, 2.0)));

        // Closed Subset overlapping.
        assert_eq!( c(1.0, 2.0).union(& o(1.2, 1.8)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(&lo(1.2, 1.8)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(&ro(1.2, 1.8)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).union(& c(1.2, 1.8)), Some( c(1.0, 2.0)));

        // Right non-overlapping.
        assert_eq!( o(1.0, 2.0).union(& o(2.0, 3.0)), None);
        assert_eq!( o(1.0, 2.0).union(&lo(2.0, 3.0)), None);
        assert_eq!( o(1.0, 2.0).union(&ro(2.0, 3.0)), Some( o(1.0, 3.0)));
        assert_eq!( o(1.0, 2.0).union(& c(2.0, 3.0)), Some(lo(1.0, 3.0)));

        // Left non-overlapping.
        assert_eq!( o(1.0, 2.0).union(& o(0.0, 1.0)), None);
        assert_eq!( o(1.0, 2.0).union(&lo(0.0, 1.0)), Some( o(0.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&ro(0.0, 1.0)), None);
        assert_eq!( o(1.0, 2.0).union(& c(0.0, 1.0)), Some(ro(0.0, 2.0)));

        // Center Point unions.
        assert_eq!( o(1.0, 2.0).union(& o(1.5, 1.5)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&lo(1.5, 1.5)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&ro(1.5, 1.5)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(& c(1.5, 1.5)), Some( o(1.0, 2.0)));

        // Left Point unions.
        assert_eq!( o(1.0, 2.0).union(& o(1.0, 1.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&lo(1.0, 1.0)), Some(ro(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&ro(1.0, 1.0)), Some(ro(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(& c(1.0, 1.0)), Some(ro(1.0, 2.0)));

        // Left Point unions.
        assert_eq!( o(1.0, 2.0).union(& o(2.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&lo(2.0, 2.0)), Some(lo(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(&ro(2.0, 2.0)), Some(lo(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).union(& c(2.0, 2.0)), Some(lo(1.0, 2.0)));
    }

    /// Tests the Interval::connect function.
    #[test]
    fn interval_connect() {
        let o: fn(f32, f32) -> Interval<f32> = Interval::open;
        let c: fn(f32, f32) -> Interval<f32> = Interval::closed;
        let lo: fn(f32, f32) -> Interval<f32> = Interval::left_open;
        let ro: fn(f32, f32) -> Interval<f32> = Interval::right_open;

        // Open overlapping.
        assert_eq!( o(1.0, 2.0).connect(& o(1.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&lo(1.0, 2.0)), Some(lo(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&ro(1.0, 2.0)), Some(ro(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(& c(1.0, 2.0)), Some( c(1.0, 2.0)));

        // Closed overlapping.
        assert_eq!( c(1.0, 2.0).connect(& o(1.0, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(&lo(1.0, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(&ro(1.0, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(& c(1.0, 2.0)), Some( c(1.0, 2.0)));
        
        // Open left-half overlapping.
        assert_eq!( o(1.0, 2.0).connect(& o(1.0, 1.5)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&lo(1.0, 1.5)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&ro(1.0, 1.5)), Some(ro(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(& c(1.0, 1.5)), Some(ro(1.0, 2.0)));

        // Close left-half overlapping.
        assert_eq!( c(1.0, 2.0).connect(& o(1.0, 1.5)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(&lo(1.0, 1.5)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(&ro(1.0, 1.5)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(& c(1.0, 1.5)), Some( c(1.0, 2.0)));

        // Open right-half overlapping.
        assert_eq!( o(1.0, 2.0).connect(& o(1.5, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&lo(1.5, 2.0)), Some(lo(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&ro(1.5, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(& c(1.5, 2.0)), Some(lo(1.0, 2.0)));

        // Closed right-half overlapping.
        assert_eq!( c(1.0, 2.0).connect(& o(1.5, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(&lo(1.5, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(&ro(1.5, 2.0)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(& c(1.5, 2.0)), Some( c(1.0, 2.0)));

        // Open Subset overlapping.
        assert_eq!( o(1.0, 2.0).connect(& o(1.2, 1.8)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&lo(1.2, 1.8)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&ro(1.2, 1.8)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(& c(1.2, 1.8)), Some( o(1.0, 2.0)));

        // Closed Subset overlapping.
        assert_eq!( c(1.0, 2.0).connect(& o(1.2, 1.8)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(&lo(1.2, 1.8)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(&ro(1.2, 1.8)), Some( c(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).connect(& c(1.2, 1.8)), Some( c(1.0, 2.0)));

        // Right non-overlapping.
        assert_eq!( o(1.0, 2.0).connect(& o(2.0, 3.0)), Some( o(1.0, 3.0)));
        assert_eq!( o(1.0, 2.0).connect(&lo(2.0, 3.0)), Some(lo(1.0, 3.0)));
        assert_eq!( o(1.0, 2.0).connect(&ro(2.0, 3.0)), Some( o(1.0, 3.0)));
        assert_eq!( o(1.0, 2.0).connect(& c(2.0, 3.0)), Some(lo(1.0, 3.0)));

        // Left non-overlapping.
        assert_eq!( o(1.0, 2.0).connect(& o(0.0, 1.0)), Some( o(0.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&lo(0.0, 1.0)), Some( o(0.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&ro(0.0, 1.0)), Some(ro(0.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(& c(0.0, 1.0)), Some(ro(0.0, 2.0)));

        // Center Point connects.
        assert_eq!( o(1.0, 2.0).connect(& o(1.5, 1.5)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&lo(1.5, 1.5)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&ro(1.5, 1.5)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(& c(1.5, 1.5)), Some( o(1.0, 2.0)));

        // Left Point connects.
        assert_eq!( o(1.0, 2.0).connect(& o(1.0, 1.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&lo(1.0, 1.0)), Some(ro(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&ro(1.0, 1.0)), Some(ro(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(& c(1.0, 1.0)), Some(ro(1.0, 2.0)));

        // Left Point connects.
        assert_eq!( o(1.0, 2.0).connect(& o(2.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&lo(2.0, 2.0)), Some(lo(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(&ro(2.0, 2.0)), Some(lo(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).connect(& c(2.0, 2.0)), Some(lo(1.0, 2.0)));
    }
}
