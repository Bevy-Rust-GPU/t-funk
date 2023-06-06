use t_funk_macros::types;

use crate::macros::functions;

#[functions]
#[types]
pub trait Fold {
    type Fold;

    fn fold(self) -> Self::Fold;
}
