use crate::{
    closure::{Closure, OutputT},
    macros::foldable::impl_tuple_foldr,
    typeclass::foldable::Foldr,
};

impl_tuple!(impl_tuple_foldr => A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod test {
    use crate::{function::Sub, typeclass::foldable::Foldr};

    #[test]
    fn test_tuple_foldr() {
        let list = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        let res = list.foldr(Sub, 0);
        assert_eq!(
            res,
            (1 - (2 - (3 - (4 - (5 - (6 - (7 - (8 - (9 - (10 - 0))))))))))
        );
    }
}
