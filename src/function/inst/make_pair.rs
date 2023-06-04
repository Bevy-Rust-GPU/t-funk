use crate::t_funk::{
    function::Function,
    macros::{arrow::Arrow, category::Category, Closure},
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct MakePair;

impl<T> Function<T> for MakePair
where
    T: Clone,
{
    type Output = (T, T);

    fn call(t: T) -> Self::Output {
        (t.clone(), t)
    }
}
