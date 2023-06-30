use t_funk_macros::{Applicative, Functor, Monad};

use crate::{
    closure::Closure,
    function::{AddT, DivT, MulT, SubT},
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
pub struct Add<T>(pub T);

impl<T, U> Closure<U> for Add<T>
where
    U: core::ops::Add<T>,
{
    type Output = AddT<U, T>;

    fn call(self, input: U) -> Self::Output {
        input + self.0
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
pub struct Sub<T>(pub T);

impl<T, U> Closure<U> for Sub<T>
where
    U: core::ops::Sub<T>,
{
    type Output = SubT<U, T>;

    fn call(self, input: U) -> Self::Output {
        input - self.0
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
pub struct Mul<T>(pub T);

impl<T, U> Closure<U> for Mul<T>
where
    U: core::ops::Mul<T>,
{
    type Output = MulT<U, T>;

    fn call(self, input: U) -> Self::Output {
        input * self.0
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
pub struct Div<T>(pub T);

impl<T, U> Closure<U> for Div<T>
where
    U: core::ops::Div<T>,
{
    type Output = DivT<U, T>;

    fn call(self, input: U) -> Self::Output {
        input / self.0
    }
}
