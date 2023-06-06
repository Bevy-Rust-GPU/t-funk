use core::marker::PhantomData;

use t_funk_macros::types;

use crate::t_funk::{
    function::Function,
    macros::{
        arrow::{Arr, First, Second},
        category::{Compose, Id},
        Closure,
    },
};

/// A type that can wrap a value
#[types]
pub trait Pointed
where
    Self: Sized,
{
    /// The wrapped value
    type Pointed;

    /// Wrap `Pointed` into `Self`
    fn point(unit: Self::Pointed) -> Self;
}

/// Pointed::point
#[derive(Closure, Id, Compose, Arr, First, Second)]
pub struct PointF<I>(PhantomData<I>);

impl<T> Default for PointF<T> {
    fn default() -> Self {
        PointF(PhantomData)
    }
}

impl<T> Clone for PointF<T> {
    fn clone(&self) -> Self {
        PointF(PhantomData)
    }
}

impl<I> Function<I::Pointed> for PointF<I>
where
    I: Pointed,
{
    type Output = I;

    fn call(input: I::Pointed) -> I {
        I::point(input)
    }
}
