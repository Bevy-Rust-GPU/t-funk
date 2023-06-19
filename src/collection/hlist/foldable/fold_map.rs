use crate::{
    closure::{Closure, OutputT},
    collection::hlist::Cons,
    typeclass::{
        foldable::{FoldMap, Foldl, FoldlT},
        functor::{Fmap, FmapT},
        monoid::{Mempty, MemptyT},
        semigroup::MappendF,
    },
};

impl<Head, Tail, _Function> FoldMap<_Function> for Cons<Head, Tail>
where
    Self: Fmap<_Function> + Mempty,
    FmapT<Self, _Function>: Foldl<MappendF, MemptyT<OutputT<_Function, Head>>>,
    _Function: Closure<Head>,
    OutputT<_Function, Head>: Mempty,
{
    type FoldMap =
        FoldlT<FmapT<Cons<Head, Tail>, _Function>, MappendF, MemptyT<OutputT<_Function, Head>>>;

    fn fold_map(self, f: _Function) -> Self::FoldMap {
        self.fmap(f)
            .foldl(MappendF::default(), OutputT::<_Function, Head>::mempty())
    }
}
