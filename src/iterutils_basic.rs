// Analog of Itertools. Mostly a subset, but some interfaces are different. Notably, we are using
// arrays instead of homogenous tuples.
//
// This file contains basic utilities. Larger tools are stored separately as "itertools_foo.rs".

use std::fmt;


pub trait IterutilsBasic
where
    Self: Sized + Iterator,
{
    fn join(self, sep: &str) -> String where Self::Item: fmt::Display;
    fn collect_vec(self) -> Vec<Self::Item>;
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
