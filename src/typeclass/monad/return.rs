use core::marker::PhantomData;

use t_funk_macros::{types, PhantomClone, PhantomCopy, PhantomDefault};

use crate::{function::Function, macros::Closure};

#[types]
pub trait Return<T> {
    type Return;
    fn r#return(t: T) -> Self::Return;
}

#[derive(PhantomDefault, PhantomClone, PhantomCopy, Closure)]
pub struct ReturnF<T>(PhantomData<T>);

impl<T, U> Function<U> for ReturnF<T>
where
    T: Return<U>,
{
    type Output = ReturnT<T, U>;

    fn call(input: U) -> Self::Output {
        T::r#return(input)
    }
}

