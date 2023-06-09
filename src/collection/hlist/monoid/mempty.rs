use crate::{
    collection::hlist::{Cons, Nil},
    typeclass::monoid::Mempty,
};

impl<Head, Tail> Mempty for Cons<Head, Tail> {
    type Mempty = Nil;

    fn mempty() -> Self::Mempty {
        Nil
    }
}

impl Mempty for Nil {
    type Mempty = Nil;

    fn mempty() -> Self::Mempty {
        Nil
    }
}
