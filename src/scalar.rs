use core::ops::{Add, Mul, Neg, Sub, AddAssign, MulAssign};

use crate::Bytes;
use crate::DisLogPoint;
use crate::Point;

/// This trait restrict scalar number's behavier.
pub trait ScalarNumber: Bytes + Clone + Copy + PartialEq {
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
    pub inner: S,
}

impl<S: ScalarNumber> Scalar<S> {
    pub fn order() -> Scalar<S> {
        Scalar {
            inner: S::order()
        }
    }

    pub fn zero() -> Scalar<S> {
        Scalar {
            inner: S::zero()
        }
    }

    pub fn one() -> Scalar<S> {
        Scalar {
            inner: S::one()
        }
    }

    pub fn inv(&self) -> Scalar<S> {
        Scalar {
            inner: self.inner.inv()
        }
    }
}

impl<S: ScalarNumber> Clone for Scalar<S> {
    fn clone(&self) -> Scalar<S> {
        Scalar {
            inner: self.inner.clone(),
        }
    }
}

impl<S: ScalarNumber> Copy for Scalar<S> {}

impl<S: ScalarNumber> core::fmt::Debug for Scalar<S> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "Scalar{{\n\tbytes: {:?},\n}}", &self.inner.to_bytes())
    }
}

impl<S: ScalarNumber> Default for Scalar<S> {
    fn default() -> Scalar<S> {
        let inner = S::zero();
        Scalar { inner }
    }
}

impl<S: ScalarNumber> PartialEq for Scalar<S> {
    fn eq(&self, other: &Scalar<S>) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<S: ScalarNumber> Eq for Scalar<S> {}

impl<'a, S: ScalarNumber> Neg for &'a Scalar<S> {
    type Output = Scalar<S>;

    fn neg(self) -> Scalar<S> {
        Scalar {
            inner: self.inner.neg(),
        }
    }
}

impl<S: ScalarNumber> Neg for Scalar<S> {
    type Output = Scalar<S>;

    fn neg(self) -> Scalar<S> {
        -&self
    }
}

impl<'a, 'b, S: ScalarNumber> Add<&'b Scalar<S>> for &'a Scalar<S> {
    type Output = Scalar<S>;

    fn add(self, rhs: &'b Scalar<S>) -> Scalar<S> {
        let inner = self.inner.add(&rhs.inner);
        Scalar { inner }
    }
}

impl<'a, S: ScalarNumber> Add<Scalar<S>> for &'a Scalar<S> {
    type Output = Scalar<S>;

    fn add(self, rhs: Scalar<S>) -> Scalar<S> {
        self + &rhs
    }
}

impl<'b, S: ScalarNumber> Add<&'b Scalar<S>> for Scalar<S> {
    type Output = Scalar<S>;

    fn add(self, rhs: &'b Scalar<S>) -> Scalar<S> {
        &self + rhs
    }
}

impl<S: ScalarNumber> Add<Scalar<S>> for Scalar<S> {
    type Output = Scalar<S>;

    fn add(self, rhs: Scalar<S>) -> Scalar<S> {
        &self + &rhs
    }
}

impl<'a, 'b, S: ScalarNumber> Mul<&'b Scalar<S>> for &'a Scalar<S> {
    type Output = Scalar<S>;

    fn mul(self, rhs: &'b Scalar<S>) -> Scalar<S> {
        let inner = self.inner.mul(&rhs.inner);
        Scalar { inner }
    }
}

impl<'a, S: ScalarNumber> Mul<Scalar<S>> for &'a Scalar<S> {
    type Output = Scalar<S>;

    fn mul(self, rhs: Scalar<S>) -> Scalar<S> {
        self * &rhs
    }
}

impl<'b, S: ScalarNumber> Mul<&'b Scalar<S>> for Scalar<S> {
    type Output = Scalar<S>;

    fn mul(self, rhs: &'b Scalar<S>) -> Scalar<S> {
        &self * rhs
    }
}

impl<S: ScalarNumber> Mul<Scalar<S>> for Scalar<S> {
    type Output = Scalar<S>;

    fn mul(self, rhs: Scalar<S>) -> Scalar<S> {
        &self * &rhs
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

impl<'a, S: ScalarNumber<Point = P>, P: DisLogPoint<Scalar = S>> Mul<Point<P>> for &'a Scalar<S> {
    type Output = Point<P>;

    fn mul(self, rhs: Point<P>) -> Point<P> {
        self * &rhs
    }
}

impl<'b, S: ScalarNumber<Point = P>, P: DisLogPoint<Scalar = S>> Mul<&'b Point<P>> for Scalar<S> {
    type Output = Point<P>;

    fn mul(self, rhs: &'b Point<P>) -> Point<P> {
        &self * rhs
    }
}

impl<'b, S: ScalarNumber<Point = P>, P: DisLogPoint<Scalar = S>> Mul<Point<P>> for Scalar<S> {
    type Output = Point<P>;

    fn mul(self, rhs: Point<P>) -> Point<P> {
        &self * &rhs
    }
}

impl<'a, 'b, S: ScalarNumber> Sub<&'b Scalar<S>> for &'a Scalar<S> {
    type Output = Scalar<S>;

    fn sub(self, rhs: &'b Scalar<S>) -> Scalar<S> {
        let inner = self.inner.add(&rhs.inner.neg());
        Scalar { inner }
    }
}

impl<'b, S: ScalarNumber> AddAssign<&'b Scalar<S>> for Scalar<S> {
    fn add_assign(&mut self, rhs: &'b Self) {
        self.inner = self.inner.add(&rhs.inner)
    }
}

impl<'b, S: ScalarNumber> MulAssign<&'b Scalar<S>> for Scalar<S> {
    fn mul_assign(&mut self, rhs: &'b Self) {
        self.inner = self.inner.mul(&rhs.inner)
    }
}
