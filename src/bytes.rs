use core::fmt::Debug;

pub trait Bytes: Sized {
    type BytesType: Debug;

    type Error;

    fn from_bytes(bytes: Self::BytesType) -> Result<Self, Self::Error>;

    fn to_bytes(&self) -> Self::BytesType;
}
