use crate::Bytes;
use crate::DisLogPoint;
use crate::Point;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub};
use rand::RngCore;
use serde::{Deserialize, Serialize};

pub trait ScalarNumber: Bytes + Clone + PartialEq + Serialize + for<'de> Deserialize<'de> {
    type Point: DisLogPoint;

    fn random<R: RngCore>(rng: &mut R) -> Self;

    fn order() -> Self;

    fn zero() -> Self;

    fn one() -> Self;

    fn add(&self, o: &Self) -> Self;

    fn mul(&self, o: &Self) -> Self;

    fn inv(&self) -> Self;

    fn neg(&self) -> Self;
}

#[derive(Serialize, Deserialize)]
pub struct Scalar<S: ScalarNumber>(#[serde(bound(deserialize = "S: ScalarNumber"))] pub S);

impl<S: ScalarNumber> Scalar<S> {
    pub fn random<R: RngCore>(rng: &mut R) -> Scalar<S> {
        Scalar(S::random::<R>(rng))
    }

    pub fn order() -> Scalar<S> {
        Scalar(S::order())
    }

    pub fn zero() -> Scalar<S> {
        Scalar(S::zero())
    }

    pub fn one() -> Scalar<S> {
        Scalar(S::one())
    }

    pub fn inv(&self) -> Scalar<S> {
        Scalar(self.0.inv())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, S::Error> {
        match S::from_bytes(bytes) {
            Ok(x) => Ok(Self(x)),
            Err(x) => Err(x),
        }
    }

    pub fn to_bytes(&self) -> S::BytesType {
        self.0.to_bytes()
    }
}

impl<S: ScalarNumber> Clone for Scalar<S> {
    fn clone(&self) -> Scalar<S> {
        Scalar(self.0.clone())
    }
}

impl<S: ScalarNumber> core::fmt::Debug for Scalar<S> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "Scalar{{\n\tbytes: {:?},\n}}", &self.0.to_bytes())
    }
}

impl<S: ScalarNumber> Default for Scalar<S> {
    fn default() -> Scalar<S> {
        let inner = S::zero();
        Scalar(inner)
    }
}

impl<S: ScalarNumber> PartialEq for Scalar<S> {
    fn eq(&self, other: &Scalar<S>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<S: ScalarNumber> Eq for Scalar<S> {}

impl<'a, S: ScalarNumber> Neg for &'a Scalar<S> {
    type Output = Scalar<S>;

    fn neg(self) -> Scalar<S> {
        Scalar(self.0.neg())
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
        let inner = self.0.add(&rhs.0);
        Scalar(inner)
    }
}

define_l_val_r_ref!(Scalar, ScalarNumber, Add, add, Scalar<T>);
define_l_val_r_val!(Scalar, ScalarNumber, Add, add, Scalar<T>);
define_l_ref_r_val!(Scalar, ScalarNumber, Add, add, Scalar<T>);

impl<'a, 'b, S: ScalarNumber> Mul<&'b Scalar<S>> for &'a Scalar<S> {
    type Output = Scalar<S>;

    fn mul(self, rhs: &'b Scalar<S>) -> Scalar<S> {
        let inner = self.0.mul(&rhs.0);
        Scalar(inner)
    }
}

define_l_val_r_ref!(Scalar, ScalarNumber, Mul, mul, Scalar<T>);
define_l_val_r_val!(Scalar, ScalarNumber, Mul, mul, Scalar<T>);
define_l_ref_r_val!(Scalar, ScalarNumber, Mul, mul, Scalar<T>);

impl<'a, 'b, S: ScalarNumber<Point = P>, P: DisLogPoint<Scalar = S>> Mul<&'b Point<P>>
    for &'a Scalar<S>
{
    type Output = Point<P>;

    fn mul(self, rhs: &'b Point<P>) -> Point<P> {
        let inner = rhs.0.mul(&self.0);
        Point(inner)
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
        let inner = self.0.add(&rhs.0.neg());
        Scalar(inner)
    }
}

define_l_val_r_ref!(Scalar, ScalarNumber, Sub, sub, Scalar<T>);
define_l_val_r_val!(Scalar, ScalarNumber, Sub, sub, Scalar<T>);
define_l_ref_r_val!(Scalar, ScalarNumber, Sub, sub, Scalar<T>);

impl<'b, S: ScalarNumber> AddAssign<&'b Scalar<S>> for Scalar<S> {
    fn add_assign(&mut self, rhs: &'b Self) {
        self.0 = self.0.add(&rhs.0)
    }
}

impl<'b, S: ScalarNumber> MulAssign<&'b Scalar<S>> for Scalar<S> {
    fn mul_assign(&mut self, rhs: &'b Self) {
        self.0 = self.0.mul(&rhs.0)
    }
}
