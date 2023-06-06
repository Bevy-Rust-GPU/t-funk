use t_funk_macros::types;

use crate::macros::functions;

#[functions]
#[types]
pub trait Replace<T> {
    type Replace;

    fn replace(self, t: T) -> Self::Replace;
}
