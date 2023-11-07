// TODO: Deprecate VertexId, use this for graph vertex index conversion.

use crate::num::Integer;


pub trait BaseOneConversion {
    fn from1b(self) -> Self;
    fn to1b(self) -> Self;
}

pub trait IteratorBaseOneConversion {
    type Item: Integer;
    type From1BOutput: Iterator<Item = Self::Item>;
    type To1BOutput: Iterator<Item = Self::Item>;
    fn from1b(self) -> Self::From1BOutput;
    fn to1b(self) -> Self::To1BOutput;
}

impl<T: Integer> BaseOneConversion for T {
    fn from1b(self) -> Self { self - T::one() }
    fn to1b(self) -> Self { self + T::one() }
}

impl<T: Integer, I: Iterator<Item = T> + 'static> IteratorBaseOneConversion for I {
    // Rust-upgrade (https://github.com/rust-lang/rust/issues/63063):
    //   Replace `Box<dyn Iterator<...>>` with `impl Iterator<...>`.
    type Item = T;
    type From1BOutput = Box<dyn Iterator<Item = T>>;
    type To1BOutput = Box<dyn Iterator<Item = T>>;
    fn from1b(self) -> Self::From1BOutput { Box::new(self.map(|x| x - T::one())) }
    fn to1b(self) -> Self::To1BOutput { Box::new(self.map(|x| x + T::one())) }
}

impl<T: Integer> BaseOneConversion for Vec<T> {
    fn from1b(self) -> Self { self.into_iter().map(|x| x.from1b()).collect() }
    fn to1b(self) -> Self { self.into_iter().map(|x| x.to1b()).collect() }
}

impl<T: Integer, const N: usize> BaseOneConversion for [T; N] {
    fn from1b(self) -> Self { self.map(|x| x.from1b()) }
    fn to1b(self) -> Self { self.map(|x| x.to1b()) }
}
