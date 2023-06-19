use crate::{
    closure::{Closure, OutputT},
    macros::foldable::impl_tuple_foldl,
    typeclass::foldable::Foldl,
};

impl_tuple!(impl_tuple_foldl => A, B, C, D, E, F, G, H, I, J, K, L, M);
