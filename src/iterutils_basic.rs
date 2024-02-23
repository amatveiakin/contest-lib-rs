// Analog of Itertools. Mostly a subset, but some interfaces are different:
//   - Using arrays instead of homogenous tuples;
//   - Some functions panic instead of returning an `Option`: we would always unwrap it anyway.
//
// This file contains basic utilities. Larger tools are stored separately as "itertools_foo.rs".

use std::hash::Hash;
use std::{fmt, array};
use std::collections::HashSet;


pub trait IterutilsBasic
where
    Self: Sized + Iterator,
{
    fn join(self, sep: &str) -> String where Self::Item: fmt::Display;
    fn collect_vec(self) -> Vec<Self::Item>;
    fn collect_array<const N: usize>(self) -> [Self::Item; N];
    fn collect_set(self) -> HashSet<Self::Item> where Self::Item: Eq + Hash;
}

impl<I: Iterator> IterutilsBasic for I {
    fn join(self, sep: &str) -> String where Self::Item: fmt::Display {
        let mut ret = String::new();
        for (i, item) in self.enumerate() {
            if i > 0 {
                ret += sep;
            }
            ret += &item.to_string();
        }
        ret
    }

    fn collect_vec(self) -> Vec<Self::Item> { self.collect() }

    fn collect_array<const N: usize>(mut self) -> [Self::Item; N] {
        let ret = array::from_fn(|_| self.next().unwrap());
        assert!(self.next().is_none());
        ret
    }

    fn collect_set(self) -> HashSet<Self::Item> where Self::Item: Eq + Hash {
        self.collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join() {
        let v = [1, 2, 3];
        assert!(v.iter().join(", ") == "1, 2, 3");
    }
}
