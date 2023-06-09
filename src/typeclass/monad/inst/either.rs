use crate::macros::{
    applicative::Applicative, foldable::Foldable, functor::Functor, monad::Monad, 
    semigroup::Semigroup, Copointed, Pointed,
};

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
    Semigroup,
    Foldable,
)]
pub struct Left<T>(pub T);

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
    Semigroup,
    Foldable,
)]
pub struct Right<T>(pub T);

#[cfg(test)]
mod test {
    use crate::{
        closure::Curry2,
        function::{Add, Mul},
        typeclass::functor::{test_functor_laws, Fmap},
    };

    use super::{Left, Right};

    #[test]
    fn test_either() {
        let left = Left(2);
        let mapped = left.fmap(Add.prefix2(1));
        assert_eq!(mapped, Left(3));

        let left = Right(2);
        let mapped = left.fmap(Add.prefix2(1));
        assert_eq!(mapped, Right(3));
    }

    #[test]
    fn test_functor_laws_maybe() {
        test_functor_laws(Left(1), Add.prefix2(2), Mul.prefix2(2));
        test_functor_laws(Right(1), Add.prefix2(2), Mul.prefix2(2));
    }
}
