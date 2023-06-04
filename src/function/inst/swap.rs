use crate::{
    function::Function,
    macros::{arrow::Arrow, category::Category, Closure},
};

/// Swap the elements of a 2-tuple
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Swap;

impl<A, B> Function<(A, B)> for Swap {
    type Output = (B, A);

    fn call((a, b): (A, B)) -> Self::Output {
        (b, a)
    }
}
