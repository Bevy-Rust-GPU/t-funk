use crate::{
    collection::hlist::{Cons, Nil},
    typeclass::applicative::Apply,
    typeclass::functor::Fmap,
    typeclass::semigroup::Mappend,
};

impl<Head, Tail, U> Apply<U> for Cons<Head, Tail>
where
    Tail: Apply<U>,
    U: Clone + Fmap<Head>,
    U::Fmap: Mappend<Tail::Apply>,
{
    type Apply = <U::Fmap as Mappend<<Tail as Apply<U>>::Apply>>::Mappend;

    fn apply(self, a: U) -> Self::Apply {
        a.clone().fmap(self.0).mappend(self.1.apply(a))
    }
}

impl<T> Apply<T> for Nil {
    type Apply = Nil;

    fn apply(self, _: T) -> Self::Apply {
        self
    }
}
