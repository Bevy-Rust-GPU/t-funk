use crate::{
    collection::hlist::{Cons, Nil},
    typeclass::applicative::Pure,
};

impl<T, N> Pure for Cons<T, N>
where
    N: Pure,
{
    type Pure<U> = Cons<U, Nil>;

    fn pure<U>(unit: U) -> Self::Pure<U> {
        Cons(unit, Nil)
    }
}

impl Pure for Nil {
    type Pure<T> = Cons<T, Nil>;

    fn pure<T>(unit: T) -> Self::Pure<T> {
        Cons(unit, Nil)
    }
}
