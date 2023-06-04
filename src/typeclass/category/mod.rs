mod compose;
mod id;

pub use compose::*;
pub use id::*;

#[cfg(test)]
mod test {
    use crate::t_funk::{
        typeclass::category::Compose, typeclass::category::Id, function::Add, closure::{Closure, Curry2},
    };

    #[test]
    fn test_category() {
        let foo = Add.suffix2(3);
        let bar = Add::id();
        let baz = foo.compose(bar);
        let res = baz.call(1234);
        assert_eq!(res, 1237)
    }
}
