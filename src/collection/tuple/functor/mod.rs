mod fmap;
mod replace {
    use crate::{
        closure::{Curry2, Curry2A},
        function::Const,
        typeclass::{
            functor::Replace,
            functor::{Fmap, FmapT},
        },
    };

    macro_rules! implementation {
    ($($ident:ident),*) => {
        impl<$($ident,)* _Type> Replace<_Type> for ($($ident,)*)
        where
            Self: Fmap<Curry2A<Const, _Type>>
        {
            type Replace = FmapT<Self, Curry2A<Const, _Type>>;

            #[allow(non_snake_case)]
            fn replace(self, t: _Type) -> Self::Replace {
                self.fmap(Const.prefix2(t))
            }
        }
    };
}

    impl_tuple!(implementation => A, B, C, D, E, F, G, H, I, J, K, L, M);

    #[cfg(test)]
    mod test {
        use crate::typeclass::functor::Replace;

        #[test]
        fn test_tuple_replace() {
            let tup = (1, 2.0, '3');
            let replaced = tup.replace("4");
            assert_eq!(replaced, ("4", "4", "4"))
        }
    }
}
