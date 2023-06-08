use t_funk_macros::impl_adt;

use crate::{
    collection::hlist::{Cons, Nil},
    typeclass::{
        applicative::{Pure, PureT},
        monad::Return,
    },
};

impl_adt! {
    impl<T, N, U> Return<U> for Cons<T, N> | Nil where Self: Pure<U> {
        type Return = PureT<Self, U>;

        fn r#return(unit: U) -> Self::Return {
            Self::pure(unit)
        }
    }
}

