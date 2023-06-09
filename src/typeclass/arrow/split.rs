use t_funk_macros::types;

use crate::t_funk::macros::{arrow::Arrow, category::Category, functions, Copointed, Pointed};

use crate::t_funk::closure::Closure;

#[functions]
#[types]
pub trait Split<G>: Sized {
    type Split;

    fn split(self, g: G) -> Self::Split;
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
pub struct Splitted<F, G>(pub F, pub G);

impl<F, G, X, Y> Closure<(X, Y)> for Splitted<F, G>
where
    F: Closure<X>,
    G: Closure<Y>,
{
    type Output = (F::Output, G::Output);

    fn call(self, (x, y): (X, Y)) -> Self::Output {
        (self.0.call(x), self.1.call(y))
    }
}
