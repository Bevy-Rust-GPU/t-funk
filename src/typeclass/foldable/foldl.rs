use t_funk_macros::types;

use crate::macros::functions;

#[functions]
#[types]
pub trait Foldl<F, Z> {
    type Foldl;

    fn foldl(self, f: F, z: Z) -> Self::Foldl;
}

