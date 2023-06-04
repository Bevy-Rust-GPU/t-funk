use crate::t_funk::macros::functions;

#[functions]
pub trait Mconcat: Sized {
    type Mconcat;

    fn mconcat(self) -> Self::Mconcat;
}

