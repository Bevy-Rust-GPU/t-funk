use core::marker::PhantomData;

use t_funk_macros::{lift, Closure, Copointed, Pointed};

use crate::{
    closure::{Closure, Curry3, Curry3AB, OutputT},
    function::Function,
    typeclass::{
        applicative::{Apply, Pure},
        functor::Fmap,
        monad::{trans::Lift, Ap, ChainT, LiftM},
    },
};

use crate::typeclass::monad::inst::{Just, Nothing};
use crate::typeclass::monad::{Chain, Return, ReturnT};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
struct MaybeT<T>(pub T);

#[lift]
fn make_maybe_t<T>(t: T) -> MaybeT<T> {
    MaybeT(t)
}

#[lift]
fn make_just<T>(t: T) -> Just<T> {
    Just(t)
}

// Functor

impl<T, F> Fmap<F> for MaybeT<T>
where
    LiftM: Closure<(F, Self)>,
{
    type Fmap = OutputT<LiftM, (F, Self)>;

    fn fmap(self, f: F) -> Self::Fmap {
        LiftM.call((f, self))
    }
}

// Applicative

impl<T, U> Pure<U> for MaybeT<T>
where
    Self: Return<U>,
{
    type Pure = ReturnT<Self, U>;

    fn pure(t: U) -> Self::Pure {
        Self::r#return(t)
    }
}

impl<T, U> Apply<U> for MaybeT<T>
where
    Ap: Closure<(Self, U)>,
{
    type Apply = OutputT<Ap, (Self, U)>;

    fn apply(self, a: U) -> Self::Apply {
        Ap.call((self, a))
    }
}

// Monad

impl<T, U> Return<U> for MaybeT<T>
where
    T: Return<Just<U>>,
{
    type Return = MaybeT<ReturnT<T, Just<U>>>;

    fn r#return(t: U) -> Self::Return {
        MaybeT(T::r#return(Just(t)))
    }
}

impl<T, F> Chain<F> for MaybeT<T>
where
    T: Chain<Curry3AB<ChainMaybeT, PhantomData<T>, F>>,
{
    type Chain = MaybeT<ChainT<T, Curry3AB<ChainMaybeT, PhantomData<T>, F>>>;

    fn chain(self, f: F) -> Self::Chain {
        let MaybeT(x) = self;
        MaybeT(x.chain(ChainMaybeT.prefix3(PhantomData::<T>).call(f)))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
struct ChainMaybeT;

impl<M, F> Function<(PhantomData<M>, F, Nothing)> for ChainMaybeT
where
    M: Return<Nothing>,
{
    type Output = ReturnT<M, Nothing>;

    fn call((_, _, t): (PhantomData<M>, F, Nothing)) -> Self::Output {
        M::r#return(t)
    }
}

impl<M, F, T, U> Function<(PhantomData<M>, F, Just<T>)> for ChainMaybeT
where
    F: Closure<T, Output = MaybeT<U>>,
{
    type Output = U;

    fn call((_, f, Just(input)): (PhantomData<M>, F, Just<T>)) -> Self::Output {
        let MaybeT(out) = f.call(input);
        out
    }
}

// MonadTrans

impl<T, U> Lift<U> for MaybeT<T> {
    type Lift = MaybeT<U>;

    fn lift(t: U) -> Self::Lift {
        MaybeT(t)
    }
}

#[cfg(test)]
mod test {
    use t_funk_macros::lift;

    use crate::{
        closure::{Const, Curry2},
        function::{Id, Mul},
        typeclass::{
            applicative::Apply,
            functor::Fmap,
            monad::{trans::Lift, Chain, Identity, Just, Nothing},
        },
    };

    use super::MaybeT;

    #[lift]
    fn make_just<T>(input: T) -> MaybeT<Identity<Just<T>>> {
        MaybeT(Identity(Just(input)))
    }

    #[lift]
    fn make_nothing<T>(_input: T) -> MaybeT<Identity<Nothing>> {
        MaybeT(Identity(Nothing))
    }

    #[test]
    fn test_maybe_t() {
        let m = Identity(Just(1234));
        let maybe_t = MaybeT::<Identity<Just<i32>>>::lift(m);

        assert_eq!(maybe_t.chain(MakeJust), MaybeT(Identity(Just(1234))));
        assert_eq!(maybe_t.chain(MakeNothing), MaybeT(Identity(Nothing)));

        let apply_mul_2 = maybe_t.fmap(Const(Mul.suffix2(2))).apply(maybe_t);
        assert_eq!(apply_mul_2, MaybeT(Identity(Just(2468))));

        let fmap_identity = maybe_t.fmap(Id);
        assert_eq!(fmap_identity, MaybeT(Identity(Just(1234))));
    }
}
