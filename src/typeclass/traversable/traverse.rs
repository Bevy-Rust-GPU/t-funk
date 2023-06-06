use crate::macros::{functions, types};

#[functions]
#[types]
pub trait Traverse<F, P> {
    type Traverse;

    fn traverse(self, f: F) -> Self::Traverse;
}

