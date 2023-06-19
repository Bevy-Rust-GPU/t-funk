use crate::{
    closure::{Closure, OutputT},
    typeclass::applicative::Apply,
};

macro_rules! impl_apply {
    ($($ident:ident),*) => {
        #[allow(unused_parens)]
        impl<_Type, $($ident),*> Apply<_Type> for ($($ident,)*)
        where
            _Type: Clone,
            $(
                $ident: Closure<_Type>
            ),*
        {
            type Apply = ($(OutputT<$ident, _Type>,)*);

            #[allow(non_snake_case)]
            fn apply(self, t: _Type) -> Self::Apply {
                let ($($ident,)*) = self;
                ($($ident.call(t.clone()),)*)
            }
        }
    };
}

impl_tuple!(impl_apply => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::{
        closure::Curry2,
        function::{Add, Mul, Sub},
        typeclass::applicative::Apply,
    };

    #[test]
    fn test_tuple_apply() {
        let tup = (Add.suffix2(2), Sub.suffix2(2), Mul.suffix2(2));
        let applied = tup.apply(4);
        assert_eq!(applied, (4 + 2, 4 - 2, 4 * 2))
    }
}

