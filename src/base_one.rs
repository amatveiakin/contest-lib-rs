use crate::num::Integer;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Base {
    ZERO,
    ONE,
}

pub trait BaseOneConversion where Self: Sized {
    fn from1b(self) -> Self { self.from_base(Base::ONE) }
    fn to1b(self) -> Self { self.to_base(Base::ONE) }
    fn from_base(self, base: Base) -> Self;
    fn to_base(self, base: Base) -> Self;
}

pub trait IteratorBaseOneConversion where Self: Sized {
    type Item: Integer;
    fn from1b(self) -> impl Iterator<Item = Self::Item> { self.from_base(Base::ONE) }
    fn to1b(self) -> impl Iterator<Item = Self::Item> { self.to_base(Base::ONE) }
    fn from_base(self, base: Base) -> impl Iterator<Item = Self::Item>;
    fn to_base(self, base: Base) -> impl Iterator<Item = Self::Item>;
}

impl<T: Integer> BaseOneConversion for T {
    fn from_base(self, base: Base) -> Self { self - base_value(base) }
    fn to_base(self, base: Base) -> Self { self + base_value(base) }
}

impl<T: Integer, I: Iterator<Item = T> + 'static> IteratorBaseOneConversion for I {
    type Item = T;
    fn from_base(self, base: Base) -> impl Iterator<Item = T> { self.map(move |x| x - base_value(base)) }
    fn to_base(self, base: Base) -> impl Iterator<Item = T> { self.map(move |x| x + base_value(base)) }
}

impl<T: Integer> BaseOneConversion for Vec<T> {
    fn from_base(self, base: Base) -> Self { self.into_iter().map(|x| x.from_base(base)).collect() }
    fn to_base(self, base: Base) -> Self { self.into_iter().map(|x| x.to_base(base)).collect() }
}

impl<T: Integer, const N: usize> BaseOneConversion for [T; N] {
    fn from_base(self, base: Base) -> Self { self.map(move |x| x.from_base(base)) }
    fn to_base(self, base: Base) -> Self { self.map(move |x| x.to_base(base)) }
}

fn base_value<T: Integer>(base: Base) -> T {
    match base {
        Base::ZERO => T::zero(),
        Base::ONE => T::one(),
    }
}
