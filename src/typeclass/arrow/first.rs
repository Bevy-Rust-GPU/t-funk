use t_funk_macros::types;

use crate::t_funk::{
    closure::Closure,
    macros::{arrow::Arrow, category::Category, functions, Copointed, Pointed},
};

#[functions]
#[types]
pub trait First {
    type First;

    fn first(self) -> Self::First;
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Category,
    Arrow,
)]
pub struct Firsted<F>(pub F);

impl<F, X, Y> Closure<(X, Y)> for Firsted<F>
where
    F: Closure<X>,
{
    type Output = (F::Output, Y);

    fn call(self, (x, y): (X, Y)) -> Self::Output {
        (self.0.call(x), y)
    }
}
