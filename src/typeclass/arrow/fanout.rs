use t_funk_macros::types;

use crate::t_funk::{
    closure::Closure,
    macros::{arrow::Arrow, category::Category, functions, Copointed, Pointed},
};

#[functions]
#[types]
pub trait Fanout<F> {
    type Fanout;

    fn fanout(self, f: F) -> Self::Fanout;
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
pub struct Fanouted<F, G>(pub F, pub G);

impl<F, G, X> Closure<X> for Fanouted<F, G>
where
    F: Closure<X>,
    G: Closure<X>,
    X: Clone,
{
    type Output = (F::Output, G::Output);

    fn call(self, x: X) -> Self::Output {
        (self.0.call(x.clone()), self.1.call(x))
    }
}
