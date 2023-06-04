use type_fields::t_funk::{
    closure::Closure,
    macros::{applicative::Applicative, functor::Functor, monad::Monad},
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
pub struct Const<T>(pub T);

impl<T, U> Closure<U> for Const<T> {
    type Output = T;

    fn call(self, _: U) -> Self::Output {
        self.0
    }
}
