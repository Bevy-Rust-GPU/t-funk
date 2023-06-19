use crate::collection::{
    hlist::{PopBack as HListPopBack, ToTuple},
    tlist::Tuple,
};

pub trait PopBack: Tuple {
    type PopBack: Tuple;

    fn pop_back(self) -> Self::PopBack;
}

impl<T> PopBack for T
where
    T: Tuple,
    T::HList: HListPopBack,
    <T::HList as HListPopBack>::PopBack: ToTuple,
{
    type PopBack = <<T::HList as HListPopBack>::PopBack as ToTuple>::Tuple;

    fn pop_back(self) -> Self::PopBack {
        self.to_hlist().pop_back().to_tuple()
    }
}

#[cfg(test)]
mod test {
    use crate::collection::tlist::pop_back::PopBack;

    #[test]
    fn test_tuple_pop_back() {
        let list: (usize, f32, &str) = (1, 2.0, "three");
        let list: (usize, f32) = list.pop_back();
        let list: (usize,) = list.pop_back();
        let list: () = list.pop_back();
        assert_eq!((), list);
    }
}
