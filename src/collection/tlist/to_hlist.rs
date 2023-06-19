use crate::{
    collection::hlist::{HList, Nil},
    macros::functions,
};

/// Convert a flat tuple into a cons list.
/// ex. `(1, 2, 3, 4)` -> `(1, (2, (3, (4, ()))))`
#[functions]
pub trait ToHList {
    type HList: HList<Tuple = Self>;

    fn to_hlist(self) -> Self::HList;
}

impl ToHList for () {
    type HList = Nil;

    fn to_hlist(self) -> Self::HList {
        Nil
    }
}
