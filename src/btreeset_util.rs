// Adds two methods to `BTreeSet`:
//   - `next_value(v)`: returns the smallest value strictly larger than `v`, or `None`.
//   - `prev_value(v)`: returns the largest value strictly smaller than `v`, or `None`.

use std::collections::BTreeSet;


pub trait OrderedSetNeighborValues {
    type Item;
    fn next_value(&self, v: &Self::Item) -> Option<&Self::Item>;
    fn prev_value(&self, v: &Self::Item) -> Option<&Self::Item>;
}

impl<T: Ord> OrderedSetNeighborValues for BTreeSet<T> {
    type Item = T;

    fn prev_value(&self, v: &Self::Item) -> Option<&Self::Item> {
        use std::ops::Bound::*;
        self.range((Unbounded, Excluded(v))).next_back()
    }

    fn next_value(&self, v: &Self::Item) -> Option<&Self::Item> {
        use std::ops::Bound::*;
        self.range((Excluded(v), Unbounded)).next()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let s: BTreeSet<_> = [1, 3, 4, 6, 9].into_iter().collect();

        assert_eq!(s.prev_value(&0), None);
        assert_eq!(s.prev_value(&1), None);
        assert_eq!(s.prev_value(&2), Some(&1));
        assert_eq!(s.prev_value(&3), Some(&1));
        assert_eq!(s.prev_value(&5), Some(&4));
        assert_eq!(s.prev_value(&9), Some(&6));
        assert_eq!(s.prev_value(&10), Some(&9));

        assert_eq!(s.next_value(&0), Some(&1));
        assert_eq!(s.next_value(&1), Some(&3));
        assert_eq!(s.next_value(&4), Some(&6));
        assert_eq!(s.next_value(&5), Some(&6));
        assert_eq!(s.next_value(&8), Some(&9));
        assert_eq!(s.next_value(&9), None);
        assert_eq!(s.next_value(&10), None);
    }
}
