use crate::{
    closure::{Closure, OutputT},
    macros::foldable::impl_tuple_foldl,
    typeclass::foldable::Foldl,
};

impl_tuple!(impl_tuple_foldl => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::{function::Sub, typeclass::foldable::Foldl};

    #[test]
    fn test_tuple_foldl() {
        let list = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        let res = list.foldl(Sub, 0);
        assert_eq!(res, 0 - 1 - 2 - 3 - 4 - 5 - 6 - 7 - 8 - 9 - 10);
    }
}
