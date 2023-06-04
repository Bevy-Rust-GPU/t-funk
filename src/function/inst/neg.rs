use crate::{
    function::Function,
    macros::{arrow::Arrow, category::Category, Closure},
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Neg;

impl<T> Function<T> for Neg
where
    T: core::ops::Neg,
{
    type Output = T::Output;

    fn call(input: T) -> Self::Output {
        input.neg()
    }
}
