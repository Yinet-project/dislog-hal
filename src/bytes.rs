use core::fmt::Debug;

pub trait Bytes {
    type BytesType: Debug;

    fn from_bytes(bytes: Self::BytesType) -> Self;

    fn to_bytes(&self) -> Self::BytesType;
}
