use core::ops::{Add, Mul};

use crate::Bytes;
use crate::Scalar;
use crate::ScalarNumber;

pub trait DisLogPoint: Bytes {
    type Scalar: ScalarNumber;

    fn order() -> Self::Scalar;

    fn zero() -> Self;

    fn one() -> Self;

    fn generator() -> Self;

    fn add(&self, o: &Self) -> Self;

    fn mul(&self, o: &Self::Scalar) -> Self;

    fn neg(&self) -> Self;
}

// #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Point<P: DisLogPoint> {
    pub(crate) inner: P,
}

impl<'a, 'b, P: DisLogPoint<Scalar = S>, S: ScalarNumber<Point = P>> Add<&'b Point<P>>
    for &'a Point<P>
{
    type Output = Point<P>;

    fn add(self, rhs: &'b Point<P>) -> Point<P> {
        let inner = self.inner.add(&rhs.inner);
        Point { inner }
    }
}

impl<'a, 'b, P: DisLogPoint<Scalar = S>, S: ScalarNumber<Point = P>> Mul<&'b Scalar<S>>
    for &'a Point<P>
{
    type Output = Point<P>;

    fn mul(self, rhs: &'b Scalar<S>) -> Point<P> {
        let inner = self.inner.mul(&rhs.inner);
        Point { inner }
    }
}
