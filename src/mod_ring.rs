// Improvement potential: Add benchmarks.
// Improvement potential: Add `PrimInt` parent trait and implement it for `ModNum`.

use std::{fmt, ops};

use crate::trait_for_value_and_ref;
use crate::io::Emittable;


pub const CODEFORCES_MOD: i32 = 1_000_000_007;

// M must:
//   - be greater than 1;
//   - be smaller than (i32::MAX / 2), because addition and subtraction are done with i32.
// These conditions are checked at compile time.
// M does not have to be prime, but if it's not, division may panic at runtime.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ModNumber<const M: i32> {
    val: i32,  // always in [0, M)
}

impl<const M: i32> ModNumber<M> {
    pub fn new_unchecked(x: i32) -> Self {
        // This is a compile-time assertion, but it must be located in a function that is called
        // at least once.
        Self::assert_mod_ok();
        ModNumber { val: x }
    }
    pub fn val(self) -> i32 { self.val }
    pub fn pow(self, mut exp: u32) -> Self {
        let mut res = ModNumber::new_unchecked(1);
        let mut base = self;
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
            }
            base *= base;
            exp >>= 1;
        }
        res
    }

    fn assert_mod_ok() { let () = AssertModOk::<M>::OK; }
}

impl<const M: i32> From<i32> for ModNumber<M> {
    fn from(x: i32) -> Self { ModNumber::new_unchecked(x.rem_euclid(M)) }
}
impl<const M: i32> From<u32> for ModNumber<M> {
    fn from(x: u32) -> Self { ModNumber::new_unchecked(x.rem_euclid(M as u32) as i32) }
}
impl<const M: i32> From<i64> for ModNumber<M> {
    fn from(x: i64) -> Self { ModNumber::new_unchecked(x.rem_euclid(M as i64) as i32) }
}
impl<const M: i32> From<u64> for ModNumber<M> {
    fn from(x: u64) -> Self { ModNumber::new_unchecked(x.rem_euclid(M as u64) as i32) }
}

impl<const M: i32> fmt::Display for ModNumber<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.val) }
}
trait_for_value_and_ref!(impl<{const M: i32}> Emittable for ModNumber<M> {
    fn emit(&self, writer: &mut impl std::io::Write) { write!(writer, "{} ", self).unwrap(); }
});

impl<const M: i32> ops::Neg for ModNumber<M> {
    type Output = Self;
    fn neg(self) -> Self {
        ModNumber::new_unchecked(if self.val == 0 { 0 } else { M - self.val })
    }
}

impl<const M: i32> ops::Add for ModNumber<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        // On `if x < M` vs multiplying by `(x < M) as i32`. Both compile to the exact same assembly
        // relying on `cmovll` instead of branching or multiplication:
        // https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=d452dd3eb9c4b2b0060a800579a076bc
        let x = self.val + rhs.val;
        ModNumber::new_unchecked(if x < M { x } else { x - M })
    }
}
impl<const M: i32> ops::Sub for ModNumber<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let x = self.val - rhs.val;
        ModNumber::new_unchecked(if x < 0 { x + M } else { x })
    }
}
impl<const M: i32> ops::Mul for ModNumber<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        ModNumber::from((self.val as i64) * (rhs.val as i64))
    }
}
impl<const M: i32> ops::Div for ModNumber<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        ModNumber::from((self.val as i64) * (mod_inverse(rhs.val, M).unwrap() as i64))
    }
}

impl<const M: i32> ops::AddAssign for ModNumber<M> {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}
impl<const M: i32> ops::SubAssign for ModNumber<M> {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}
impl<const M: i32> ops::MulAssign for ModNumber<M> {
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}
impl<const M: i32> ops::DivAssign for ModNumber<M> {
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}

fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    let (g, x, _) = egcd(a, m);
    if g != 1 {
        None
    } else {
        Some(x.rem_euclid(m))
    }
}

#[allow(dead_code)]
struct AssertModOk<const M: i32>;
impl<const M: i32> AssertModOk<M> {
    // TODO: Better static assertion: this fails with `cargo build`, but not with `cargo check`.
    #[allow(dead_code)]
    const OK: () = assert!(1 < M && M <= i32::MAX / 2);
}


#[cfg(test)]
mod tests {
    use crate::internal_testing::*;
    use super::*;

    const M: i32 = CODEFORCES_MOD;
    const M1: i32 = M - 1;
    type ModNum = ModNumber<M>;

    #[test]
    fn creation() {
        assert_eq!(ModNum::from(0).val(), 0);
        assert_eq!(ModNum::from(1).val(), 1);
        assert_eq!(ModNum::from(-1).val(), M1);
        assert_eq!(ModNum::from(-M1).val(), 1);
        assert_eq!(ModNum::from(M).val(), 0);
        assert_eq!(ModNum::from(-M).val(), 0);
        assert_eq!(ModNum::from(M + 7).val(), 7);
        assert_eq!(ModNum::from(-M - 3).val(), M - 3);
        assert_eq!(ModNum::from(4_000_000_000u32).val(), 999999979);
        assert_eq!(ModNum::from(1_000_000_007_000_000_003_i64).val(), 3);
        assert_eq!(ModNum::from(-1_000_000_007_000_000_003_i64).val(), 1000000004);
        assert_eq!(ModNum::from(18_000_000_000_000_000_000_u64).val(), 882);
    }

