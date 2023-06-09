use crate::{
    closure::Closure,
    collection::hlist::{Cons, HList, Nil},
    typeclass::foldable::Foldr,
};

impl<Head, Tail, F, Z> Foldr<F, Z> for Cons<Head, Tail>
where
    Self: HList,
    Tail: Foldr<F, Z>,
    F: Clone + Closure<(Head, Tail::Foldr)>,
{
    type Foldr = F::Output;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        f.clone().call((self.0, self.1.foldr(f, z)))
    }
}

impl<F, T> Foldr<F, T> for Nil {
    type Foldr = T;

    fn foldr(self, _: F, z: T) -> Self::Foldr {
        z
    }
}

#[cfg(test)]
mod test {
    use crate::{collection::tlist::ToHList, function::Sub, typeclass::foldable::Foldr};

    #[test]
    fn test_hlist_foldr() {
        let list = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10).to_hlist();
        let res = list.foldr(Sub, 0);
        assert_eq!(
            res,
            (1 - (2 - (3 - (4 - (5 - (6 - (7 - (8 - (9 - (10 - 0))))))))))
        );
    }
}
