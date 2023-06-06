use t_funk_macros::types;

use crate::macros::functions;

#[functions]
#[types]
pub trait FoldMap<F> {
    type FoldMap;

    fn fold_map(self, f: F) -> Self::FoldMap;
}
