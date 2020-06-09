use crate::Canonicalize;

use std::path::PathBuf;
use std::collections::VecDeque;
use std::ffi::FromBytesWithNulError;
use std::ffi::IntoStringError;
use std::time::SystemTimeError;

impl Canonicalize for String {}
impl Canonicalize for FromBytesWithNulError {}
impl Canonicalize for IntoStringError {}
impl Canonicalize for SystemTimeError {}

/// Canonicalizes a vector
///
/// This canonicalizes every single element.
/// # Bugs #
/// This implementation only works if the single elements implement the `Copy` trait.  I think that this isn't realy necessary
impl<T: Canonicalize + Copy> Canonicalize for Vec<T>
{
    fn canon(self) -> Self
    {
        self.iter().map(|x| x.canon()).collect()
    }
}

/// Canonicalize a `PathBuf`
///
/// Canonicalies a `PathBuf`
/// # Panics #
/// This panics if std::fs::canonicalize returns an Err variant.
impl Canonicalize for PathBuf
{
    fn canon(self) -> Self
    {
        self.canonicalize().unwrap() // This unwrap is on purpose.
    }
}

impl<T> Canonicalize for [T; 0]
{
    fn canon(self) -> Self
    {
        []
    }
}

impl<T: Canonicalize> Canonicalize for Box<T>
{
    fn canon(self) -> Self
    {
        Box::new((*self).canon())
    }
}

impl<T: Canonicalize + Copy> Canonicalize for VecDeque<T>
{
    fn canon(self) -> Self
    {
        self.iter().map(|x| x.canon()).collect()
    }
}
