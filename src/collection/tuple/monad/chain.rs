use crate::typeclass::{
    functor::{Fmap, FmapT},
    monad::Chain,
    monoid::{Mconcat, MconcatT},
};

macro_rules! implementation {
    ($($ident:ident),*) => {
        impl<$($ident,)* _Function> Chain<_Function> for ($($ident,)*)
        where
            Self: Fmap<_Function>,
            FmapT<Self, _Function>: Mconcat,
        {
            type Chain = MconcatT<FmapT<Self, _Function>>;

            #[allow(non_snake_case)]
            fn chain(self, f: _Function) -> Self::Chain {
                self.fmap(f).mconcat()
            }
        }
    };
}

impl_tuple!(implementation => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use t_funk_macros::lift;

    use crate::{
        collection::hlist::{Cons, Nil},
        typeclass::{copointed::CopointF, functor::Fmap, monad::Chain},
    };

    #[test]
    fn test_tuple_chain() {
        let tuple = (1, 2.0, '3');

        #[lift]
        fn make_list<T>(t: T) -> Cons<T, Nil> {
            Cons(t, Nil)
        }

        #[lift]
        fn make_tuple<T>(t: T) -> (T,) {
            (t,)
        }

        let chained = tuple.chain(MakeList);
        let chained = chained.chain(MakeTuple);
        let chained = chained.fmap(CopointF);
        assert_eq!(chained, tuple)
    }
}
