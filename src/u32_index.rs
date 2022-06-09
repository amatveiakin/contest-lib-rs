use std::ops;


// Provides a way of indexing an array-like structure with either a single index or a range
// without having to overload indexing for every range type separately.
pub trait U32Index {
    // Returns bound, lower bound inclusive, upper bound exclusive.
    // If lower is unbounded, 0 is returned.
    // If upper is unbounded, `size` is returned.
    // Also checks that bounds are valid and within `size`.
    fn bounds(self, size: u32) -> (u32, u32);
}

impl U32Index for u32 {
    fn bounds(self, size: u32) -> (u32, u32) { checked((self, self + 1), size) }
}

impl U32Index for ops::Range<u32> {
    fn bounds(self, size: u32) -> (u32, u32) { checked((self.start, self.end), size) }
}
impl U32Index for ops::RangeInclusive<u32> {
    fn bounds(self, size: u32) -> (u32, u32) { checked((*self.start(), *self.end() + 1), size) }
}
impl U32Index for ops::RangeFrom<u32> {
    fn bounds(self, size: u32) -> (u32, u32) { checked((self.start, size), size) }
}
impl U32Index for ops::RangeTo<u32> {
    fn bounds(self, size: u32) -> (u32, u32) { checked((0, self.end), size) }
}
impl U32Index for ops::RangeToInclusive<u32> {
    fn bounds(self, size: u32) -> (u32, u32) { checked((0, self.end + 1), size) }
}
impl U32Index for ops::RangeFull {
    fn bounds(self, size: u32) -> (u32, u32) { checked((0, size), size) }
}

fn checked(bounds: (u32, u32), size: u32) -> (u32, u32) {
    let (begin, end) = bounds;
    assert!(begin <= end, "[{}, {}) segment invalid", begin, end);
    assert!(end <= size, "[{}, {}) is not a valid range for a collection of {} elements", begin, end, size);
    bounds
}
