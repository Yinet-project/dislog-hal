pub trait Bytes {
    type BytesType;

    fn from_bytes(bytes: Self::BytesType) -> Self;

    fn to_bytes(&self) -> Self::BytesType;
}
