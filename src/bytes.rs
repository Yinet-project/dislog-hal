use core::fmt::Debug;
use hex::{FromHex, ToHex};

pub trait Bytes: Sized {
    type BytesType: Debug + FromHex + ToHex + AsRef<[u8]>;

    type Error: Debug;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;

    fn to_bytes(&self) -> Self::BytesType;
}
