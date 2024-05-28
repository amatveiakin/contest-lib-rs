// Bidirectional hash map.
//
// Optimized for small elements, ideally copiable. If large elements are needed, it would be best to
// wrap everything in `Rc`s in order to avoid cloning (like the `bimap` crate does).

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;


#[derive(Clone, Debug)]
pub struct BiHashMap<A: Clone + Eq + Hash, B: Clone + Eq + Hash> {
    ltr: HashMap<A, B>,
    rtl: HashMap<B, A>,
}

impl<A: Clone + Eq + Hash, B: Clone + Eq + Hash> BiHashMap<A, B> {
    pub fn new() -> Self {
        BiHashMap {
            ltr: HashMap::new(),
            rtl: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool { self.ltr.is_empty() }
    pub fn len(&self) -> usize { self.ltr.len() }

    pub fn contains_left(&self, a: &A) -> bool { self.ltr.contains_key(a) }
    pub fn contains_right(&self, b: &B) -> bool { self.rtl.contains_key(b) }

    pub fn get_by_left(&self, a: &A) -> Option<&B> { self.ltr.get(a) }
    pub fn get_by_right(&self, b: &B) -> Option<&A> { self.rtl.get(b) }

    pub fn try_insert(&mut self, a: A, b: B) -> Result<(), BiHashMapInsertionError> {
        let Entry::Vacant(fwd_entry) = self.ltr.entry(a.clone()) else {
            return Err(BiHashMapInsertionError::DuplicateLeft);
        };
        let Entry::Vacant(bwd_entry) = self.rtl.entry(b.clone()) else {
            return Err(BiHashMapInsertionError::DuplicateRight);
        };
        fwd_entry.insert(b);
        bwd_entry.insert(a);
        Ok(())
    }

    pub fn remove_by_left(&mut self, a: &A) -> Option<B> {
        let b = self.ltr.remove(a);
        if let Some(b) = &b {
            assert!(self.rtl.remove(b).is_some());
        }
        b
    }
    pub fn remove_by_right(&mut self, b: &B) -> Option<A> {
        let a = self.rtl.remove(b);
        if let Some(a) = &a {
            assert!(self.ltr.remove(a).is_some());
        }
        a
    }

    pub fn clear(&mut self) {
        self.ltr.clear();
        self.rtl.clear();
    }

    pub fn left_to_right_map(&self) -> &HashMap<A, B> { &self.ltr }
    pub fn right_to_left_map(&self) -> &HashMap<B, A> { &self.rtl }
    pub fn iter(&self) -> impl Iterator<Item = (&A, &B)> { self.ltr.iter() }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BiHashMapInsertionError {
    DuplicateLeft,
    DuplicateRight,
}


#[cfg(test)]
mod tests {
    use std::vec;

    use crate::iterutils_basic::IterutilsBasic;

    use super::*;

    #[test]
    fn test_bihashmap() {
        let mut m = BiHashMap::new();
        assert!(m.is_empty());
        assert_eq!(m.len(), 0);
        assert!(!m.contains_left(&1));
        assert!(!m.contains_right(&"1"));
        assert_eq!(m.get_by_left(&1), None);
        assert_eq!(m.get_by_right(&"1"), None);

        assert_eq!(m.try_insert(1, "1"), Ok(()));
        assert_eq!(m.try_insert(1, "2"), Err(BiHashMapInsertionError::DuplicateLeft));
        assert_eq!(m.try_insert(2, "1"), Err(BiHashMapInsertionError::DuplicateRight));

        assert!(!m.is_empty());
        assert_eq!(m.len(), 1);
        assert!(m.contains_left(&1));
        assert!(m.contains_right(&"1"));
        assert_eq!(m.get_by_left(&1), Some(&"1"));
        assert_eq!(m.get_by_right(&"1"), Some(&1));

        assert_eq!(m.try_insert(2, "2"), Ok(()));
        assert_eq!(m.try_insert(3, "3"), Ok(()));
        assert_eq!(m.len(), 3);

        assert_eq!(m.remove_by_left(&1), Some("1"));
        assert_eq!(m.remove_by_left(&1), None);
        assert_eq!(m.remove_by_right(&"2"), Some(2));
        assert_eq!(m.remove_by_right(&"2"), None);

        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect_vec(), vec![(&3, &"3")]);
    }
}
