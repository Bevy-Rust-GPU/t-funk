use crate::macros::{
    applicative::Applicative, foldable::Foldable, functor::Functor, monad::Monad, Copointed,
    Pointed,
};

use crate::typeclass::monad::Return;
use crate::typeclass::{
    applicative::{Apply, Pure},
    copointed::Copointed,
    foldable::{Fold, FoldMap, Foldl, Foldr},
    functor::{Fmap, Replace},
    monad::{Chain, Then},
    monoid::{Mconcat, Mempty},
    pointed::Pointed,
    semigroup::Mappend,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nothing;

impl Pointed for Nothing {
    type Pointed = Nothing;

    fn point(_: Self::Pointed) -> Self {
        Nothing
    }
}

impl Copointed for Nothing {
    type Copointed = Nothing;

    fn copoint(self) -> Self::Copointed {
        Nothing
    }
}

impl<F> Fmap<F> for Nothing {
    type Fmap = Nothing;

    fn fmap(self, _: F) -> Self::Fmap {
        Nothing
    }
}

impl<T> Replace<T> for Nothing {
    type Replace = Nothing;

    fn replace(self, _: T) -> Self::Replace {
        Nothing
    }
}

impl<T> Pure<T> for Nothing {
    type Pure = Just<T>;

    fn pure(t: T) -> Self::Pure {
        Just(t)
    }
}

impl<T> Apply<T> for Nothing {
    type Apply = T;

    fn apply(self, a: T) -> Self::Apply {
        a
    }
}

impl<F> Chain<F> for Nothing {
    type Chain = Nothing;

    fn chain(self, _: F) -> Self::Chain {
        Nothing
    }
}

impl<T> Return<T> for Nothing {
    type Return = Just<T>;

    fn r#return(t: T) -> Self::Return {
        Just(t)
    }
}

impl<F> Then<F> for Nothing {
    type Then = Nothing;

    fn then(self, _: F) -> Self::Then {
        Nothing
    }
}

impl Mempty for Nothing {
    type Mempty = Nothing;

    fn mempty() -> Self::Mempty {
        Nothing
    }
}

impl Mappend<Nothing> for Nothing {
    type Mappend = Nothing;

    fn mappend(self, _: Nothing) -> Self::Mappend {
        self
    }
}

impl<T> Mappend<Just<T>> for Nothing {
    type Mappend = Just<T>;

    fn mappend(self, t: Just<T>) -> Self::Mappend {
        t
    }
}

impl Mconcat for Nothing {
    type Mconcat = Nothing;

    fn mconcat(self) -> Self::Mconcat {
        Nothing
    }
}

impl<F> FoldMap<F> for Nothing {
    type FoldMap = Nothing;

    fn fold_map(self, _: F) -> Self::FoldMap {
        self
    }
}

impl<F, Z> Foldr<F, Z> for Nothing {
    type Foldr = Z;

    fn foldr(self, _: F, z: Z) -> Self::Foldr {
        z
    }
}

impl<F, Z> Foldl<F, Z> for Nothing {
    type Foldl = Z;

    fn foldl(self, _: F, z: Z) -> Self::Foldl {
        z
    }
}

impl Fold for Nothing {
    type Fold = Nothing;

    fn fold(self) -> Self::Fold {
        self
    }
}

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
pub struct Just<T>(pub T);

impl<T> Mempty for Just<T> {
    type Mempty = Nothing;

    fn mempty() -> Self::Mempty {
        Nothing
    }
}

impl<T> Mappend<Nothing> for Just<T> {
    type Mappend = Just<T>;

    fn mappend(self, _: Nothing) -> Self::Mappend {
        self
    }
}

impl<T, U> Mappend<Just<U>> for Just<T>
where
    T: Mappend<U>,
{
    type Mappend = Just<T::Mappend>;

    fn mappend(self, t: Just<U>) -> Self::Mappend {
        Just(self.0.mappend(t.0))
    }
}

pub trait Maybe<T>: Pointed + Copointed {}
impl<T> Maybe<T> for Nothing {}
impl<T> Maybe<T> for Just<T> {}

#[cfg(test)]
mod test {
    use crate::{
        closure::{Closure, Curry2, CurryN},
        function::{Add, Mul},
        typeclass::functor::{test_functor_laws, Fmap},
    };

    use super::{Just, Nothing};

    #[test]
    fn test_maybe() {
        let nothing = Nothing;
        let mapped = nothing.fmap(CurryN::<(i32, i32)>::curry_n(Add).call(1));
        assert_eq!(mapped, Nothing);

        let just = Just(2);
        let mapped = just.fmap(Add.curry_n().call(1));
        assert_eq!(mapped, Just(3));
    }

    #[test]
    fn test_functor_laws_maybe() {
        test_functor_laws(Nothing, Add.prefix2(2), Mul.prefix2(2));
        test_functor_laws(Just(1), Add.prefix2(2), Mul.prefix2(2));
    }
}
