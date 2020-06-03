#![no_std]

/// Canonicalizes values
pub trait Canonicalize: Sized
{
    /// Performs the canonicalizing
    fn canon<E>(self) -> Result<Self, E>;
}
