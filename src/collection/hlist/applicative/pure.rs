use t_funk_macros::impl_adt;

use crate::{
    collection::hlist::{Cons, Nil},
    typeclass::applicative::Pure,
};

impl_adt! {
    impl<T, N, U> Pure<U> for Cons<T, N> | Nil {
        type Pure = Cons<U, Nil>;

        fn pure(unit: U) -> Self::Pure {
            Cons(unit, Nil)
        }
    }
}
