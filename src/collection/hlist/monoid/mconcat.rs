use crate::{
    collection::hlist::{Cons, Nil},
    typeclass::{
        foldable::{Foldl, FoldlT},
        monoid::{Mconcat, Mempty, MemptyT},
        semigroup::MappendF,
    },
};

impl<Head, Tail> Mconcat for Cons<Head, Tail>
where
    Self: Mempty + Foldl<MappendF, <Self as Mempty>::Mempty>,
{
    type Mconcat = FoldlT<Self, MappendF, MemptyT<Self>>;

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
