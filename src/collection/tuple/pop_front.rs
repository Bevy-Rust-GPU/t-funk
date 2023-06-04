use crate::collection::{
    hlist::{PopFront as HListPopFront, ToTuple},
    tuple::Tuple,
};

pub trait PopFront: Tuple {
    type PopFront: Tuple;

    fn pop_front(self) -> Self::PopFront;
}

impl<T> PopFront for T
where
    T: Tuple,
    T::HList: HListPopFront,
    <T::HList as HListPopFront>::PopFront: ToTuple,
{
    type PopFront = <<T::HList as HListPopFront>::PopFront as ToTuple>::Tuple;

    fn pop_front(self) -> Self::PopFront {
        self.to_hlist().pop_front().to_tuple()
    }
}

#[cfg(test)]
mod test {
    use crate::collection::tuple::pop_front::PopFront;

    #[test]
    fn test_tuple_pop_back() {
        let list: (usize, f32, &str) = (1, 2.0, "three");
        let list: (f32, &str) = list.pop_front();
        let list: (&str,) = list.pop_front();
        let list: () = list.pop_front();
        assert_eq!((), list);
    }
}
