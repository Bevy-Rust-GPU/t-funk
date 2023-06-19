use crate::collection::hlist::{AsMut, ToTuple};

use super::{Tuple, MutTuple};

pub trait AsTListMut<'a>: Tuple {
    type AsHListMut: MutTuple<'a>;
}

impl<'a, T> AsTListMut<'a> for T
where
    T: Tuple,
    T::HList: AsMut<'a>,
{
    type AsHListMut = <<T::HList as AsMut<'a>>::AsMut as ToTuple>::Tuple;
}
