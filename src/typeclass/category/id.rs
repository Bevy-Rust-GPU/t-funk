use crate::t_funk::macros::functions;

#[functions]
pub trait Id {
    type Id;

    fn id() -> Self::Id;
}
