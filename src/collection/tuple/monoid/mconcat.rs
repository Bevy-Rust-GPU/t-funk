use crate::typeclass::{
    foldable::{Foldl, FoldlT},
    monoid::{Mconcat, Mempty, MemptyT},
    semigroup::MappendF,
};

macro_rules! implementation {
    () => {};
    ($ident:ident $(, $rest:ident)*) => {
        impl<$ident, $($rest,)*> Mconcat for ($ident, $($rest,)*)
        where
            $ident: Mempty,
            Self: Foldl<MappendF, MemptyT<$ident>>
        {
            type Mconcat = FoldlT<Self, MappendF, MemptyT<$ident>>;

            fn mconcat(self) -> Self::Mconcat {
                self.foldl(MappendF::default(), $ident::mempty())
            }
        }
    };
}

impl_tuple!(implementation => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::typeclass::{monoid::Mconcat, semigroup::Sum};

    #[test]
    fn test_tuple_mconcat() {
        let tup = (Sum(1), Sum(2), Sum(3));
        let mconcatted = tup.mconcat();
        assert_eq!(mconcatted, Sum(1 + 2 + 3));
    }
}
