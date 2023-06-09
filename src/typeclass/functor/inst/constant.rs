use crate::{
    macros::{Copointed, Pointed},
    typeclass::functor::Fmap,
};

/// The Const functor, returns self when mapped
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct Const<T>(pub T);

impl<T, F> Fmap<F> for Const<T> {
    type Fmap = Const<T>;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}
