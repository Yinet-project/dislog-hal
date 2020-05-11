use core::ops::{Add, AddAssign, Mul, Neg, Sub};

use crate::Bytes;
use crate::Scalar;
use crate::ScalarNumber;
use serde::{Deserialize, Serialize};

pub trait DisLogPoint: Bytes + Clone + PartialEq + Serialize + for<'de> Deserialize<'de> {
    type Scalar: ScalarNumber;

    fn order() -> Self::Scalar;

    fn zero() -> Self;

    fn one() -> Self;

    fn generator() -> Self;

    fn add(&self, o: &Self) -> Self;

    fn mul(&self, o: &Self::Scalar) -> Self;

    fn neg(&self) -> Self;

    fn get_x(&self) -> Scalar<Self::Scalar>;

    fn get_y(&self) -> Scalar<Self::Scalar>;
}

#[derive(Serialize, Deserialize)]
pub struct Point<P: DisLogPoint>(#[serde(bound(deserialize = "P: DisLogPoint"))] pub P);

impl<P: DisLogPoint> Point<P> {
    pub fn order() -> Scalar<P::Scalar> {
        Scalar(P::order())
    }

    pub fn zero() -> Point<P> {
        Point(P::zero())
    }

    pub fn one() -> Point<P> {
        Point(P::one())
    }

    pub fn generator() -> Point<P> {
        Point(P::generator())
    }

    pub fn get_x(&self) -> Scalar<P::Scalar> {
        self.0.get_x()
    }

    pub fn get_y(&self) -> Scalar<P::Scalar> {
        self.0.get_y()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, P::Error> {
        match P::from_bytes(bytes) {
            Ok(x) => Ok(Self(x)),
            Err(x) => Err(x),
        }
    }

    pub fn to_bytes(&self) -> P::BytesType {
        self.0.to_bytes()
    }
}

impl<P: DisLogPoint> Clone for Point<P> {
    fn clone(&self) -> Point<P> {
        Point(self.0.clone())
    }
}

impl<P: DisLogPoint> core::fmt::Debug for Point<P> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "Point{{\n\tbytes: {:?},\n}}", &self.0.to_bytes())
    }
}

impl<P: DisLogPoint> Default for Point<P> {
    fn default() -> Point<P> {
        let inner = P::generator();
        Point(inner)
    }
}

impl<P: DisLogPoint> PartialEq for Point<P> {
    fn eq(&self, other: &Point<P>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<P: DisLogPoint> Eq for Point<P> {}

impl<'a, 'b, P: DisLogPoint> Add<&'b Point<P>> for &'a Point<P> {
    type Output = Point<P>;

    fn add(self, rhs: &'b Point<P>) -> Point<P> {
        let inner = self.0.add(&rhs.0);
        Point(inner)
    }
}

define_l_val_r_ref!(Point, DisLogPoint, Add, add, Point<T>);
define_l_val_r_val!(Point, DisLogPoint, Add, add, Point<T>);
define_l_ref_r_val!(Point, DisLogPoint, Add, add, Point<T>);

impl<'a, P: DisLogPoint> Neg for &'a Point<P> {
    type Output = Point<P>;

    fn neg(self) -> Point<P> {
        Point(self.0.neg())
    }
}

impl<P: DisLogPoint> Neg for Point<P> {
    type Output = Point<P>;

    fn neg(self) -> Point<P> {
        -&self
    }
}

impl<'a, 'b, P: DisLogPoint<Scalar = S>, S: ScalarNumber<Point = P>> Mul<&'b Scalar<S>>
    for &'a Point<P>
{
    type Output = Point<P>;

    fn mul(self, rhs: &'b Scalar<S>) -> Point<P> {
        let inner = self.0.mul(&rhs.0);
        Point(inner)
    }
}

impl<'b, P: DisLogPoint<Scalar = S>, S: ScalarNumber<Point = P>> Mul<&'b Scalar<S>> for Point<P> {
    type Output = Point<P>;

    fn mul(self, rhs: &'b Scalar<S>) -> Point<P> {
        &self * rhs
    }
}

impl<P: DisLogPoint<Scalar = S>, S: ScalarNumber<Point = P>> Mul<Scalar<S>> for Point<P> {
    type Output = Point<P>;

    fn mul(self, rhs: Scalar<S>) -> Point<P> {
        &self * &rhs
    }
}

impl<'a, 'b, S: DisLogPoint> Sub<&'b Point<S>> for &'a Point<S> {
    type Output = Point<S>;

    fn sub(self, rhs: &'b Point<S>) -> Point<S> {
        let inner = self.0.add(&rhs.0.neg());
        Point(inner)
    }
}

define_l_val_r_ref!(Point, DisLogPoint, Sub, sub, Point<T>);
define_l_val_r_val!(Point, DisLogPoint, Sub, sub, Point<T>);
define_l_ref_r_val!(Point, DisLogPoint, Sub, sub, Point<T>);

impl<'b, S: DisLogPoint> AddAssign<&'b Point<S>> for Point<S> {
    fn add_assign(&mut self, rhs: &'b Self) {
        self.0 = self.0.add(&rhs.0)
    }
}
