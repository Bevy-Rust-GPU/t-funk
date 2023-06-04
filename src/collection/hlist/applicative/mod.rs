mod apply;
mod pure;

pub use apply::*;
pub use pure::*;

#[cfg(test)]
mod test {
    use crate::{
        closure::Curry2,
        collection::hlist::{Cons, Nil},
        function::Add,
        function::Mul,
        typeclass::applicative::Apply,
    };

    #[test]
    fn test_hlist_applicative() {
        let funcs = Cons(Add.suffix2(2), Cons(Mul.suffix2(2), Nil));
        let nums = Cons(1, Cons(2, Cons(3, Nil)));
        let res = funcs.apply(nums);
        assert_eq!(
            res,
            Cons(3, Cons(4, Cons(5, Cons(2, Cons(4, Cons(6, Nil))))))
        );
    }
}
