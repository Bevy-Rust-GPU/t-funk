use crate::t_funk::macros::functions;

#[functions]
pub trait Traverse<F, P> {
    type Traverse;

    fn traverse(self, f: F) -> Self::Traverse;
}

