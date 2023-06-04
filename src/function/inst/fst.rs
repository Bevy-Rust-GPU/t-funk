use crate::{
    function::Function,
    macros::{arrow::Arrow, category::Category, Closure},
};

/// Extract the first component of a 2-tuple
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Fst;

impl<A, B> Function<(A, B)> for Fst {
    type Output = A;

    fn call((a, _): (A, B)) -> Self::Output {
        a
    }
}
