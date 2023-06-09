use crate::{
    macros::{
        applicative::Applicative, foldable::Foldable, functor::Functor, monad::Monad,
        Copointed, Pointed,
    },
    typeclass::semigroup::Mappend,
};

/// A `Semigroup` wrapper that can append with OR semantics.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Functor,
    Applicative,
    Monad,
    Foldable,
)]
pub struct Any<T>(pub T);

impl<T, U> Mappend<Any<U>> for Any<T>
where
    T: core::ops::BitOr<U>,
{
    type Mappend = Any<T::Output>;

    fn mappend(self, t: Any<U>) -> Self::Mappend {
        Any(self.0 | t.0)
    }
}

#[cfg(test)]
mod test {
    use crate::typeclass::semigroup::{Any, Mappend};

    #[test]
    fn test_any() {
        assert_eq!(Any(true), Any(true).mappend(Any(false)).mappend(Any(true)))
    }
}
