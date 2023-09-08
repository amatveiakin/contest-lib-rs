// Given an array of length N with K distinct values does O(N * K) time and space precomputation and
// then answers the question "How many times does value X appear in the subarray [L, R]?" in O(1).

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::prefix_accumulate::PrefixSum;
use crate::u32_index::U32Index;


pub struct SegmentBucketCounter<T>
where
    T: Clone + Eq + Hash
{
    buckets: HashMap<T, PrefixSum<u32>>,
}

impl<T> SegmentBucketCounter<T>
where
    T: Clone + Eq + Hash
{
    pub fn new(slice: &[T]) -> Self {
        let unique_values = slice.iter().clone().collect::<HashSet<_>>();
        let mut buckets = HashMap::new();
        for value in unique_values {
            buckets.insert(
                value.clone(),
                PrefixSum::from_iter(slice.iter().map(|x| (*x == *value) as u32))
            );
        }
        Self { buckets }
    }

    pub fn count(&self, value: T, idx: impl U32Index) -> u32 {
        self.buckets.get(&value).map(|s| s.get(idx)).unwrap_or(0)
    }
}
