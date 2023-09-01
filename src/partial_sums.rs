use crate::u32_index::U32Index;


pub struct PartialSums<T>
where
    T: Default + Clone + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>
{
    sums: Vec<T>,
}

impl<T> PartialSums<T>
where
    T: Default + Clone + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>
{
    pub fn from_iter(iter: impl ExactSizeIterator<Item = T>) -> Self {
        let mut sums = vec![T::default(); iter.len() + 1];
        for (i, v) in iter.enumerate() {
            sums[i + 1] = sums[i] + v;
        }
        Self { sums }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        Self::from_iter(slice.iter().copied())
    }

    pub fn sum(&self, idx: impl U32Index) -> T {
        let (begin, end) = idx.bounds(self.sums.len() as u32 - 1);
        self.sums[end as usize] - self.sums[begin as usize]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v = vec![1, -1, 2, -2, 3, -3, 4];
        let sums = PartialSums::from_slice(&v);
        assert_eq!(sums.sum(0..7), 4);
        assert_eq!(sums.sum(0..0), 0);
        assert_eq!(sums.sum(1..4), -1);
        assert_eq!(sums.sum(1..=4), 2);
        assert_eq!(sums.sum(3..=3), -2);
        assert_eq!(sums.sum(3..), 2);
        assert_eq!(sums.sum(..3), 2);
        assert_eq!(sums.sum(..), 4);
    }
}
