use crate::{
    closure::{Closure, OutputT},
    macros::foldable::impl_tuple_foldr,
    typeclass::foldable::Foldr,
};

impl_tuple!(impl_tuple_foldr => A, B, C, D, E, F, G, H, I, J, K, L, M);

