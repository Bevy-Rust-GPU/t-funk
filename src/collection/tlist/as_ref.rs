use crate::collection::{
    hlist::{AsRef, ToTuple},
    tlist::{Tuple, RefTuple},
};

pub trait AsTListRef<'a>: Tuple {
    type AsHListRef: RefTuple<'a>;
}

impl<'a, T> AsTListRef<'a> for T
where
    T: Tuple,
    T::HList: AsRef<'a>,
{
    type AsHListRef = <<T::HList as AsRef<'a>>::AsRef as ToTuple>::Tuple;
}
