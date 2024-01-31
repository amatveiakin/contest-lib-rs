// TODO: Deprecate VertexId, use this for graph vertex index conversion.

use crate::num::Integer;


pub trait BaseOneConversion {
    fn from1b(self) -> Self;
    fn to1b(self) -> Self;
}

pub trait IteratorBaseOneConversion {
    type Item: Integer;
    fn from1b(self) -> impl Iterator<Item = Self::Item>;
    fn to1b(self) -> impl Iterator<Item = Self::Item>;
}

impl<T: Integer> BaseOneConversion for T {
    fn from1b(self) -> Self { self - T::one() }
    fn to1b(self) -> Self { self + T::one() }
}

impl<T: Integer, I: Iterator<Item = T> + 'static> IteratorBaseOneConversion for I {
    type Item = T;
    fn from1b(self) -> impl Iterator<Item = T> { self.map(|x| x - T::one()) }
    fn to1b(self) -> impl Iterator<Item = T> { self.map(|x| x + T::one()) }
}

impl<T: Integer> BaseOneConversion for Vec<T> {
    fn from1b(self) -> Self { self.into_iter().map(|x| x.from1b()).collect() }
    fn to1b(self) -> Self { self.into_iter().map(|x| x.to1b()).collect() }
}

impl<T: Integer, const N: usize> BaseOneConversion for [T; N] {
    fn from1b(self) -> Self { self.map(|x| x.from1b()) }
    fn to1b(self) -> Self { self.map(|x| x.to1b()) }
}
