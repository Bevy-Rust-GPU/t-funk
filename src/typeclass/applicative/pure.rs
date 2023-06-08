use core::marker::PhantomData;

use t_funk_macros::types;

use crate::{function::Function, macros::Closure};

#[types]
pub trait Pure<T> {
    type Pure;
    fn pure(t: T) -> Self::Pure;
}

#[derive(Closure)]
pub struct PureF<T>(PhantomData<T>);

impl<T> Default for PureF<T> {
    fn default() -> Self {
        PureF(PhantomData)
    }
}

impl<T> Clone for PureF<T> {
    fn clone(&self) -> Self {
        PureF(PhantomData)
    }
}

impl<T, U> Function<U> for PureF<T>
where
    T: Pure<U>,
{
    type Output = T::Pure;

    fn call(input: U) -> Self::Output {
        T::pure(input)
    }
}
