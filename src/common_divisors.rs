use crate::num::RegularInteger;


pub fn gcd<T: RegularInteger>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        (a, b) = (b, a % b);
    }
    a
}

pub fn lcm<T: RegularInteger>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}
