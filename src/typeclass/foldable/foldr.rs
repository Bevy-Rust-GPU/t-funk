use t_funk_macros::types;

use crate::macros::functions;

#[functions]
#[types]
pub trait Foldr<F, Z> {
    type Foldr;

    fn foldr(self, f: F, z: Z) -> Self::Foldr;
}

