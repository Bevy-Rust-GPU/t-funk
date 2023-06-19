use crate::typeclass::monoid::Mempty;

macro_rules! implementation {
    ($($ident:ident),*) => {
        impl<$($ident,)*> Mempty for ($($ident,)*)
        {
            type Mempty = ();

            fn mempty() -> Self::Mempty {
                ()
            }
        }
    };
}

impl_tuple!(implementation => A, B, C, D, E, F, G, H, I, J, K, L, M);

