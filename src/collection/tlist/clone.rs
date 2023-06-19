use crate::collection::{
    hlist::{HListClone, ToTuple},
    tlist::{Tuple, RefTuple},
};

pub trait TListClone<'a>: RefTuple<'a> {
    type Cloned: Tuple;

    fn tlist_clone(self) -> Self::Cloned;
}

impl<'a, T> TListClone<'a> for T
where
    T: RefTuple<'a>,
    T::HList: HListClone<'a>,
{
    type Cloned = <<T::HList as HListClone<'a>>::Cloned as ToTuple>::Tuple;

    fn tlist_clone(self) -> Self::Cloned {
        self.to_hlist().hlist_clone().to_tuple()
    }
}
