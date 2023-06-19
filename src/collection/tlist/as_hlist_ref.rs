use crate::collection::{
    hlist::{HListRef, Nil},
    tlist::Tuple,
};

/// Immutably borrow a HList from a flat tuple.
/// ex. `&(1, 2, 3, 4)` -> `(&1, (&2, (&3, (&4, ()))))`
pub trait AsHListRef: Tuple {
    type HListRef<'a>: HListRef<'a>
    where
        Self: 'a;

    fn as_hlist_ref<'a>(&'a self) -> Self::HListRef<'a>;
}

impl AsHListRef for () {
    type HListRef<'a> = Nil;

    fn as_hlist_ref<'a>(&'a self) -> Self::HListRef<'a> {
        Nil
    }
}
