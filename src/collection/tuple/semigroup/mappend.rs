use crate::typeclass::semigroup::Mappend;

impl<T> Mappend<T> for () {
    type Mappend = (T,);

    fn mappend(self, t: T) -> Self::Mappend {
        (t,)
    }
}

macro_rules! implementation {
    ($($ident:ident),*) => {
        impl<_Type, $($ident,)*> Mappend<_Type> for ($($ident,)*)
        {
            type Mappend = ($($ident,)* _Type);

            #[allow(non_snake_case)]
            fn mappend(self, t: _Type) -> Self::Mappend {
                let ($($ident,)*) = self;
                ($($ident,)* t)
            }
        }
    };
}

impl_tuple!(implementation => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::typeclass::semigroup::Mappend;

    #[test]
    fn test_tuple_mappend() {
        let tup = ();
        let tup = tup.mappend(1);
        let tup = tup.mappend(2.0);
        let tup = tup.mappend('3');
        let tup = tup.mappend("4");
        assert_eq!(tup, (1, 2.0, '3', "4"));
    }
}

