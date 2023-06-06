use t_funk_macros::lift;

use crate::{
    closure::Closure,
    either::Either,
    macros::{arrow::Arrow, category::Category},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
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
