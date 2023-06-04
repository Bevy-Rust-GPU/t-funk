use crate::collection::hlist::{GetsImpl as HListGetsImpl, Paths, ToTuple};

use super::Tuple;

pub trait GetsImpl<In, P>: Tuple {
    fn gets_impl(self) -> In;
}

impl<T, P, In> GetsImpl<In, P> for T
where
    T: Tuple,
    In: Tuple,
    T::HList: HListGetsImpl<In::HList, P>,
    P: Paths,
{
    fn gets_impl(self) -> In {
        self.to_hlist().gets_impl().to_tuple()
    }
}

impl<T> GetsImpl<(), ()> for T
where
    T: Tuple,
{
    fn gets_impl(self) -> () {
        ()
    }
}

pub trait Gets<P>: Tuple {
    fn gets<T>(self) -> T
    where
        Self: GetsImpl<T, P>;
}

impl<T, P> Gets<P> for T
where
    T: Tuple,
{
    fn gets<In>(self) -> In
    where
        Self: GetsImpl<In, P>,
    {
        self.gets_impl()
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::tuple::GetsImpl;

    #[test]
    fn test_tuple_gets() {
        let list = (1, 2.0, "three");
        let (string, float, int) = GetsImpl::<(&str, f32, usize), _>::gets_impl(list);
        assert_eq!((1, 2.0, "three"), (int, float, string));
    }
}
