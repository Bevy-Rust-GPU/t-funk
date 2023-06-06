use t_funk_macros::types;

use crate::{
    closure::Closure,
    macros::{arrow::Arrow, category::Category, functions, Copointed, Pointed},
};

/// Right-to-left composition
/// (.)
#[functions]
#[types]
pub trait Compose<F>: Sized {
    type Compose;
    type ComposeL;
    fn compose(self, f: F) -> Self::Compose;
    fn compose_l(self, f: F) -> Self::ComposeL;
}

impl<T, F> Compose<F> for T {
    type Compose = Composed<T, F>;
    type ComposeL = Composed<F, T>;

    /// Compose `F(A) -> B` with `F(B) -> C` to produce `F(A) -> C`
    fn compose(self, f: F) -> Self::Compose {
        Composed(self, f)
    }

    fn compose_l(self, f: F) -> Self::ComposeL {
        Composed(f, self)
    }
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
pub struct Composed<F1, F2>(pub F1, pub F2);

impl<F1, F2, A> Closure<A> for Composed<F1, F2>
where
    F2: Closure<A>,
    F1: Closure<F2::Output>,
{
    type Output = F1::Output;

    fn call(self, a: A) -> Self::Output {
        self.0.call(self.1.call(a))
    }
}
