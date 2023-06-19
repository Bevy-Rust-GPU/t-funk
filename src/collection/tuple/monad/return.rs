use crate::typeclass::monad::Return;

impl<U> Return<U> for () {
    type Return = (U,);

    fn r#return(t: U) -> Self::Return {
        (t,)
    }
}

macro_rules! impl_apply {
    ($($ident:ident),*) => {
        #[allow(unused_parens)]
        impl<_Type, $($ident),*> Return<_Type> for ($($ident,)*)
        {
            type Return = (_Type,);

            fn r#return(unit: _Type) -> Self::Return {
                (unit,)
            }
        }
    };
}

impl_tuple!(impl_apply => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::typeclass::monad::Return;

    #[test]
    fn test_tuple_pure() {
        let foo = <() as Return<_>>::r#return(1);
        assert_eq!(foo, (1,));

        let bar = <((),) as Return<_>>::r#return(2.0);
        assert_eq!(bar, (2.0,));

        let baz = <((), ()) as Return<_>>::r#return('3');
        assert_eq!(baz, ('3',));

        let decafisbad = <((), (), ()) as Return<_>>::r#return("4");
        assert_eq!(decafisbad, ("4",));
    }
}

