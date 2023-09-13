// Add the following functions to all integers:
//   - `div_up(a, b)`: returns a / b rounded up when b is positive. Behavior for negative b is
//     unspecified. Implemented as `(a + b - 1) / b`, so it may overflow unnecessarily.

use crate::num::RegularInteger;


pub trait IntegerExtension {
    // Rust-upgrade (https://github.com/rust-lang/rust/issues/88581): remove when built-in
    // `div_ceil` is stable.
    fn div_up(self, rhs: Self) -> Self;
}

impl<T: RegularInteger> IntegerExtension for T {
    fn div_up(self, rhs: Self) -> Self {
        (self + rhs - T::one()) / rhs
    }
}
