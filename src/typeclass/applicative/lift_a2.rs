use t_funk_macros::lift;

use crate::typeclass::{
    applicative::{Apply, ApplyT},
    functor::{Fmap, FmapT},
};

/// Lift a binary function to actions
#[lift]
pub fn lift_a2<F, A, B>(f: F, a: A, b: B) -> ApplyT<FmapT<A, F>, B>
where
    A: Fmap<F>,
    FmapT<A, F>: Apply<B>,
{
    a.fmap(f).apply(b)
}

#[cfg(test)]
mod test {
    use crate::{
        closure::{Closure, Curry2},
        function::MakePair,
        typeclass::applicative::LiftA2,
        typeclass::monad::Just,
    };

    #[test]
    fn test_lift_a2() {
        let foo = LiftA2.call((MakePair.curry2(), Just(3), Just(5)));
        assert_eq!(foo, Just((3, 5)));
    }
}
