// CountingSet is like MultiSet, but assumes that all elements which compare equal are identical and
// stores only one copy plus a count. Supports operations on individual element level and on
// equality group level. It is guaranteed that all counts are always non-zero.

use std::collections::{BTreeMap, btree_map};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CountingSet<T: Ord + Clone> {
    map: BTreeMap<T, usize>,
}

impl<T: Ord + Clone> CountingSet<T> {
    pub fn new() -> Self {
        CountingSet {
            map: BTreeMap::new(),
        }
    }

    pub fn from_items_iter(iter: impl IntoIterator<Item = T>) -> Self {
        let mut s = Self::new();
        for x in iter {
            s.push(x);
        }
        s
    }
    pub fn from_group_iter(iter: impl IntoIterator<Item = (T, usize)>) -> Self {
        // Note. Could've done `BTreeMap::from_iter(iter.into_iter().filter(|(_, n)| *n > 0))`,
        // but that would be wrong if there are duplicates.
        let mut s = Self::new();
        for (x, n) in iter {
            s.push_multiple(x, n);
        }
        s
    }

    pub fn is_empty(&self) -> bool { self.map.is_empty() }
    pub fn len_slow(&self) -> usize { self.map.values().sum() }
    pub fn num_groups(&self) -> usize { self.map.len() }

    pub fn contains(&self, x: &T) -> bool { self.map.contains_key(x) }
    pub fn count(&self, x: &T) -> usize { *self.map.get(x).unwrap_or(&0) }
    pub fn first(&self) -> Option<&T> { self.first_group().map(|(k, _)| k) }
    pub fn last(&self) -> Option<&T> { self.last_group().map(|(k, _)| k) }
    pub fn first_group(&self) -> Option<(&T, usize)> { self.map.first_key_value().map(|(k, n)| (k, *n)) }
    pub fn last_group(&self) -> Option<(&T, usize)> { self.map.last_key_value().map(|(k, n)| (k, *n)) }

    pub fn is_subset(&self, other: &Self) -> bool { self.map.iter().all(|(x, n)| *n <= other.count(x)) }
    pub fn is_superset(&self, other: &Self) -> bool { other.is_subset(self) }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut s = Self::new();
        for (x, n) in self.group_iter() {
            s.push_multiple(x.clone(), n.min(other.count(x)));
        }
        s
    }
    pub fn union(&self, other: &Self) -> Self {
        let mut s = self.clone();
        for (x, n) in other.group_iter() {
            s.map.entry(x.clone()).and_modify(|m| *m = (*m).max(n)).or_insert(n);
        }
        s
    }

    pub fn item_iter(&self) -> impl Iterator<Item = &T> {
        self.group_iter().flat_map(|(x, n)| std::iter::repeat(x).take(n))
    }
    pub fn group_iter(&self) -> impl ExactSizeIterator<Item = (&T, usize)> {
        self.map.iter().map(|(x, n)| (x, *n))
    }

    pub fn push(&mut self, x: T) {
        *self.map.entry(x).or_insert(0) += 1;
    }
    pub fn push_multiple(&mut self, x: T, n: usize) {
        if n > 0 {
            *self.map.entry(x).or_insert(0) += n;
        }
    }

    pub fn remove(&mut self, x: T) -> bool {
        self.remove_up_to(x, 1) > 0
    }
    pub fn remove_up_to(&mut self, x: T, n: usize) -> usize {
        match self.map.entry(x) {
            btree_map::Entry::Occupied(mut e) => {
                let count = *e.get();
                if count <= n {
                    e.remove_entry();
                    count
                } else {
                    *e.get_mut() -= n;
                    n
                }
            }
            btree_map::Entry::Vacant(_) => 0,
        }
    }
    pub fn remove_exact(&mut self, x: T, n: usize) -> bool {
        match self.map.entry(x) {
            btree_map::Entry::Occupied(mut e) => {
                let count = *e.get();
                if count > n {
                    *e.get_mut() -= n;
                    true
                } else if count == n {
                    e.remove_entry();
                    true
                } else {
                    false
                }
            }
            btree_map::Entry::Vacant(_) => false,
        }
    }
    pub fn remove_group(&mut self, x: &T) -> usize {
        self.map.remove(x).unwrap_or(0)
    }

    pub fn pop_first(&mut self) -> Option<T> {
        let mut e = self.map.first_entry()?;
        *e.get_mut() -= 1;
        if *e.get() == 0 {
            Some(e.remove_entry().0)
        } else {
            Some(e.key().clone())
        }
    }
    pub fn pop_last(&mut self) -> Option<T> {
        let mut e = self.map.last_entry()?;
        *e.get_mut() -= 1;
        if *e.get() == 0 {
            Some(e.remove_entry().0)
        } else {
            Some(e.key().clone())
        }
    }
    pub fn pop_first_group(&mut self) -> Option<(T, usize)> { self.map.pop_first() }
    pub fn pop_last_group(&mut self) -> Option<(T, usize)> { self.map.pop_last() }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    // Note. No mutable BTreeMap getter to ensure that there are no zero counts.
    pub fn btree_map(&self) -> &BTreeMap<T, usize> { &self.map }
    pub fn into_btree_map(self) -> BTreeMap<T, usize> { self.map }
    pub fn from_btree_map(map: BTreeMap<T, usize>) -> Self {
        CountingSet {
            map: BTreeMap::from_iter(map.into_iter().filter(|(_, n)| *n > 0)),
        }
    }
}

