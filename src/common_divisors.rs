use crate::num::Integer;


pub fn gcd<T: Integer>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        (a, b) = (b, a % b);
    }
    a
}

pub fn lcm<T: Integer>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}
