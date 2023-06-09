use crate::{
    closure::Closure,
    macros::{Copointed, Pointed},
    typeclass::functor::Fmap,
};

/// Identity monad, used to lift values into a monadic context.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct Pair<A, B>(pub A, pub B);

impl<A, B, F> Fmap<F> for Pair<A, B>
where
    F: Clone + Closure<A> + Closure<B>,
{
    type Fmap = Pair<<F as Closure<A>>::Output, <F as Closure<B>>::Output>;

    fn fmap(self, f: F) -> Self::Fmap {
        Pair(f.clone().call(self.0), f.call(self.1))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        closure::Curry2,
        function::{Add, Function, Mul},
        macros::Closure,
        typeclass::{
            functor::{test_functor_laws, Fmap},
            monad::Pair,
        },
    };

    #[test]
    fn test_pair() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
        struct Mul3;

        impl Function<i32> for Mul3 {
            type Output = i32;

            fn call(input: i32) -> Self::Output {
                input * 3
            }
        }

        impl Function<f32> for Mul3 {
            type Output = f32;

            fn call(input: f32) -> Self::Output {
                input * 3.0
            }
        }

        let id1: Pair<i32, f32> = Pair(5, 6.0);
        assert_eq!(id1, Pair(5, 6.0));

        let id2: Pair<i32, f32> = id1.fmap(Mul3);
        assert_eq!(id2, Pair(15, 18.0));
    }

    #[test]
    fn test_functor_laws_pair() {
        test_functor_laws(Pair(1, 2), Add.prefix2(2), Mul.prefix2(2));
    }
}
