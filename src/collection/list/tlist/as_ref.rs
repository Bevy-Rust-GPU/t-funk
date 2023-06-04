use crate::t_funk::collection::{
    hlist::{AsRef, ToTList},
    tlist::{TList, TListRef},
};

pub trait AsTListRef<'a>: TList {
    type AsHListRef: TListRef<'a>;
}

impl<'a, T> AsTListRef<'a> for T
where
    T: TList,
    T::HList: AsRef<'a>,
{
    type AsHListRef = <<T::HList as AsRef<'a>>::AsRef as ToTList>::TList;
}
