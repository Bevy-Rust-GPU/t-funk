use t_funk_macros::{functions, types};

#[types]
#[functions]
pub trait PushFront<T> {
    type PushFront;

    fn push_front(self, t: T) -> Self::PushFront;
}

macro_rules! implementation {
    ($($ident:ident),*) => {
        impl<_Type, $($ident,)*> PushFront<_Type> for ($($ident,)*)
        {
            type PushFront = (_Type, $($ident),*);

            #[allow(non_snake_case)]
            fn push_front(self, t: _Type) -> Self::PushFront {
                let ($($ident,)*) = self;
                (t, $($ident),*)
            }
        }
    };
}

impl_tuple!(implementation => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::collection::tuple::PushFront;

    #[test]
    fn test_tuple_mappend() {
        let tup = ();
        let tup = tup.push_front(1);
        let tup = tup.push_front(2.0);
        let tup = tup.push_front('3');
        let tup = tup.push_front("4");
        assert_eq!(tup, ("4", '3', 2.0, 1));
    }
}
