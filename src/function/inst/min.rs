use crate::{
    function::Function,
    macros::{arrow::Arrow, category::Category, Closure},
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Min;

impl Function<(f32, f32)> for Min {
    type Output = f32;

    fn call((l, r): (f32, f32)) -> Self::Output {
        l.min(r)
    }
}
