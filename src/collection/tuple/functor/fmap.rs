use crate::{
    closure::{Closure, OutputT},
    typeclass::functor::Fmap,
};

macro_rules! implementation {
    ($($ident:ident),*) => {
        impl<$($ident,)* _Function> Fmap<_Function> for ($($ident,)*)
        where
            _Function: Clone + $(
                Closure<$ident> +
            )*
        {
            type Fmap = ($(OutputT<_Function, $ident>,)*);

            #[allow(non_snake_case)]
            fn fmap(self, f: _Function) -> Self::Fmap {
                let ($($ident,)*) = self;
                ($(f.clone().call($ident),)*)
            }
        }
    };
}

impl_tuple!(implementation => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::{closure::Curry2, function::Mul, typeclass::functor::Fmap};

    #[test]
    fn test_tuple_fmap() {
        let tup = (1, 2, 3);
        let fmapped = tup.fmap(Mul.suffix2(2));
        assert_eq!(fmapped, (2, 4, 6))
    }
}

