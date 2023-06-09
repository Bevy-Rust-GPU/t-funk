use crate::t_funk::macros::{monad::Chain, Copointed, Pointed};

use crate::t_funk::{
    closure::Closure,
    typeclass::{
        applicative::{Apply, Pure},
        functor::Fmap,
        monoid::Mempty,
        semigroup::Mappend,
    },
};
use crate::typeclass::monoid::Mconcat;
use core::ops::Add;

/// A `Semigroup` wrapper that can append additively.
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed, Chain,
)]
pub struct Sum<T>(pub T);

impl<T, F> Fmap<F> for Sum<T>
where
    F: Closure<T>,
{
    type Fmap = Sum<F::Output>;

    fn fmap(self, f: F) -> Self::Fmap {
        Sum(f.call(self.0))
    }
}

impl<F, T> Apply<Sum<T>> for Sum<F>
where
    F: Closure<T>,
{
    type Apply = Sum<F::Output>;

    fn apply(self, Sum(t): Sum<T>) -> Self::Apply {
        Sum(self.0.call(t))
    }
}

impl<T, U> Mappend<Sum<U>> for Sum<T>
where
    T: Add<U>,
{
    type Mappend = Sum<T::Output>;

    fn mappend(self, t: Sum<U>) -> Self::Mappend {
        Sum(self.0 + t.0)
    }
}

impl<T> Mempty for Sum<T>
where
    T: Default,
{
    type Mempty = Sum<T>;

    fn mempty() -> Self::Mempty {
        Sum(Default::default())
    }
}

impl<T> Mconcat for Sum<T> {
    type Mconcat = Self;

    fn mconcat(self) -> Self::Mconcat {
        self
    }
}

impl<T, U> Pure<U> for Sum<T> {
    type Pure = Sum<U>;

    fn pure(t: U) -> Self::Pure {
        Sum(t)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::typeclass::semigroup::{Mappend, Sum};

    #[test]
    fn test_sum() {
        assert_eq!(Sum(2).mappend(Sum(4)).mappend(Sum(6)), Sum(12))
    }
}
