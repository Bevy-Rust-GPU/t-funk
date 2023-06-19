use t_funk_macros::Monoid;

use crate::{
    macros::{
        applicative::Applicative, foldable::Foldable, functor::Functor, monad::Monad, Copointed,
        Pointed,
    },
    typeclass::semigroup::{Mappend, MappendT},
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
    Monoid,
    Foldable,
)]
pub struct Dual<T>(pub T);

impl<T, U> Mappend<Dual<U>> for Dual<T>
where
    U: Mappend<T>,
{
    type Mappend = Dual<MappendT<U, T>>;

    fn mappend(self, t: Dual<U>) -> Self::Mappend {
        Dual(t.0.mappend(self.0))
    }
}
