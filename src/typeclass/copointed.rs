use crate::macros::{functions, types};

/// A type that can unwrap a value
#[functions]
#[types]
pub trait Copointed
where
    Self: Sized,
{
    type Copointed;

    /// Unwrap `Unit` from `Self`
    fn copoint(self) -> Self::Copointed;
}
