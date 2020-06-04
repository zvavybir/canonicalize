#![no_std]

use core::cell::Cell;
use core::cell::Ref;
use core::cell::UnsafeCell;

/// Canonicalizes values
///
/// Every type implementing this trait can be canonicalited.
///
/// Many `core` types implement this trait
pub trait Canonicalize: Sized
{
    /// Performs the canonicalizing
    fn canon(self) -> Self
    {
        self
    }
}

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
