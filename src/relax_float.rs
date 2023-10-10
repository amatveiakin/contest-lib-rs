// `Relax` equivalent for floats. Need a separate implementation: floating point types are not `Ord`
// because of NaN.

pub trait RelaxFloat {
    fn relax_min(&mut self, other: Self);
    fn relax_max(&mut self, other: Self);
    fn relax_clamp(&mut self, min: Self, max: Self);
}

macro_rules! impl_relax_float {
    ($($t:ty),*) => {
        $(
            impl RelaxFloat for $t {
                fn relax_min(&mut self, other: Self) {
                    *self = self.min(other);
                }
                fn relax_max(&mut self, other: Self) {
                    *self = self.max(other);
                }
                fn relax_clamp(&mut self, min: Self, max: Self) {
                    *self = self.clamp(min, max);
                }
            }
        )*
    };
}

impl_relax_float!(f32, f64);
