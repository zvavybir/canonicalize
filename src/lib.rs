#![no_std]

/// Canonicalizes values
pub trait Canonicalize: Sized
{
    /// Type in which errors get reported
    type ErrorType;
    /// Performs the canonicalizing
    fn canon<E>(self) -> Result<Self, Self::ErrorType>;
}
