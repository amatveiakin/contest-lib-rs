use crate::num::{Integer, RingInteger};
use crate::u32_index::U32Index;


#[macro_export]
macro_rules! define_prefix_accumulation {
    ($struct_name:ident for $t:ident : ( $( $t_bounds:tt )* ); with {
        identity() { $identity:expr }
        combine($union_lhs:ident, $union_rhs:ident) { $combine:expr }
        uncombine($difference_lhs:ident, $difference_rhs:ident) { $uncombine:expr }
    }) => {
        pub struct $struct_name<$t> {
            prefixes: Vec<$t>,
        }

        impl<$t> $struct_name<$t>
        where
            $t: $( $t_bounds )*,
        {
            pub fn from_iter(iter: impl IntoIterator<Item = $t>) -> Self {
                let iter = iter.into_iter();
                let mut prefixes = vec![Self::identity()];
                prefixes.reserve_exact(iter.size_hint().0);
                for v in iter {
                    prefixes.push(Self::combine(*prefixes.last().unwrap(), v));
                }
                Self { prefixes }
            }

            // Note. Would be great to use `Index` instead, but it returns a reference.
            pub fn get(&self, idx: impl U32Index) -> $t {
                let (begin, end) = idx.bounds(self.prefixes.len() as u32 - 1);
                Self::uncombine(self.prefixes[end as usize], self.prefixes[begin as usize])
            }

            fn identity() -> $t {
                $identity
            }
            fn combine($union_lhs: $t, $union_rhs: $t) -> $t {
                $combine
            }
            fn uncombine($difference_lhs: $t, $difference_rhs: $t) -> $t {
                $uncombine
            }
        }
    };
}


define_prefix_accumulation! {
    PrefixSum
    for T: (std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Default + Clone + Copy);
    with {
        identity() { T::default() }
        combine(a, b) { a + b }
        uncombine(a, b) { a - b }
    }
}

define_prefix_accumulation! {
    PrefixProduct
    for T: (RingInteger);
    with {
        identity() { T::one() }
        combine(a, b) { a * b }
        uncombine(a, b) { a / b }
    }
}

define_prefix_accumulation! {
    PrefixXor
    for T: (Integer);
    with {
        identity() { T::zero() }
        combine(a, b) { a ^ b }
        uncombine(a, b) { a ^ b }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v = vec![1, -1, 2, -2, 3, -3, 4];
        let sums = PrefixSum::from_iter(v);
        assert_eq!(sums.get(0..7), 4);
        assert_eq!(sums.get(0..0), 0);
        assert_eq!(sums.get(1..4), -1);
        assert_eq!(sums.get(1..=4), 2);
        assert_eq!(sums.get(3..=3), -2);
        assert_eq!(sums.get(3..), 2);
        assert_eq!(sums.get(..3), 2);
        assert_eq!(sums.get(..), 4);
    }
}
