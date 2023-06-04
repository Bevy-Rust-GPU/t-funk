use crate::{
    collection::hlist::{Cons, Nil},
    typeclass::{
        foldable::Foldl,
        monoid::{Mconcat, Mempty},
        semigroup::MappendF,
    },
};

impl<Head, Tail> Mconcat for Cons<Head, Tail>
where
    Self: Mempty + Foldl<MappendF, <Self as Mempty>::Mempty>,
{
    type Mconcat = <Cons<Head, Tail> as Foldl<MappendF, <Self as Mempty>::Mempty>>::Foldl;

    fn mconcat(self) -> Self::Mconcat {
        self.foldl(MappendF::default(), Self::mempty())
    }
}

impl Mconcat for Nil {
    type Mconcat = Self;

    fn mconcat(self) -> Self::Mconcat {
        self
    }
}
