use crate::{
    function::Function,
    macros::{arrow::Arrow, category::Category, Closure},
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Add;

impl<A, B> Function<(A, B)> for Add
where
    A: core::ops::Add<B>,
{
    type Output = A::Output;

    fn call((a, b): (A, B)) -> Self::Output {
        a + b
    }
}
