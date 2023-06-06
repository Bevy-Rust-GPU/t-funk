use crate::macros::{functions, types};

#[functions]
#[types]
pub trait Then<F> {
    type Then;

    fn then(self, f: F) -> Self::Then;
}
