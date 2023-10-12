use std::array;


pub trait IterutilsWindows
where
    Self: Sized + Iterator,
{
    fn array_windows<const N: usize>(self) -> ArrayWindows<Self, N>;
}

impl<I: Iterator> IterutilsWindows for I {
    fn array_windows<const N: usize>(mut self) -> ArrayWindows<Self, N> {
        assert!(N > 0);
        // Rust-upgrade (https://github.com/rust-lang/rust/issues/89379):
        //   Use `array::try_from_fn`.
        let mut data = array::from_fn(|_| None);
        for i in 0..N {
            if let Some(v) = self.next() {
                data[i] = Some(v);
            } else {
                return ArrayWindows {
                    iter: self,
                    next: None,
                };
            }
        }
        ArrayWindows {
            iter: self,
            next: Some(data.map(|v| v.unwrap())),
        }
    }
}

pub struct ArrayWindows<I: Iterator, const N: usize> {
    iter: I,
    next: Option<[I::Item; N]>,
}

impl<I: Iterator, const N: usize> Iterator for ArrayWindows<I, N>
where
    I::Item: Clone,
{
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next.clone();
        if let Some(v) = self.iter.next() {
            if let Some(ref mut next) = self.next {
                next.rotate_left(1);
                next[N - 1] = v;
            }
        } else {
            self.next = None;
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use crate::iterutils_basic::IterutilsBasic;

    use super::*;

    #[test]
    pub fn windows_normal() {
        let v = [1, 2, 3, 4, 5];
        let w = v.iter().copied().array_windows().collect_vec();
        assert_eq!(w, vec![[1, 2, 3], [2, 3, 4], [3, 4, 5]]);
    }

    #[test]
    pub fn windows_single() {
        let v = [1, 2, 3];
        let w = v.iter().copied().array_windows().collect_vec();
        assert_eq!(w, vec![[1, 2, 3]]);
    }

    #[test]
    pub fn windows_none() {
        let v = [1];
        let w = v.iter().copied().array_windows::<3>().collect_vec();
        assert!(w.is_empty());
    }

    #[test]
    pub fn windows_length_one() {
        let v = [1, 2, 3];
        let w = v.iter().copied().array_windows().collect_vec();
        assert_eq!(w, vec![[1], [2], [3]]);
    }
}
