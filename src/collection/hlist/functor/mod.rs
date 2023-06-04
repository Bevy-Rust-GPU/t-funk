mod fmap;
mod replace;

pub use fmap::*;
pub use replace::*;

#[cfg(test)]
mod test {
    use crate::{
        collection::{hlist::ToTuple, tuple::ToHList},
        function::Function,
        macros::Closure,
        typeclass::functor::Fmap,
    };

    #[test]
    fn test_hlist_functor() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
        struct Mul2;

        impl Function<u32> for Mul2 {
            type Output = u32;

            fn call(input: u32) -> Self::Output {
                input * 2
            }
        }

        impl Function<f32> for Mul2 {
            type Output = f32;

            fn call(input: f32) -> Self::Output {
                input * 2.0
            }
        }

        let list = (1u32, 2.0f32, 3u32).to_hlist();
        let list = list.fmap(Mul2);
        assert_eq!(list.to_tuple(), (2, 4.0, 6))
    }
}
