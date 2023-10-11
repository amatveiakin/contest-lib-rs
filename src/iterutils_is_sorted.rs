// Rust-upgrade (https://github.com/rust-lang/rust/issues/53485):
//   Replace with built-in `is_sorted`.


pub trait IterutilsIsSorted
where
    Self: Sized + Iterator,
{
    // Cannot use `is_sorted` to avoid collisions with future built-in `is_sorted`.
    fn issorted(self) -> bool;
    fn issorted_by_key<K: Ord>(self, f: impl FnMut(Self::Item) -> K) -> bool;
}

impl<I: Iterator> IterutilsIsSorted for I
where
    I::Item: Ord,
{
    fn issorted(self) -> bool {
        self.issorted_by_key(std::convert::identity)
    }

    fn issorted_by_key<K: Ord>(mut self, mut f: impl FnMut(Self::Item) -> K) -> bool {
        let Some(mut prev) = self.next().map(|v| f(v)) else {
            return true;
        };
        for v in self {
            let cur = f(v);
            if cur < prev {
                return false;
            }
            prev = cur;
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sorted() {
        assert!(([] as [i32; 0]).iter().issorted());
        assert!([7].iter().issorted());
        assert!([1, 3, 7, 10].iter().issorted());
        assert!([1, 1, 1].iter().issorted());
        assert!([1, 2, 2, 3, 3, 3].iter().issorted());
    }

    #[test]
    pub fn not_sorted() {
        assert!(![2, 1].iter().issorted());
        assert!(![12, 6, 3].iter().issorted());
        assert!(![3, 3, 3, 2, 2, 1].iter().issorted());
    }

    #[test]
    pub fn sorted_by_key() {
        assert!(([] as [i32; 0]).iter().issorted_by_key(|_| panic!("What key?")));
        assert!(["a"].iter().issorted_by_key(|s| s.len()));
        assert!(["b", "aa"].iter().issorted_by_key(|s| s.len()));
    }

    #[test]
    pub fn not_sorted_by_key() {
        assert!(!["aa", "b"].iter().issorted_by_key(|s| s.len()));
    }
}
