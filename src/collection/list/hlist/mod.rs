//! Heterogeneous List (or 'Cons List')
//! Useful for recursive trait evaluation.

mod macros;

mod append;
mod applicative;
mod as_mut;
mod as_ref;
mod chain;
mod clone;
mod fold_map;
mod foldl;
mod foldr;
mod functor;
mod get;
mod gets;
mod hlist;
mod hlist_mut;
mod hlist_ref;
mod length;
mod mconcat;
mod monad;
mod monoid;
mod path;
mod pop_back;
mod pop_front;
mod push_back;
mod push_front;
mod remove;
mod semigroup;
mod set;
mod sets;
mod to_tlist;
mod traversable;

pub use append::*;
pub use applicative::*;
pub use as_mut::*;
pub use as_ref::*;
pub use chain::*;
pub use clone::*;
pub use fold_map::*;
pub use foldl::*;
pub use foldr::*;
pub use functor::*;
pub use get::*;
pub use gets::*;
pub use hlist::*;
pub use hlist_mut::*;
pub use hlist_ref::*;
pub use length::*;
pub use mconcat::*;
pub use path::*;
pub use pop_back::*;
pub use pop_front::*;
pub use push_back::*;
pub use push_front::*;
pub use remove::*;
pub use semigroup::*;
pub use set::*;
pub use sets::*;
pub use to_tlist::*;
pub use traversable::*;

use crate::t_funk::{
    function::Function,
    macros::{arrow::Arrow, category::Category, Closure},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cons<T, N>(pub T, pub N);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nil;

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct MakeCons;

impl<L, R> Function<(L, R)> for MakeCons {
    type Output = Cons<L, R>;

    fn call((l, r): (L, R)) -> Self::Output {
        Cons(l, r)
    }
}
