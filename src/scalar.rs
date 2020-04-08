use core::ops::{Mul};

use crate::DisLogPoint;
use crate::Point;

pub trait ScalarNumber {
    type Point: DisLogPoint;

    fn order() -> Self;

    fn zero() -> Self;

    fn one() -> Self;
    
    fn add(self, o: Self) -> Self;

    fn mul(self, o: Self) -> Self;

    fn inv(self) -> Self;

    fn neg(self) -> Self;
}

pub struct Scalar<S: ScalarNumber> {
    pub inner: S
}

impl<S: ScalarNumber> Mul for Scalar<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let inner = self.inner.mul(rhs.inner);
        Self { inner }
    }
}

impl<S: ScalarNumber<Point = P>, P: DisLogPoint<Scalar = S>> Mul<Point<P>> for Scalar<S> {
    type Output = Point<P>;

    fn mul(self, rhs: Point<P>) -> Point<P> {
        let inner = rhs.inner.mul(self.inner);
        Point { inner }
    }
}

