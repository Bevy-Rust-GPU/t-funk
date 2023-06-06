use crate::{
    collection::hlist::{Cons, Nil},
    typeclass::{
        functor::{Fmap, FmapT},
        monad::Chain,
        monoid::{Mconcat, MconcatT},
    },
};

impl<Head, Tail, F> Chain<F> for Cons<Head, Tail>
where
    Self: Fmap<F>,
    FmapT<Self, F>: Mconcat,
{
    type Chain = MconcatT<FmapT<Self, F>>;

    fn chain(self, f: F) -> Self::Chain {
        self.fmap(f).mconcat()
    }
}

impl<F> Chain<F> for Nil {
    type Chain = Self;

    fn chain(self, _: F) -> Self::Chain {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{
        collection::{
            hbtree::{root, Branch, Leaf},
            hlist::{Cons, Nil},
        },
        function::Function,
        macros::Closure,
    };
    use t_funk::typeclass::monad::Chain;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
    struct MakeTree;

    impl<T> Function<T> for MakeTree {
        type Output = Branch<Leaf, T, Leaf>;

        fn call(input: T) -> Self::Output {
            root(input)
        }
    }

    #[test]
    fn test_hlist_chain() {
        let list = Cons(1, Cons(2.0, Cons('3', Cons("4", Nil))));
        let list = list.chain(MakeTree);
        //panic!("{list:#?}");
    }
}
