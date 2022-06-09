use std::ops;


// Provides a way of indexing an array-like structure with either a single index or a range
// without having to overload indexing for every range type separately.
pub trait U32Index {
    // Returns bound, lower bound inclusive, upper bound exclusive.
    // If lower is unbounded, 0 is returned.
    // If upper is unbounded, size is returned.
    fn bounds(self, size: u32) -> (u32, u32);
}

impl U32Index for u32 {
    fn bounds(self, _size: u32) -> (u32, u32) { (self, self + 1) }
}

impl U32Index for ops::Range<u32> {
    fn bounds(self, _size: u32) -> (u32, u32) { (self.start, self.end) }
}
impl U32Index for ops::RangeInclusive<u32> {
    fn bounds(self, _size: u32) -> (u32, u32) { (*self.start(), *self.end() + 1) }
}
impl U32Index for ops::RangeFrom<u32> {
    fn bounds(self, size: u32) -> (u32, u32) { (self.start, size) }
}
impl U32Index for ops::RangeTo<u32> {
    fn bounds(self, _size: u32) -> (u32, u32) { (0, self.end) }
}
impl U32Index for ops::RangeToInclusive<u32> {
    fn bounds(self, _size: u32) -> (u32, u32) { (0, self.end + 1) }
}
impl U32Index for ops::RangeFull {
    fn bounds(self, size: u32) -> (u32, u32) { (0, size) }
}
