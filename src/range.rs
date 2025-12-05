//! An implementation of a multirange for AoC.

use std::iter::Sum;
use std::mem;
use std::ops::{Add, Range, Sub};

use merging_iterator::MergeIter;

/// A MultiRange represents a set of a totally ordered type T, where inclusion in the set
/// is defined by being within certain bounds. For example, `MultiRange::from(0..2) + MultiRange::from(5..8)`
/// represents the integer set {0, 1, 5, 6, 7}.
/// At this time, all ranges must be bounded and half-open (ie. they must be a std::ops::Range).
// We can safely derive PartialEq and Hash because the invariant ensures all equivalent values
// have the same representation.
#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MultiRange<T> {
    // Invariant: ranges are non-overlapping and sorted by start point
    ranges: [Range<T>],
}

impl<T> MultiRange<T> {
    // Invariant: Slice must contain non-overlapping ranges sorted by start point
    const fn from_slice(slice: &[Range<T>]) -> &MultiRange<T> {
        // SAFETY: MultiRange<T> is repr(transparent) and contains only a [Range<T>]
        // so &MultiRange<T> is guarenteed to have the same layout as &[Range<T>]
        // Invariant: Upheld by caller.
        unsafe { mem::transmute(slice) }
    }

    // Invariant: Slice must contain non-overlapping ranges sorted by start point
    const fn from_boxed_slice(slice: Box<[Range<T>]>) -> Box<MultiRange<T>> {
        // SAFETY: MultiRange<T> is repr(transparent) and contains only a [Range<T>]
        // so Box<MultiRange<T>> is guarenteed to have the same layout as Box<[Range<T>]>
        // Invariant: Upheld by caller.
        unsafe { mem::transmute(slice) }
    }

    const fn into_boxed_slice(self: Box<Self>) -> Box<[Range<T>]> {
        // SAFETY: MultiRange<T> is repr(transparent) and contains only a [Range<T>]
        // so Box<MultiRange<T>> is guarenteed to have the same layout as Box<[Range<T>]>
        unsafe { mem::transmute(self) }
    }

    /// Returns a new empty boxed MultiRange
    pub fn empty_owned() -> Box<MultiRange<T>> {
        let boxed_slice = Box::new([]);
        MultiRange::from_boxed_slice(boxed_slice)
    }

    /// Returns true if the MultiRange is empty
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
}

impl<'a, T: 'a> MultiRange<T> {
    /// A reference to an empty MultiRange<T>
    pub const EMPTY: &'a Self = MultiRange::from_slice(&[]);
}

impl<T: Ord> MultiRange<T> {
    /// Returns whether a given value is within the multirange
    pub fn contains(&self, value: T) -> bool {
        for range in self {
            if range.end <= value {
                // We're past where the value should be
                return false;
            }
            if range.start <= value {
                // Start is before us but end is after us, we're found
                return true;
            }
        }
        // We're past the end of the last range
        false
    }
}

impl<T: Clone> ToOwned for MultiRange<T> {
    type Owned = Box<MultiRange<T>>;

    fn to_owned(&self) -> Self::Owned {
        let boxed_slice: Box<[Range<T>]> = self.ranges.to_vec().into_boxed_slice();
        MultiRange::from_boxed_slice(boxed_slice)
    }
}

impl<T, A> MultiRange<T>
where
    T: Ord + Copy + Sub<T, Output = A>,
    A: std::iter::Sum,
{
    /// Returns the total length of all component ranges.
    pub fn size(&self) -> A {
        self.ranges.iter().map(|r| r.end - r.start).sum()
    }
}

impl<'a, T> From<&'a Range<T>> for &'a MultiRange<T> {
    /// Create a MultiRange that is equivalent to a single Range.
    fn from(range: &'a Range<T>) -> &'a MultiRange<T> {
        let ranges = std::slice::from_ref(range);
        // Invariant check: A single-element list cannot contain overlaps and is always sorted.
        MultiRange::from_slice(ranges)
    }
}

// &MultiRange + &MultiRange
impl<T: Ord + Clone> Add for &MultiRange<T> {
    type Output = Box<MultiRange<T>>;

    // TODO: Can I remove this dependency?
    fn add(self, other: Self) -> Self::Output {
        let mut iter = MergeIter::with_custom_ordering(self, other, |a, b| b.start < a.start);
        let mut current = if let Some(range) = iter.next() {
            range.clone()
        } else {
            return MultiRange::empty_owned();
        };
        let mut result = Vec::new();
        for next in iter {
            if current.end < next.start {
                // Current is entirely before next, so there's a gap.
                // Emit current and set next as new current.
                result.push(current);
                current = next.clone();
            } else if current.end < next.end {
                // Next is partially overlapping current.
                // Extend current to end of next.
                current.end = next.end.clone();
            }
        }
        // Emit the final current range
        result.push(current);
        MultiRange::from_boxed_slice(result.into_boxed_slice())
    }
}

// &MultiRange + Range
impl<T: Ord + Clone> Add<Range<T>> for &MultiRange<T> {
    type Output = Box<MultiRange<T>>;

    fn add(self, other: Range<T>) -> Self::Output {
        let as_multirange: &MultiRange<T> = (&other).into();
        self + as_multirange
    }
}

impl<T: Ord + Clone, V> Sum<V> for Box<MultiRange<T>>
where
    for<'a> &'a MultiRange<T>: Add<V, Output = Box<MultiRange<T>>>,
{
    fn sum<I: Iterator<Item = V>>(iter: I) -> Self {
        let mut result: Self = MultiRange::empty_owned();
        for value in iter {
            result = &*result + value;
        }
        result
    }
}

impl<T: Ord + Clone> FromIterator<Range<T>> for Box<MultiRange<T>> {
    /// Create a MultiRange that contains the union of all ranges in the given iterator.
    fn from_iter<I: IntoIterator<Item = Range<T>>>(iter: I) -> Self {
        iter.into_iter().sum()
    }
}

impl<T: Ord> IntoIterator for Box<MultiRange<T>> {
    type Item = Range<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.into_boxed_slice().into_vec().into_iter()
    }
}

impl<'a, T: Ord> IntoIterator for &'a MultiRange<T> {
    type Item = &'a Range<T>;
    type IntoIter = std::slice::Iter<'a, Range<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.ranges.iter()
    }
}

#[cfg(test)]
/// TODO: Test this dang thing.
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
