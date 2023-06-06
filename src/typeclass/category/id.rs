use t_funk_macros::types;

use crate::macros::functions;

#[functions]
#[types]
pub trait Id {
    type Id;

    fn id() -> Self::Id;
}
