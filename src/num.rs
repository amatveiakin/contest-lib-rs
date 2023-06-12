use std::ops;


pub trait IntegerRing:
    Sized
    + Copy
    + Eq
    + ops::Add<Self, Output = Self>
    + ops::Sub<Self, Output = Self>
    + ops::Mul<Self, Output = Self>
    + ops::Div<Self, Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

pub trait RegularInteger:
    IntegerRing
    + Ord
    + ops::Rem<Self, Output = Self>
    + ops::Not<Output = Self>
    + ops::BitAnd<Output = Self>
    + ops::BitOr<Output = Self>
    + ops::BitXor<Output = Self>
    + ops::Shl<usize, Output = Self>
    + ops::Shr<usize, Output = Self>
{
    fn min_value() -> Self;
    fn max_value() -> Self;
}

macro_rules! impl_integer {
    ( $( $t:ty, )* ) => { $(
        impl IntegerRing for $t {
            fn zero() -> Self { 0 }
            fn one() -> Self { 1 }
        }
        impl RegularInteger for $t {
            fn min_value() -> Self { <$t>::MIN }
            fn max_value() -> Self { <$t>::MAX }
        }
    )* }
}

impl_integer!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
);
