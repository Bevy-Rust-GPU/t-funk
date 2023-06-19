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
    Head: Mempty,
    Self: Foldl<MappendF, MemptyT<Head>>,
{
    type Mconcat = FoldlT<Self, MappendF, MemptyT<Head>>;

    fn mconcat(self) -> Self::Mconcat {
        self.foldl(MappendF::default(), Head::mempty())
    }
}

impl Mconcat for Nil {
    type Mconcat = Self;

    fn mconcat(self) -> Self::Mconcat {
        self
    }
}
