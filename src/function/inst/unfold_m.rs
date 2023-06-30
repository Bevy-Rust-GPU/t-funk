use core::{marker::PhantomData, ops::Sub};

use typenum::{Sub1, ToInt, UInt, UTerm, B1};

use crate::{
    closure::{Closure, OutputT},
    function::Function,
    macros::{arrow::Arrow, category::Category, Closure},
    typeclass::{
        monad::{Just, Nothing},
        semigroup::{Mappend, MappendT},
    },
};

use core::ops::Add;

use crate::{
    closure::{Curry2, Curry2A, Curry2B},
    function::{Add as AddF, Sub as SubF},
    typeclass::functor::{Fmap, FmapT},
};

use super::As;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct UnfoldM<F, E>(pub F, pub E);

impl<F, E, A> Closure<A> for UnfoldM<F, E>
where
    F: Clone + Closure<A>,
    UnfoldMInner<F, E>: Closure<OutputT<F, A>>,
{
    type Output = OutputT<UnfoldMInner<F, E>, OutputT<F, A>>;

    fn call(self, a: A) -> Self::Output {
        UnfoldMInner(self.0.clone(), self.1).call(self.0.call(a))
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct UnfoldMInner<F, E>(F, E);

impl<F, E, A, B> Closure<Just<(A, B)>> for UnfoldMInner<F, E>
where
    A: Mappend<OutputT<UnfoldM<F, E>, B>>,
    F: Clone,
    UnfoldM<F, E>: Closure<B>,
{
    type Output = MappendT<A, OutputT<UnfoldM<F, E>, B>>;

    fn call(self, Just((first, second)): Just<(A, B)>) -> Self::Output {
        first.mappend(UnfoldM(self.0.clone(), self.1).call(second))
    }
}

impl<F, E> Closure<Nothing> for UnfoldMInner<F, E> {
    type Output = E;

    fn call(self, _: Nothing) -> Self::Output {
        self.1
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Descending<T>(PhantomData<T>);

impl<T, U, B> Function<UInt<U, B>> for Descending<T>
where
    UInt<U, B>: Sub<B1>,
    Sub1<UInt<U, B>>: ToInt<T>,
{
    type Output = Just<((T,), Sub1<UInt<U, B>>)>;

    fn call(t: UInt<U, B>) -> Self::Output {
        let next = t - B1::default();
        let out = (Sub1::<UInt<U, B>>::to_int(),);
        Just((out, next))
    }
}

impl<T> Function<UTerm> for Descending<T> {
    type Output = Nothing;

    fn call(_: UTerm) -> Self::Output {
        Nothing
    }
}

type AddT<A, B> = <A as Add<B>>::Output;
type SubT<A, B> = <A as Sub<B>>::Output;

pub fn range_int<F, T, O>() -> FmapT<
    FmapT<OutputT<UnfoldM<Descending<O>, ()>, AddT<SubT<T, F>, B1>>, Curry2A<SubF, O>>,
    Curry2B<AddF, O>,
>
where
    F: ToInt<O>,
    T: Sub<F>,
    O: Default,
    SubT<T, F>: Add<B1> + ToInt<O>,
    AddT<SubT<T, F>, B1>: Default,
    UnfoldM<Descending<O>, ()>: Closure<AddT<SubT<T, F>, B1>>,
    OutputT<UnfoldM<Descending<O>, ()>, AddT<SubT<T, F>, B1>>: Fmap<Curry2A<SubF, O>>,
    FmapT<OutputT<UnfoldM<Descending<O>, ()>, AddT<SubT<T, F>, B1>>, Curry2A<SubF, O>>:
        Fmap<Curry2B<AddF, O>>,
{
    UnfoldM(Descending::<O>::default(), ())
        .call(AddT::<SubT<T, F>, B1>::default())
        .fmap(SubF.prefix2(SubT::<T, F>::to_int()))
        .fmap(AddF.suffix2(F::to_int()))
}

pub fn range<F, T, O>() -> FmapT<
    FmapT<
        FmapT<OutputT<UnfoldM<Descending<usize>, ()>, AddT<SubT<T, F>, B1>>, Curry2A<SubF, usize>>,
        Curry2B<AddF, usize>,
    >,
    As<O>,
>
where
    F: ToInt<usize>,
    T: Sub<F>,
    O: Default,
    SubT<T, F>: Add<B1> + ToInt<usize>,
    AddT<SubT<T, F>, B1>: Default,
    UnfoldM<Descending<usize>, ()>: Closure<AddT<SubT<T, F>, B1>>,
    OutputT<UnfoldM<Descending<usize>, ()>, AddT<SubT<T, F>, B1>>: Fmap<Curry2A<SubF, usize>>,
    FmapT<OutputT<UnfoldM<Descending<usize>, ()>, AddT<SubT<T, F>, B1>>, Curry2A<SubF, usize>>:
        Fmap<Curry2B<AddF, usize>>,
    FmapT<
        FmapT<OutputT<UnfoldM<Descending<usize>, ()>, AddT<SubT<T, F>, B1>>, Curry2A<SubF, usize>>,
        Curry2B<AddF, usize>,
    >: Fmap<As<O>>,
{
    range_int::<F, T, usize>().fmap(As::<O>::default())
}

#[cfg(test)]
mod test {
    use typenum::{U1, U4, U5};

    use crate::{
        closure::Closure,
        function::{inst::unfold_m::Descending, range, UnfoldM},
    };

    #[test]
    fn test_unfold_m() {
        let foo = UnfoldM(Descending::<usize>::default(), ()).call(U4::default());
        assert_eq!(foo, (3, 2, 1, 0));
    }

    #[test]
    fn test_range() {
        let foo = range::<U1, U5, f32>();
        assert_eq!(foo, (1.0, 2.0, 3.0, 4.0, 5.0));
    }
}
