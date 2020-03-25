use core::ops::{Mul};

use crate::ScalarNumber;
use crate::Scalar;

pub trait DisLogPoint {
    type Scalar: ScalarNumber;

    const ZERO: Self;
    
    const ONE: Self;

    const GENERATOR: Self;

    fn add(self, o: Self) -> Self;

    fn mul(self, o: Self::Scalar) -> Self;

    fn inv(self) -> Self;
}

pub struct Point<P: DisLogPoint> {
    pub inner: P,
}

impl<P: DisLogPoint<Scalar = S>, S: ScalarNumber<Point = P>> Mul<Scalar<S>> for Point<P> {
    type Output = Self;

    fn mul(self, rhs: Scalar<S>) -> Self {
        let inner = self.inner.mul(rhs.inner);
        Self { inner }
    }
}


