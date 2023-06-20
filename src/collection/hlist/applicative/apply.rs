use crate::{
    collection::hlist::{Cons, Nil},
    typeclass::applicative::Apply,
    typeclass::semigroup::Mappend,
    typeclass::{
        functor::Fmap,
        monoid::{Mempty, MemptyT},
    },
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

impl<T> Apply<T> for Nil
where
    T: Mempty,
{
    type Apply = MemptyT<T>;

    fn apply(self, _: T) -> Self::Apply {
        T::mempty()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        closure::Curry2,
        collection::hlist::{Cons, Nil},
        function::Mul,
        typeclass::applicative::Apply,
    };

    #[test]
    fn test_hlist_apply() {
        let list_a = Cons(
            Mul.suffix2(2),
            Cons(Mul.suffix2(5), Cons(Mul.suffix2(9), Nil)),
        );
        let list_b = Cons(1, Cons(4, Cons(7, Nil)));
        let list = list_a.apply(list_b);

        assert_eq!(
            list,
            Cons(
                2,
                Cons(
                    8,
                    Cons(
                        14,
                        Cons(5, Cons(20, Cons(35, Cons(9, Cons(36, Cons(63, Nil))))),),
                    ),
                ),
            ),
        )
    }
}