impl<T: Ord + Clone> FromIterator<T> for CountingSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut s = Self::new();
        for x in iter {
            s.push(x);
        }
        s
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut s = CountingSet::new();
        assert!(s.is_empty());
        assert_eq!(s.len_slow(), 0);
        assert_eq!(s.item_iter().next(), None);
        assert_eq!(s.group_iter().next(), None);
        assert_eq!(s.first(), None);
        assert_eq!(s.last(), None);
        assert!(!s.contains(&"b"));
        assert_eq!(s.count(&"b"), 0);
        assert!(!s.remove("b"));
        assert_eq!(s.pop_first(), None);
        assert_eq!(s.pop_last(), None);

        s.push("b");
        s.push("b");
        s.push("a");
        s.push("c");
        s.push("c");
        s.push("c");
        assert!(!s.is_empty());
        assert_eq!(s.len_slow(), 6);
        assert_eq!(s.item_iter().collect::<Vec<_>>(), vec![&"a", &"b", &"b", &"c", &"c", &"c"]);
        assert_eq!(s.group_iter().collect::<Vec<_>>(), vec![(&"a", 1), (&"b", 2), (&"c", 3)]);
        assert_eq!(s.first(), Some(&"a"));
        assert_eq!(s.last(), Some(&"c"));
        assert!(s.contains(&"b"));
        assert_eq!(s.count(&"b"), 2);
        assert!(s.remove("b"));
        assert_eq!(s.pop_first(), Some("a"));
        assert_eq!(s.pop_last(), Some("c"));
        assert_eq!(s.item_iter().collect::<Vec<_>>(), vec![&"b", &"c", &"c"]);
    }

    #[test]
    fn removal() {
        let mut s = CountingSet::new();
        s.push_multiple("foo", 3);
        {
            let mut s = s.clone();
            assert!(s.remove("foo"));
            assert!(s.remove("foo"));
            assert!(s.remove("foo"));
            assert!(!s.remove("foo"));
            assert!(!s.remove("foo"));
        }
        {
            let mut s = s.clone();
            assert_eq!(s.remove_group(&"foo"), 3);
            assert!(s.is_empty());
        }
        {
            let mut s = s.clone();
            assert!(s.remove_exact("foo", 2));
            assert_eq!(s.len_slow(), 1);
        }
        {
            let mut s = s.clone();
            assert!(s.remove_exact("foo", 3));
            assert_eq!(s.len_slow(), 0);
        }
        {
            let mut s = s.clone();
            assert!(!s.remove_exact("foo", 4));
            assert_eq!(s.len_slow(), 3);
        }
        {
            let mut s = s.clone();
            assert_eq!(s.remove_up_to("foo", 2), 2);
            assert_eq!(s.len_slow(), 1);
        }
        {
            let mut s = s.clone();
            assert_eq!(s.remove_up_to("foo", 3), 3);
            assert_eq!(s.len_slow(), 0);
        }
        {
            let mut s = s.clone();
            assert_eq!(s.remove_up_to("foo", 4), 3);
            assert_eq!(s.len_slow(), 0);
        }
    }

    #[test]
    fn set_operations() {
        let a = CountingSet::from_items_iter(["a", "b", "b", "c", "c", "c"]);
        assert!(a.is_subset(&a));
        assert!(a.is_superset(&a));
        assert_eq!(a.union(&a), a);
        assert_eq!(a.intersection(&a), a);

        let b = CountingSet::from_items_iter(["b", "c", "c"]);
        assert!(b.is_subset(&a));
        assert!(a.is_superset(&b));
        assert_eq!(a.union(&b), a);
        assert_eq!(a.intersection(&b), b);

        let c = CountingSet::from_items_iter(["b", "b", "b", "c", "d", "d"]);
        assert!(!c.is_subset(&a));
        assert!(!a.is_subset(&c));
        assert_eq!(a.union(&c),
            CountingSet::from_items_iter(["a", "b", "b", "b", "c", "c", "c", "d", "d"]));
        assert_eq!(a.intersection(&c), CountingSet::from_items_iter(["b", "b", "c"]));
    }
}
