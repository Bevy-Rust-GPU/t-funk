use crate::macros::{functions, types};

/// A type with a binary associative function.
#[functions]
#[types]
pub trait Mappend<T> {
    type Mappend;

    fn mappend(self, t: T) -> Self::Mappend;
}

