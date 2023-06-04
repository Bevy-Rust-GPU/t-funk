use crate::t_funk::macros::functions;

#[functions]
pub trait Replace<T> {
    type Replace;

    fn replace(self, t: T) -> Self::Replace;
}

