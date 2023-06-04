use crate::t_funk::macros::functions;

#[functions]
pub trait Fold {
    type Fold;

    fn fold(self) -> Self::Fold;
}
