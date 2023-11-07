// Add the following functions to all integers:
//   - `div_up(a, b)`: returns a / b rounded up when b is positive. Behavior for negative b is
//     unspecified. Implemented as `(a + b - 1) / b`, so it may overflow unnecessarily.

use crate::num::Integer;


pub trait IntegerExtension {
    // Rust-upgrade: `div_ceil` for unsigned ints is stable since Rust 1.73.
    // Rust-upgrade (https://github.com/rust-lang/rust/issues/88581): `div_ceil` for signed ints.
    fn div_up(self, rhs: Self) -> Self;
}

impl<T: Integer> IntegerExtension for T {
    fn div_up(self, rhs: Self) -> Self {
        (self + rhs - T::one()) / rhs
    }
}
