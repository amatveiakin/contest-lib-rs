pub trait IterutilsZipEq
where
    Self: Sized + Iterator,
{
    fn zip_eq<J: IntoIterator>(self, other: J) -> ZipEq<Self, J::IntoIter>;
}

impl<I: Iterator> IterutilsZipEq for I {
    fn zip_eq<J: IntoIterator>(self, other: J) -> ZipEq<Self, J::IntoIter> {
        ZipEq { iter_a: self, iter_b: other.into_iter() }
    }
}

pub struct ZipEq<I, J> {
    iter_a: I,
    iter_b: J,
}

impl<I, J> Iterator for ZipEq<I, J>
where
    I: Iterator,
    J: Iterator,
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter_a.next();
        let b = self.iter_b.next();
        assert!(a.is_some() == b.is_some());
        Some((a?, b?))
    }
}


#[cfg(test)]
mod tests {
    use crate::assert_panics;
    use crate::iterutils_basic::IterutilsBasic;

    use super::*;

    #[test]
    fn basic() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8];
        assert_eq!(a.iter().zip(&b).collect_vec(), a.iter().zip_eq(&b).collect_vec());
        assert_panics!(|| a.iter().zip_eq(&c).collect_vec());
    }
}
