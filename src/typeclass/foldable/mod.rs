mod fold;
mod fold_map;
mod foldl;
mod foldr;

pub use fold::*;
pub use fold_map::*;
pub use foldl::*;
pub use foldr::*;

#[cfg(test)]
mod test {
    use crate::{
        closure::{Closure, Compose, Curry2, Flip},
        collection::tuple::ToHList,
        function::Sub,
        typeclass::{
            foldable::FoldMap,
            monoid::{Dual, Endo},
            pointed::PointF,
        },
    };

    // 'default' impl of Foldl with respect to Dual / Endo.
    //
    // May be useful for derives, but no compatible types outside of the already-specialized
    // HList have been implemented yet.
    #[test]
    fn test_derived_foldl() {
        let t = (1, 2, 3).to_hlist();

        let Dual(Endo(f)) = t.fold_map(
            PointF::<Dual<_>>::default()
                .compose(PointF::<Endo<_>>::default())
                .compose(Sub.flip().curry2()),
        );

        let res = f.call(0);

        assert_eq!(res, 0 - 1 - 2 - 3);
    }
}