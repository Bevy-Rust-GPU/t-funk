use t_funk_macros::{lift, Arrow, Category};

use crate::{closure::Closure, either::Either};

/// Conditional closure that takes two distinct types, and returns them via Either
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow,
)]
pub struct If<T, U>(pub T, pub U);

impl<T, U> Closure<bool> for If<T, U> {
    type Output = Either<T, U>;

    fn call(self, input: bool) -> Self::Output {
        if input {
            Either::Left(self.0)
        } else {
            Either::Right(self.1)
        }
    }
}

#[lift]
pub fn make_if<A, B>(a: A, b: B) -> If<A, B> {
    If(a, b)
}

