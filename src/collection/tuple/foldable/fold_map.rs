use crate::{
    closure::{Closure, OutputT},
    typeclass::{
        foldable::{FoldMap, Foldl, FoldlT},
        functor::{Fmap, FmapT},
        monoid::{Mempty, MemptyT},
        semigroup::MappendF,
    },
};

macro_rules! implementation {
    () => {};
    ($ident:ident $(, $rest:ident)*) => {
        impl<_Function, $ident $(, $rest)*> FoldMap<_Function> for ($ident $(, $rest)*,)
        where
            Self: Fmap<_Function> + Mempty,
            FmapT<Self, _Function>: Foldl<MappendF, MemptyT<OutputT<_Function, $ident>>>,
            _Function: Closure<$ident>,
            OutputT<_Function, $ident>: Mempty,
        {
            type FoldMap = FoldlT<FmapT<Self, _Function>, MappendF, MemptyT<OutputT<_Function, $ident>>>;

            fn fold_map(self, f: _Function) -> Self::FoldMap {
                self.fmap(f).foldl(MappendF::default(), OutputT::<_Function, $ident>::mempty())
            }
        }
    };
}

impl_tuple!(implementation => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::typeclass::{foldable::FoldMap, pointed::PointF, semigroup::Sum};

    #[test]
    fn test_tuple_fold_map() {
        let tup = (1, 2, 3);
        let fold_mapped = tup.fold_map(PointF::<Sum<_>>::default());
        assert_eq!(fold_mapped, Sum(1 + 2 + 3))
    }
}
