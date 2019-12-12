//! Path manipulation

use core::convert::AsRef;
use core::{
    cmp,
};

use generic_array::{
    ArrayLength,
    GenericArray,
    typenum::marker_traits::Unsigned as _,
};

use crate::{
    traits,
};

pub struct Path<S> (GenericArray<u8, S::PATH_MAX>)
where
    S: traits::Storage,
    <S as traits::Storage>::PATH_MAX: ArrayLength<u8>,
;

impl<S> Path<S>
where
    S: traits::Storage,
    <S as traits::Storage>::PATH_MAX: ArrayLength<u8>
{
    /// Silently truncates to maximum configured path length
    pub fn new<P: AsRef<[u8]> + ?Sized>(p: &P) -> Self {
        let mut padded_path: GenericArray<u8, S::PATH_MAX> = Default::default();
        let name_max = <S as traits::Storage>::PATH_MAX::to_usize();
        let len = cmp::min(name_max - 1, p.as_ref().len());
        padded_path[..len].copy_from_slice(&p.as_ref()[..len]);
        Path(padded_path)
    }

    pub fn is_absolute(&self) -> bool {
        self.has_root()
    }

    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    pub fn has_root(&self) -> bool {
        self.0.len() > 0 && self.0[0] == b'/'
    }
}

impl<S> From<&str> for Path<S>
where
    S: traits::Storage,
    <S as traits::Storage>::PATH_MAX: ArrayLength<u8>
{
    fn from(p: &str) -> Path<S> {
        Path::new(p.as_bytes())
    }
}

impl<S> From<&[u8]> for Path<S>
where
    S: traits::Storage,
    <S as traits::Storage>::PATH_MAX: ArrayLength<u8>
{
    fn from(p: &[u8]) -> Path<S> {
        Path::new(p)
    }
}

