//   RingNumber  <-- Number  <-- SignedNumber
//        ^             ^             ^
//        |             |             |
//   RingInteger <-- Integer        Float

use std::ops;


pub trait RingNumber:
    Sized
    + Copy
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
    fn zero_ref() -> &'static Self;
    fn one_ref() -> &'static Self;
}

pub trait Number: RingNumber {
    const MIN: Self;
    const MAX: Self;

    // Cannot rely on `Ord`: floats are not `Ord` because of NaN.
    fn minv(self, rhs: Self) -> Self;
    fn maxv(self, rhs: Self) -> Self;
}

pub trait SignedNumber:
    Number
    + ops::Neg<Output = Self>
{
    fn abs(self) -> Self;
}

pub trait RingInteger:
    RingNumber
    + Eq
{}

pub trait Integer:
    Number
    + RingInteger
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
    fn from_u32(v: u32) -> Self;
    fn from_usize(v: usize) -> Self;
    fn to_u32(self) -> u32;
    fn to_usize(self) -> usize;
}

pub trait Float: SignedNumber {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;

    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn ln(self) -> Self;
}

macro_rules! impl_integer {
    ( $( $t:ty, )* ) => { $(
        impl RingNumber for $t {
            fn zero() -> Self { 0 }
            fn one() -> Self { 1 }
            fn zero_ref() -> &'static Self { &0 }
            fn one_ref() -> &'static Self { &1 }
        }
        impl Number for $t {
            const MIN: $t = <$t>::MIN;
            const MAX: $t = <$t>::MAX;
            fn minv(self, rhs: Self) -> Self { self.min(rhs) }
            fn maxv(self, rhs: Self) -> Self { self.max(rhs) }
        }
        impl RingInteger for $t {}
        impl Integer for $t {
            fn from_u32(v: u32) -> Self { v as Self }
            fn from_usize(v: usize) -> Self { v as Self }
            fn to_u32(self) -> u32 { self as u32 }
            fn to_usize(self) -> usize { self as usize }
        }
    )* }
}

macro_rules! impl_signed_integer {
    ( $( $t:ty, )* ) => { $(
        impl SignedNumber for $t {
            fn abs(self) -> Self { self.abs() }
        }
    )* }
}

macro_rules! impl_float {
    ( $( $t:ty, )* ) => { $(
        impl RingNumber for $t {
            fn zero() -> Self { 0.0 }
            fn one() -> Self { 1.0 }
            fn zero_ref() -> &'static Self { &0.0 }
            fn one_ref() -> &'static Self { &1.0 }
        }
        impl Number for $t {
            const MIN: $t = <$t>::MIN;
            const MAX: $t = <$t>::MAX;
            fn minv(self, rhs: Self) -> Self { self.min(rhs) }
            fn maxv(self, rhs: Self) -> Self { self.max(rhs) }
        }
        impl SignedNumber for $t {
            fn abs(self) -> Self { self.abs() }
        }
        impl Float for $t {
            fn floor(self) -> Self { self.floor() }
            fn ceil(self) -> Self { self.ceil() }
            fn round(self) -> Self { self.round() }

            fn powi(self, n: i32) -> Self { self.powi(n) }
            fn powf(self, n: Self) -> Self { self.powf(n) }
            fn sqrt(self) -> Self { self.sqrt() }
            fn exp(self) -> Self { self.exp() }
            fn ln(self) -> Self { self.ln() }
        }
    )* }
}

impl_integer!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
);

impl_signed_integer!(
    i8, i16, i32, i64, i128, isize,
);

impl_float!(
    f32, f64,
);
