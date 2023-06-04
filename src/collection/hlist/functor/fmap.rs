use crate::{
    closure::Closure,
    collection::hlist::{Cons, Nil},
    typeclass::functor::Fmap,
};

impl<T, N, F> Fmap<F> for Cons<T, N>
where
    F: Clone + Closure<T>,
    N: Fmap<F>,
{
    type Fmap = Cons<F::Output, N::Fmap>;

    fn fmap(self, f: F) -> Self::Fmap {
        Cons(f.clone().call(self.0), self.1.fmap(f))
    }
}

impl<F> Fmap<F> for Nil {
    type Fmap = Nil;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

