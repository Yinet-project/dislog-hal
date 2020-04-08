use core::ops::{Add, Mul};

use crate::Bytes;
use crate::DisLogPoint;
use crate::Point;

pub trait ScalarNumber: Bytes + Clone + Copy {
    type Point: DisLogPoint;

    fn order() -> Self;

    fn zero() -> Self;

    fn one() -> Self;

    fn add(&self, o: &Self) -> Self;

    fn mul(&self, o: &Self) -> Self;

    fn inv(&self) -> Self;

    fn neg(&self) -> Self;
}

// #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Scalar<S: ScalarNumber> {
    pub(crate) inner: S,
}

impl<S: ScalarNumber> Clone for Scalar<S> {
    fn clone(&self) -> Scalar<S> {
        Scalar {
            inner: self.inner.clone(),
        }
    }
}

impl<S: ScalarNumber> Copy for Scalar<S> {}

impl<'a, 'b, S: ScalarNumber> Add<&'b Scalar<S>> for &'a Scalar<S> {
    type Output = Scalar<S>;

    fn add(self, rhs: &'b Scalar<S>) -> Scalar<S> {
        let inner = self.inner.mul(&rhs.inner);
        Scalar { inner }
    }
}

impl<'a, 'b, S: ScalarNumber> Mul<&'b Scalar<S>> for &'a Scalar<S> {
    type Output = Scalar<S>;

    fn mul(self, rhs: &'b Scalar<S>) -> Scalar<S> {
        let inner = self.inner.mul(&rhs.inner);
        Scalar { inner }
    }
}

impl<'a, 'b, S: ScalarNumber<Point = P>, P: DisLogPoint<Scalar = S>> Mul<&'b Point<P>>
    for &'a Scalar<S>
{
    type Output = Point<P>;

    fn mul(self, rhs: &'b Point<P>) -> Point<P> {
        let inner = rhs.inner.mul(&self.inner);
        Point { inner }
    }
}
