// Returns the smallest power of two greater then or equal to `v`.
// Returns 1 for `v` == 0.
pub fn ceil_to_pow_2(v: u32) -> u32 {
    let mut v = v.saturating_sub(1);
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v = v.overflowing_add(1).0;
    v
}
