use core::fmt::Debug;
use core::marker::PhantomData;

mod foldable;
mod functor;
mod monoid;
mod semigroup;

pub use foldable::*;
pub use functor::*;
pub use monoid::*;
pub use semigroup::*;

use crate::macros::phantom::{PhantomClone, PhantomCopy, PhantomDefault};

#[derive(
    Debug, PhantomDefault, PhantomCopy, PhantomClone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Leaf<T = ()>(PhantomData<T>);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Branch<L, T, R>(pub L, pub T, pub R);

pub fn leaf<T>() -> Leaf<T> {
    Leaf::default()
}

pub fn root<T>(t: T) -> Branch<Leaf, T, Leaf> {
    Branch(leaf(), t, leaf())
}

pub fn branch_l<L, T>(l: L, t: T) -> Branch<L, T, Leaf> {
    Branch(l, t, leaf())
}

pub fn branch_r<T, R>(t: T, r: R) -> Branch<Leaf, T, R> {
    Branch(leaf(), t, r)
}

pub fn branch<L, T, R>(l: L, t: T, r: R) -> Branch<L, T, R> {
    Branch(l, t, r)
}

pub fn root_unary<T>(t: T) -> Branch<Leaf<T>, T, Leaf<T>> {
    Branch(leaf(), t, leaf())
}

pub fn branch_l_unary<L, T>(l: L, t: T) -> Branch<L, T, Leaf<T>> {
    Branch(l, t, leaf())
}

pub fn branch_r_unary<T, R>(t: T, r: R) -> Branch<Leaf<T>, T, R> {
    Branch(leaf(), t, r)
}

pub fn branch_unary<L, T, R>(l: L, t: T, r: R) -> Branch<L, T, R> {
    Branch(l, t, r)
}

#[cfg(test)]
mod test {
    use crate::{
        closure::Curry2,
        function::Add,
        function::ToString,
        typeclass::applicative::PureF,
        typeclass::foldable::{FoldMap, Foldl, Foldr},
        typeclass::functor::Fmap,
        typeclass::semigroup::Sum,
    };

    use super::*;

    #[test]
    fn test_htree() {
        #[rustfmt::skip]
        let tree = branch(
            branch_l(
                root("three"),
                2.0
            ),
            1,
            root(4)
        );

        let _tree = tree.fmap(ToString);
        //panic!("{tree:#?}");

        #[rustfmt::skip]
        let tree = branch(
            branch_l(
                root(1),
                2
            ),
            3,
            root(4)
        );

        let _right = tree.foldr(Add.curry2(), 0);
        //panic!("{right:}");

        let _left = tree.foldl(Add.curry2(), 0);
        //panic!("{left:}");

        #[rustfmt::skip]
        let tree = branch_unary(
            branch_l_unary(
                root_unary(1),
                2
            ),
            3,
            root_unary(4)
        );

        let mapped = tree.fold_map(PureF::<Sum<i32>>::default());
        assert_eq!(mapped, Sum(10))
    }
}
