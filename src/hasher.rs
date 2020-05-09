use core::convert::AsRef;
use core::fmt::Debug;
use hex::{FromHex, ToHex};

pub trait Hasher {
    type Output: Debug + ToHex + FromHex + PartialEq + AsRef<[u8]>;

    fn update(&mut self, data: impl AsRef<[u8]>);

    fn finalize(self) -> Self::Output;
}
