use core::fmt::Debug;
use hex::{FromHex, ToHex};

pub trait Bytes: Sized {
    type BytesType: Debug + FromHex + ToHex;

    type Error;

    fn from_bytes(bytes: Self::BytesType) -> Result<Self, Self::Error>;

    fn to_bytes(&self) -> Self::BytesType;
}

