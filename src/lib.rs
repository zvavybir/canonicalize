#![cfg_attr(not(feature = "std"), no_std)]

use core::cell::Cell;
use core::cell::Ref;
use core::cell::UnsafeCell;
use core::any::TypeId;
use core::array::TryFromSliceError;
use core::ascii;
use core::cmp::Ordering;
use core::str::Utf8Error;
use core::fmt::Formatter;
use core::time::Duration;

#[cfg(feature = "std")]
mod std;

/// Canonicalizes values
///
/// Every type implementing this trait can be canonicalited.
///
/// Many `core` and `std` types implement this trait.
///
/// It is garanteed that it will never be cloned.
pub trait Canonicalize: Sized
{
    /// Performs the canonicalizing
    fn canon(self) -> Self
    {
        self
    }
}

impl Canonicalize for () {}
impl Canonicalize for u8 {}
impl Canonicalize for i8 {}
impl Canonicalize for u16 {}
impl Canonicalize for i16 {}
impl Canonicalize for u32 {}
impl Canonicalize for i32 {}
impl Canonicalize for u64 {}
impl Canonicalize for i64 {}
impl Canonicalize for f32 {}
impl Canonicalize for f64 {}
impl Canonicalize for usize {}
impl Canonicalize for isize {}
impl Canonicalize for bool {}
impl Canonicalize for char {}
impl<T> Canonicalize for Cell<T> {}
impl<T> Canonicalize for Ref<'_, T> {}
impl<T> Canonicalize for UnsafeCell<T> {}
impl Canonicalize for TypeId {}
impl Canonicalize for TryFromSliceError {}
impl Canonicalize for ascii::EscapeDefault {}
impl Canonicalize for Ordering {}
impl Canonicalize for Utf8Error {}
impl Canonicalize for Formatter<'_> {}
impl Canonicalize for core::fmt::Error {}
impl Canonicalize for Duration {}

impl<T: Canonicalize> Canonicalize for Option<T>
{
    fn canon(self) -> Self
    {
        match self
        {
            Some(x) => Some(x.canon()),
            None => None,
        }
    }
}

impl<T: Canonicalize, E: Canonicalize> Canonicalize for Result<T, E>
{
    fn canon(self) -> Self
    {
        match self
        {
            Ok(x) => Ok(x.canon()),
            Err(e) => Err(e.canon()),
        }
    }
}
