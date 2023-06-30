use crate::typeclass::applicative::Pure;

macro_rules! impl_apply {
    ($($ident:ident),*) => {
        #[allow(unused_parens)]
        impl<_Type, $($ident),*> Pure<_Type> for ($($ident,)*)
        {
            type Pure = (_Type,);

            fn pure(unit: _Type) -> Self::Pure {
                (unit,)
            }
        }
    };
}

impl_tuple!(impl_apply => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::typeclass::applicative::Pure;

    #[test]
    fn test_tuple_pure() {
        let foo = <() as Pure<_>>::pure(1);
        assert_eq!(foo, (1,));

        let bar = <((),) as Pure<_>>::pure(2.0);
        assert_eq!(bar, (2.0,));

        let baz = <((), ()) as Pure<_>>::pure('3');
        assert_eq!(baz, ('3',));

        let decafisbad = <((), (), ()) as Pure<_>>::pure("4");
        assert_eq!(decafisbad, ("4",));
    }
}

