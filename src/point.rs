use core::ops::{Add, Mul};

use crate::Bytes;
use crate::Scalar;
use crate::ScalarNumber;

pub trait DisLogPoint: Bytes + Clone + Copy + PartialEq {
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

impl<P: DisLogPoint> Clone for Point<P> {
    fn clone(&self) -> Point<P> {
        Point {
            inner: self.inner.clone(),
        }
    }
}

impl<P: DisLogPoint> Copy for Point<P> {}

impl<P: DisLogPoint> core::fmt::Debug for Point<P> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "Point{{\n\tbytes: {:?},\n}}", &self.inner.to_bytes())
    }
}

impl<P: DisLogPoint> Default for Point<P> {
    fn default() -> Point<P> {
        let inner = P::generator();
        Point { inner }
    }
}

impl<P: DisLogPoint> PartialEq for Point<P> {
    fn eq(&self, other: &Point<P>) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<P: DisLogPoint> Eq for Point<P> {}

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
