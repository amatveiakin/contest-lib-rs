use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Indexer<T: Eq + Hash> {
    indices: HashMap<T, usize>,
}

impl <T: Eq + Hash> Indexer<T> {
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    pub fn get_index(&mut self, value: T) -> usize {
        let len = self.indices.len();
        *self.indices.entry(value).or_insert(len)
    }
}

// `derive(Default)` only works for `T: Default`
impl<T: Eq + Hash> Default for Indexer<T> {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut indexer = Indexer::new();
        assert_eq!(indexer.len(), 0);
        assert_eq!(indexer.get_index("a"), 0);
        assert_eq!(indexer.get_index("b"), 1);
        assert_eq!(indexer.get_index("a"), 0);
        assert_eq!(indexer.len(), 2);
    }
}
