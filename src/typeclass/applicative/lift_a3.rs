use t_funk_macros::lift;

use crate::typeclass::{
    applicative::Apply,
    functor::{Fmap, FmapT},
};

use super::{lift_a2, ApplyT};

/// Lift a ternary function to actions
#[lift]
pub fn lift_a3<F, A, B, C>(f: F, a: A, b: B, c: C) -> ApplyT<ApplyT<FmapT<A, F>, B>, C>
where
    A: Fmap<F>,
    FmapT<A, F>: Apply<B>,
    ApplyT<FmapT<A, F>, B>: Apply<C>,
{
    lift_a2(f, a, b).apply(c)
}
