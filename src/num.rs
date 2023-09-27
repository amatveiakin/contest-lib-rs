use std::ops;


pub trait IntegerRing:
    Sized
    + Copy
    + Eq
    + ops::Add<Self, Output = Self>
    + ops::Sub<Self, Output = Self>
    + ops::Mul<Self, Output = Self>
    + ops::Div<Self, Output = Self>
    + ops::AddAssign<Self>
    + ops::SubAssign<Self>
    + ops::MulAssign<Self>
    + ops::DivAssign<Self>
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
    + ops::RemAssign<Self>
    + ops::BitAndAssign
    + ops::BitOrAssign
    + ops::BitXorAssign
    + ops::ShlAssign<usize>
    + ops::ShrAssign<usize>
{
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn from_u32(v: u32) -> Self;
    fn from_usize(v: usize) -> Self;
    fn to_u32(self) -> u32;
    fn to_usize(self) -> usize;
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
            fn from_u32(v: u32) -> Self { v as Self }
            fn from_usize(v: usize) -> Self { v as Self }
            fn to_u32(self) -> u32 { self as u32 }
            fn to_usize(self) -> usize { self as usize }
        }
    )* }
}

impl_integer!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
);
