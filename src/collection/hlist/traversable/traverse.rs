use crate::{
    closure::{Closure, Curried2, Curry2, Flip, Flipped},
    collection::hlist::{Cons, Nil, PushFrontF},
    typeclass::{
        applicative::{Apply, Pure, PureT},
        functor::Fmap,
        traversable::Traverse,
    },
};

impl<T, N, F, P> Traverse<F, P> for Cons<T, N>
where
    F: Clone + Closure<T>,
    F::Output: Fmap<Curried2<Flipped<PushFrontF>>>,
    <F::Output as Fmap<Curried2<Flipped<PushFrontF>>>>::Fmap: Apply<N::Traverse>,
    N: Traverse<F, P>,
{
    type Traverse =
        <<F::Output as Fmap<Curried2<Flipped<PushFrontF>>>>::Fmap as Apply<N::Traverse>>::Apply;

    fn traverse(self, f: F) -> Self::Traverse {
        f.clone()
            .call(self.0)
            .fmap(PushFrontF.flip().curry2())
            .apply(self.1.traverse(f))
    }
}

impl<F, P> Traverse<F, P> for Nil
where
    P: Pure<Self>,
{
    type Traverse = PureT<P, Self>;

    fn traverse(self, _: F) -> Self::Traverse {
        P::pure(self)
    }
}
