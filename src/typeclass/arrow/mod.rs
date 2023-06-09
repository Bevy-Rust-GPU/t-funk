mod arr;
mod fanout;
mod first;
mod inst;
mod second;
mod split;

pub use arr::*;
pub use fanout::*;
pub use first::*;
pub use inst::*;
pub use second::*;
pub use split::*;

#[cfg(test)]
mod test {
    use crate::t_funk::{
        closure::{Closure, Compose, Curry2},
        function::{Add, Mul},
        typeclass::arrow::{Fanout, First, Second, Split},
    };

    #[test]
    fn test_arrow() {
        let a1 = Add.suffix2(5);
        let a2 = Mul.suffix2(2);

        let res = a1.compose(a2).call(3);
        assert_eq!(res, 11);

        let q = (1, 2);

        let res = a1.first().call(q);
        assert_eq!(res, (6, 2));

        let res = a1.second().call(q);
        assert_eq!(res, (1, 7));

        let res = a1.split(a2).call(q);
        assert_eq!(res, (6, 4));

        let res = a1.fanout(a2).call(6);
        assert_eq!(res, (11, 12));
    }
}
