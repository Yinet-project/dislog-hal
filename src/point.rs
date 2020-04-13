use core::ops::{Add, Mul, Neg};

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
    pub inner: P,
}

impl<P: DisLogPoint> Point<P> {
    pub fn order() -> Scalar<P::Scalar> {
        Scalar {
            inner: P::order()
        }
    }
    
    pub fn zero() -> Point<P> {
        Point {
            inner: P::zero()
        }
    }

    pub fn one() -> Point<P> {
        Point {
            inner: P::one()
        }
    }

    pub fn generator() -> Point<P> {
        Point {
            inner: P::generator()
        }
    }
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

impl<'a, 'b, P: DisLogPoint> Add<&'b Point<P>>
    for &'a Point<P>
{
    type Output = Point<P>;

    fn add(self, rhs: &'b Point<P>) -> Point<P> {
        let inner = self.inner.add(&rhs.inner);
        Point { inner }
    }
}

impl<'a, P: DisLogPoint> Neg for &'a Point<P>
{
    type Output = Point<P>;

    fn neg(self) -> Point<P> {
        Point {
            inner: self.inner.neg()
        }
    }
}


impl<P: DisLogPoint> Neg for Point<P>
{
    type Output = Point<P>;

    fn neg(self) -> Point<P> {
        -&self
    }
}

impl<'b, P: DisLogPoint> Add<&'b Point<P>>
    for Point<P>
{
    type Output = Point<P>;

    fn add(self, rhs: &'b Point<P>) -> Point<P> {
        &self + rhs
    }
}

impl<'a, P: DisLogPoint> Add<Point<P>>
    for &'a Point<P>
{
    type Output = Point<P>;

    fn add(self, rhs: Point<P>) -> Point<P> {
        self + &rhs
    }
}

impl<P: DisLogPoint> Add<Point<P>>
    for Point<P>
{
    type Output = Point<P>;

    fn add(self, rhs: Point<P>) -> Point<P> {
        &self + &rhs
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

impl<'b, P: DisLogPoint<Scalar = S>, S: ScalarNumber<Point = P>> Mul<&'b Scalar<S>>
    for Point<P>
{
    type Output = Point<P>;

    fn mul(self, rhs: &'b Scalar<S>) -> Point<P> {
        &self * rhs
    }
}

impl<P: DisLogPoint<Scalar = S>, S: ScalarNumber<Point = P>> Mul<Scalar<S>>
    for Point<P>
{
    type Output = Point<P>;

    fn mul(self, rhs: Scalar<S>) -> Point<P> {
        &self * &rhs
    }
}
