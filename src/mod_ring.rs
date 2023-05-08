// Improvement potential: Add benchmarks.
// Improvement potential: Add `PrimInt` parent trait and implement it for `ModNum`.

use std::{fmt, ops};

use crate::io::Emittable;


const M: i32 = 1_000_000_007;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ModNum {
    val: i32,  // always in [0, M)
}

impl ModNum {
    pub fn new_unchecked(x: i32) -> Self { ModNum { val: x } }
    pub fn val(&self) -> i32 { self.val }
}

impl From<i32> for ModNum {
    fn from(x: i32) -> Self { ModNum { val: x.rem_euclid(M) } }
}
impl From<u32> for ModNum {
    fn from(x: u32) -> Self { ModNum { val: x.rem_euclid(M as u32) as i32 } }
}
impl From<i64> for ModNum {
    fn from(x: i64) -> Self { ModNum { val: x.rem_euclid(M as i64) as i32 } }
}
impl From<u64> for ModNum {
    fn from(x: u64) -> Self { ModNum { val: x.rem_euclid(M as u64) as i32 } }
}

impl fmt::Display for ModNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.val) }
}
simple_emittable!(ModNum);

impl ops::Neg for ModNum {
    type Output = ModNum;
    fn neg(self) -> ModNum {
        ModNum::new_unchecked(if self.val == 0 { 0 } else { M - self.val })
    }
}

impl ops::Add for ModNum {
    type Output = ModNum;
    fn add(self, rhs: ModNum) -> ModNum {
        // On `if x < M` vs multiplying by `(x < M) as i32`. Both compile to the exact same assembly
        // relying on `cmovll` instead of branching or multiplication:
        // https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=d452dd3eb9c4b2b0060a800579a076bc
        let x = self.val + rhs.val;
        ModNum::new_unchecked(if x < M { x } else { x - M })
    }
}
impl ops::Sub for ModNum {
    type Output = ModNum;
    fn sub(self, rhs: ModNum) -> ModNum {
        let x = self.val - rhs.val;
        ModNum::new_unchecked(if x < 0 { x + M } else { x })
    }
}
impl ops::Mul for ModNum {
    type Output = ModNum;
    fn mul(self, rhs: ModNum) -> ModNum {
        ModNum::from((self.val as i64) * (rhs.val as i64))
    }
}
impl ops::Div for ModNum {
    type Output = ModNum;
    fn div(self, rhs: ModNum) -> ModNum {
        ModNum::from((self.val as i64) * (mod_inverse(rhs.val, M).unwrap() as i64))
    }
}

impl ops::AddAssign for ModNum {
    fn add_assign(&mut self, rhs: ModNum) { *self = *self + rhs; }
}
impl ops::SubAssign for ModNum {
    fn sub_assign(&mut self, rhs: ModNum) { *self = *self - rhs; }
}
impl ops::MulAssign for ModNum {
    fn mul_assign(&mut self, rhs: ModNum) { *self = *self * rhs; }
}
impl ops::DivAssign for ModNum {
    fn div_assign(&mut self, rhs: ModNum) { *self = *self / rhs; }
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


#[cfg(test)]
mod tests {
    use super::*;

    const M1: i32 = M - 1;

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
}
