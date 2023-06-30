use crate::typeclass::semigroup::Mappend;

macro_rules! impl_inner {
    ($($ident:ident),* => $($ident2:ident),*) => {
        impl<$($ident,)* $($ident2,)*> Mappend<($($ident2,)*)> for ($($ident,)*)
        {
            type Mappend = ($($ident,)* $($ident2,)*);

            #[allow(non_snake_case)]
            fn mappend(self, ($($ident2,)*): ($($ident2,)*)) -> Self::Mappend {
                let ($($ident,)*) = self;
                ($($ident,)* $($ident2,)*)
            }
        }
    }
}

macro_rules! implementation {
    ($($ident:ident),*) => {
        impl_inner!($($ident),* =>);
        impl_inner!($($ident),* => _A);
        impl_inner!($($ident),* => _A, _B);
        impl_inner!($($ident),* => _A, _B, _C);
        impl_inner!($($ident),* => _A, _B, _C, _D);
        impl_inner!($($ident),* => _A, _B, _C, _D, _E);
        impl_inner!($($ident),* => _A, _B, _C, _D, _E, _F);
        impl_inner!($($ident),* => _A, _B, _C, _D, _E, _F, _G);
        impl_inner!($($ident),* => _A, _B, _C, _D, _E, _F, _G, _H);
        impl_inner!($($ident),* => _A, _B, _C, _D, _E, _F, _G, _H, _I);
        impl_inner!($($ident),* => _A, _B, _C, _D, _E, _F, _G, _H, _I, _J);
        impl_inner!($($ident),* => _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K);
        impl_inner!($($ident),* => _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K, _L);
        impl_inner!($($ident),* => _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K, _L, _M);
    };
}

impl_tuple!(implementation => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::typeclass::semigroup::Mappend;

    #[test]
    fn test_tuple_mappend() {
        let tup = ();
        let tup = tup.mappend((1,));
        let tup = tup.mappend((2.0,));
        let tup = tup.mappend(('3',));
        let tup = tup.mappend(("4",));
        assert_eq!(tup, (1, 2.0, '3', "4"));
    }
}
