use crate::{collection::hlist::ToTuple, typeclass::functor::Fmap};

use super::Tuple;

pub trait Map<F>: Tuple {
    type TupleMap: Tuple;

    fn map(self, f: F) -> Self::TupleMap;
}

impl<T, F> Map<F> for T
where
    T: Tuple,
    T::HList: Fmap<F>,
    <T::HList as Fmap<F>>::Fmap: ToTuple,
{
    type TupleMap = <<T::HList as Fmap<F>>::Fmap as ToTuple>::Tuple;

    fn map(self, f: F) -> Self::TupleMap {
        self.to_hlist().fmap(f).to_tuple()
    }
}

#[cfg(test)]
mod test {
    use crate::{collection::tuple::Map, function::Function, macros::Closure};

    #[test]
    fn test_tuple_map() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
        struct Mul2;

        impl Function<u32> for Mul2 {
            type Output = u32;

            fn call(input: u32) -> u32 {
                input * 2
            }
        }

        impl Function<f32> for Mul2 {
            type Output = f32;

            fn call(input: f32) -> Self::Output {
                input * 2.0
            }
        }

        let list = (1, 2.0, 3);
        let list = list.map(Mul2);
        assert_eq!(list, (2, 4.0, 6))
    }
}
