// A 2D point/vector type. Does not distinguish between the two. ("2D vector" would've been a better
// name for this hybrid entity, but similarity with `Vec` could be confusing, so sticking with "2D
// point".)
//
// Operations on point assume that `x = x @ y` and `x @= y` are equivalent for any arithmetic
// operation `@`.
//
// Improvement potential. Add basic shapes and non-trivial operations: line crossing, tangent,
// convex hull, shape intersection, bounding box, etc.

use std::ops;

use crate::io;
use crate::num::{SignedNumber, Float};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self { Self { x, y } }

    pub fn into_array(self) -> [T; 2] { [self.x, self.y] }
    pub fn from_array([x, y]: [T; 2]) -> Self { Self { x, y } }

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Point2D<U> {
        Point2D { x: f(self.x), y: f(self.y) }
    }

    pub fn dot(self, rhs: Self) -> T
    where T: ops::Add<Output = T> + ops::Mul<Output = T>
    {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Default> Point2D<T> {
    pub fn zero() -> Self { Self::default() }
}

impl<T: SignedNumber> Point2D<T> {
    pub fn l1_norm(&self) -> T { self.x.abs() + self.y.abs() }
    pub fn l2_norm_sqr(&self) -> T { self.dot(*self)}
    pub fn linf_norm(&self) -> T { self.x.abs().maxv(self.y.abs()) }

    pub fn l1_dist(&self, rhs: Self) -> T { (*self - rhs).l1_norm() }
    pub fn l2_dist_sqr(&self, rhs: Self) -> T { (*self - rhs).l2_norm_sqr() }
    pub fn linf_dist(&self, rhs: Self) -> T { (*self - rhs).linf_norm() }
}

impl<T: Float> Point2D<T> {
    pub fn l2_norm(&self) -> T { self.l2_norm_sqr().sqrt() }

    pub fn l2_dist(&self, rhs: Self) -> T { (*self - rhs).l2_norm() }
}

impl<T: ops::Neg> ops::Neg for Point2D<T> {
    type Output = Point2D<T::Output>;
    fn neg(self) -> Self::Output {
        Point2D { x: -self.x, y: -self.y }
    }
}

impl<T: ops::AddAssign> ops::AddAssign for Point2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T: ops::AddAssign> ops::Add for Point2D<T> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<T: ops::SubAssign> ops::SubAssign for Point2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl<T: ops::SubAssign> ops::Sub for Point2D<T> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl<T: ops::MulAssign + Clone> ops::MulAssign<T> for Point2D<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs;
    }
}
impl<T: ops::MulAssign + Clone> ops::Mul<T> for Point2D<T> {
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self {
        self *= rhs;
        self
    }
}

impl <T: ops::DivAssign + Clone> ops::DivAssign<T> for Point2D<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs;
    }
}
impl <T: ops::DivAssign + Clone> ops::Div<T> for Point2D<T> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self {
        self /= rhs;
        self
    }
}

pub trait PointReading {
    fn p2_i32(&mut self) -> Point2D<i32>;
    fn p2_i64(&mut self) -> Point2D<i64>;
    fn p2_f32(&mut self) -> Point2D<f32>;
    fn p2_f64(&mut self) -> Point2D<f64>;
}

impl<R: std::io::BufRead> PointReading for io::Reader<R> {
    fn p2_i32(&mut self) -> Point2D<i32> { Point2D::new(self.i32(), self.i32()) }
    fn p2_i64(&mut self) -> Point2D<i64> { Point2D::new(self.i64(), self.i64()) }
    fn p2_f32(&mut self) -> Point2D<f32> { Point2D::new(self.f32(), self.f32()) }
    fn p2_f64(&mut self) -> Point2D<f64> { Point2D::new(self.f64(), self.f64()) }
}
