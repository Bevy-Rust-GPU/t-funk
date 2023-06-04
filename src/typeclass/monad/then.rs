use crate::t_funk::macros::functions;

#[functions]
pub trait Then<F> {
    type Then;

    fn then(self, f: F) -> Self::Then;
}
