pub trait RelaxMinMax {
    fn relax_min(&mut self, other: Self);
    fn relax_max(&mut self, other: Self);
    fn relax_clamp(&mut self, min: Self, max: Self);
}

impl<T: Ord> RelaxMinMax for T {
    fn relax_min(&mut self, other: Self) {
        if other < *self {
            *self = other;
        }
    }

    fn relax_max(&mut self, other: Self) {
        if other > *self {
            *self = other;
        }
    }

    fn relax_clamp(&mut self, min: Self, max: Self) {
        assert!(min <= max);
        self.relax_min(max);
        self.relax_max(min);
    }
}
