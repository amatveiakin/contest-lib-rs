use std::ops;

use crate::counting_set::CountingSet;
use crate::factors::factors;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FactoredNum {
    factors: CountingSet<u32>
}

impl FactoredNum {
    // Requires: The first number in each pair must be prime. Otherwise the result is undefined.
    pub fn from_factors(factors: impl IntoIterator<Item = (u32, u32)>) -> Self {
        FactoredNum {
            factors: CountingSet::from_group_iter(factors.into_iter().map(|(a, n)| (a, n as usize)))
        }
    }

    // Not ordered! If ordering is needed, sort manually.
    pub fn factors(&self) -> impl ExactSizeIterator<Item = (u32, u32)> + '_ {
        self.factors.group_iter().map(|(&a, n)| (a, n as u32))
    }

    pub fn num_divisors(&self) -> u32 {
        self.factors.group_iter().map(|(_, c)| c as u32 + 1).product()
    }

    pub fn divisible_by(&self, rhs: &Self) -> bool {
        self.factors.is_superset(&rhs.factors)
    }

    pub fn gcd(&self, rhs: &Self) -> Self {
        FactoredNum {
            factors: self.factors.intersection(&rhs.factors)
        }
    }

    pub fn lcm(&self, rhs: &Self) -> Self {
        FactoredNum {
            factors: self.factors.union(&rhs.factors)
        }
    }
}

impl From<u32> for FactoredNum {
    fn from(x: u32) -> Self {
        FactoredNum {
            factors: CountingSet::from_group_iter(
                factors(x).into_iter().map(|(p, c)| (p, c as usize)))
        }
    }
}

impl From<FactoredNum> for u32 {
    fn from(x: FactoredNum) -> Self {
        x.factors.group_iter().map(|(p, c)| p.pow(c as u32)).product()
    }
}

impl ops::MulAssign for FactoredNum {
    fn mul_assign(&mut self, rhs: Self) {
        for (&p, c) in rhs.factors.group_iter() {
            self.factors.push_multiple(p, c);
        }
    }
}

impl ops::Mul for FactoredNum {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::Div for FactoredNum {
    type Output = Option<Self>;
    fn div(mut self, rhs: Self) -> Self::Output {
        for (&p, c) in rhs.factors.group_iter() {
            if !self.factors.remove_exact(p, c) {
                return None;
            }
        }
        Some(self)
    }
}


#[cfg(test)]
mod tests {
    use crate::common_divisors::{gcd, lcm};

    use super::*;

    #[test]
    fn equivalence() {
        let numbers = [1, 2, 3, 4, 5, 6, 8, 9, 12, 16, 27, 30, 32, 60, 64];
        for x in numbers {
            for y in numbers {
                let fx = FactoredNum::from(x);
                let fy = FactoredNum::from(y);
                assert_eq!(x * y, u32::from(fx.clone() * fy.clone()));
                assert_eq!(gcd(x, y), u32::from(fx.gcd(&fy)));
                assert_eq!(lcm(x, y), u32::from(fx.lcm(&fy)));
                if x % y == 0 {
                    assert!(fx.divisible_by(&fy));
                    assert_eq!(Some(x / y), (fx.clone() / fy.clone()).map(|v| u32::from(v)));
                } else {
                    assert!(!fx.divisible_by(&fy));
                    assert_eq!(None, fx.clone() / fy.clone());
                }
            }
        }
    }

    #[test]
    fn num_divisors() {
        assert_eq!(FactoredNum::from(1).num_divisors(), 1);
        assert_eq!(FactoredNum::from(2).num_divisors(), 2);
        assert_eq!(FactoredNum::from(3).num_divisors(), 2);
        assert_eq!(FactoredNum::from(4).num_divisors(), 3);
        assert_eq!(FactoredNum::from(6).num_divisors(), 4);
        assert_eq!(FactoredNum::from(8).num_divisors(), 4);
        assert_eq!(FactoredNum::from(12).num_divisors(), 6);
        assert_eq!(FactoredNum::from(16).num_divisors(), 5);
    }
}
