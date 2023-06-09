use crate::collection::{
    hlist::{HListMut, Nil},
    tlist::AsHListRef,
};

/// Mutably borrow a HList from a flat tuple.
/// ex. `&mut (1, 2, 3, 4)` -> `(&mut 1, (&mut 2, (&mut 3, (&mut 4, ()))))`
pub trait AsHListMut: AsHListRef {
    type HListMut<'a>: HListMut<'a>
    where
        Self: 'a;

    fn as_hlist_mut<'a>(&'a mut self) -> Self::HListMut<'a>;
}

impl AsHListMut for () {
    type HListMut<'a> = Nil;

    fn as_hlist_mut<'a>(&'a mut self) -> Self::HListMut<'a> {
        Nil
    }
}
