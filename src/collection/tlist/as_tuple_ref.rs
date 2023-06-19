use crate::collection::{
    hlist::ToTuple,
    tlist::{AsHListRef, Tuple},
};

pub trait AsTupleRef: AsHListRef {
    type TupleRef<'a>: Tuple
    where
        Self: 'a;

    fn as_ref<'a>(&'a self) -> Self::TupleRef<'a>;
}

impl<T> AsTupleRef for T
where
    T: AsHListRef,
{
    type TupleRef<'a> = <T::HListRef<'a> as ToTuple>::Tuple where T: 'a;

    fn as_ref<'a>(&'a self) -> Self::TupleRef<'a> {
        self.as_hlist_ref().to_tuple()
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::tlist::AsTupleRef;

    #[test]
    fn test_tuple_ref() {
        let list = (1, 2.0, "three").as_ref();
        assert_eq!((&1, &2.0, &"three"), list);
    }
}