    #[test]
    fn negation() {
        assert_eq!(-ModNum::from(0), ModNum::from(0));
        assert_eq!(-ModNum::from(1), ModNum::from(-1));
        assert_eq!(-ModNum::from(100), ModNum::from(-100));
        assert_eq!(-ModNum::from(M1), ModNum::from(-M1));
    }

    #[test]
    fn addition() {
        assert_eq!(ModNum::from(0) + ModNum::from(0), ModNum::from(0));
        assert_eq!(ModNum::from(1) + ModNum::from(0), ModNum::from(1));
        assert_eq!(ModNum::from(1) + ModNum::from(1), ModNum::from(2));
        assert_eq!(ModNum::from(1) + ModNum::from(2), ModNum::from(3));
        assert_eq!(ModNum::from(1) + ModNum::from(-1), ModNum::from(0));
        assert_eq!(ModNum::from(-1) + ModNum::from(-1), ModNum::from(-2));
        assert_eq!(ModNum::from(1) + ModNum::from(-2), ModNum::from(-1));
        assert_eq!(ModNum::from(1_000_000_000) + ModNum::from(500_000_007), ModNum::from(500_000_000));
    }

    #[test]
    fn subtraction() {
        assert_eq!(ModNum::from(0) - ModNum::from(0), ModNum::from(0));
        assert_eq!(ModNum::from(1) - ModNum::from(0), ModNum::from(1));
        assert_eq!(ModNum::from(1) - ModNum::from(1), ModNum::from(0));
        assert_eq!(ModNum::from(1) - ModNum::from(-1), ModNum::from(2));
        assert_eq!(ModNum::from(1) - ModNum::from(2), ModNum::from(-1));
        assert_eq!(ModNum::from(-1) - ModNum::from(1), ModNum::from(-2));
    }

    #[test]
    fn multiplication() {
        assert_eq!(ModNum::from(0) * ModNum::from(0), ModNum::from(0));
        assert_eq!(ModNum::from(0) * ModNum::from(1), ModNum::from(0));
        assert_eq!(ModNum::from(1) * ModNum::from(1), ModNum::from(1));
        assert_eq!(ModNum::from(1) * ModNum::from(-1), ModNum::from(-1));
        assert_eq!(ModNum::from(-1) * ModNum::from(-1), ModNum::from(1));
        assert_eq!(ModNum::from(-1) * ModNum::from(-1), ModNum::from(1));
        assert_eq!(ModNum::from(1_000_000) * ModNum::from(1_000_000), ModNum::from(999993007));
    }

    #[test]
    fn division() {
        assert_eq!(ModNum::from(0) / ModNum::from(1), ModNum::from(0));
        assert_eq!(ModNum::from(1) / ModNum::from(1), ModNum::from(1));
        assert_eq!(ModNum::from(-1) / ModNum::from(-1), ModNum::from(1));
        assert_eq!(ModNum::from(60) / ModNum::from(6), ModNum::from(10));
        assert_eq!(ModNum::from(1) / ModNum::from(2), ModNum::from(M / 2 + 1));
        assert_eq!(ModNum::from(1) / ModNum::from(3), ModNum::from(M / 3 + 1));
    }

    #[test]
    fn invertibility() {
        const VALUES: &[i32; 12] = &[
            1, -1, 2, -2, 3, -3, 4, -4, 1000, 1_000_000, 500_000_000, 500_000_011
        ];
        for &a in VALUES {
            for &b in VALUES {
                let a = ModNum::from(a);
                let b = ModNum::from(b);
                assert_eq!((a + b) - b, a);
                assert_eq!((a - b) + b, a);
                assert_eq!((a / b) * b, a);
                assert_eq!((a * b) / b, a);
            }
        }
    }

    #[test]
    fn pow() {
        assert_eq!(ModNum::from(2).pow(0), ModNum::from(1));
        assert_eq!(ModNum::from(2).pow(1), ModNum::from(2));
        assert_eq!(ModNum::from(2).pow(2), ModNum::from(4));
        assert_eq!(ModNum::from(2).pow(3), ModNum::from(8));
        assert_eq!(ModNum::from(2).pow(4), ModNum::from(16));
        assert_eq!(ModNum::from(2).pow(5), ModNum::from(32));
        assert_eq!(ModNum::from(2).pow(10), ModNum::from(1024));
        assert_eq!(ModNum::from(2).pow(30), ModNum::from(73741817));
        assert_eq!(ModNum::from(0).pow(3), ModNum::from(0));
        assert_eq!(ModNum::from(10).pow(5), ModNum::from(100000));
    }

    #[test]
    fn non_prime_nod() {
        type Num6 = ModNumber<6>;
        assert_eq!(Num6::from(2) + Num6::from(8), Num6::from(4));
        assert_eq!(Num6::from(2) - Num6::from(3), Num6::from(5));
        assert_eq!(Num6::from(2) * Num6::from(5), Num6::from(4));
        assert_eq!(Num6::from(2) * Num6::from(3), Num6::from(0));
        assert_eq!(Num6::from(1) / Num6::from(5), Num6::from(5));
        let result = catch_unwind_silent(|| Num6::from(1) / Num6::from(4));
        assert!(result.is_err());
    }
}
