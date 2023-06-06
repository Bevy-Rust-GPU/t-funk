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

#[cfg(test)]
mod test {
    use crate::{collection::hlist::{Cons, Nil}, function::Mul, closure::Curry2, typeclass::applicative::Apply};

    #[test]
    fn test_hlist_apply() {
        let list_a = Cons(
            Mul.suffix2(2),
            Cons(Mul.suffix2(5), Cons(Mul.suffix2(9), Nil)),
        );
        let list_b = Cons(1, Cons(4, Cons(7, Nil)));
        let list = list_a.apply(list_b);
        //panic!("{list:#?}");
    }
}
