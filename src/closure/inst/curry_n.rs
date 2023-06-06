use core::marker::PhantomData;

use t_funk_macros::{types, functions};

use crate::{
    closure::Closure,
    collection::{
        hlist::{Cons, HList, Nil, PushBack, ToTuple},
        tuple::ToHList,
    },
    macros::{arrow::Arrow, category::Category},
    typeclass::pointed::Pointed,
};

#[functions]
#[types]
pub trait CurryN<I> {
    type Curried;

    fn curry_n(self) -> Self::Curried;
}

impl<T, I> CurryN<I> for T
where
    T: Closure<I>,
    I: ToHList,
{
    type Curried = CurriedN<T, Nil, I::HList>;

    fn curry_n(self) -> Self::Curried {
        CurriedN(self, Nil, PhantomData)
    }
}

#[derive(Category, Arrow)]
pub struct CurriedN<F, AO, AI>(F, AO, PhantomData<AI>);

impl<F, AI> Default for CurriedN<F, Nil, AI>
where
    F: Default,
{
    fn default() -> Self {
        CurriedN(Default::default(), Nil, PhantomData)
    }
}

impl<F, AO, AI> Clone for CurriedN<F, AO, AI>
where
    F: Clone,
    AO: Clone,
{
    fn clone(&self) -> Self {
        CurriedN(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<F, AO, AI> Copy for CurriedN<F, AO, AI>
where
    F: Copy,
    AO: Copy,
{
}

impl<F, AI> Pointed for CurriedN<F, Nil, AI>
where
    F: Closure<AI::Tuple>,
    AI: HList,
{
    type Pointed = F;

    fn point(unit: Self::Pointed) -> Self {
        CurriedN(unit, Nil, PhantomData)
    }
}

impl<F, AO, I> Closure<I> for CurriedN<F, AO, Cons<I, Nil>>
where
    AO: PushBack<I>,
    F: Closure<<AO::PushBack as ToTuple>::Tuple>,
{
    type Output = F::Output;

    fn call(self, input: I) -> Self::Output {
        self.0.call(self.1.push_back(input).to_tuple())
    }
}

impl<F, AO, Tail, Tail2, I> Closure<I> for CurriedN<F, AO, Cons<I, Cons<Tail, Tail2>>>
where
    AO: PushBack<I>,
{
    type Output = CurriedN<F, AO::PushBack, Cons<Tail, Tail2>>;

    fn call(self, input: I) -> Self::Output {
        CurriedN(self.0, self.1.push_back(input), PhantomData)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        closure::{Closure, CurryN},
        function::Function,
        macros::Closure,
    };

    #[test]
    fn test_curry_n() {
        #[derive(Closure)]
        struct ManyArgs;

        impl<A, B, C, D, E, F, G> Function<(A, B, C, D, E, F, G)> for ManyArgs {
            type Output = (A, B, C, D, E, F, G);

            fn call(input: (A, B, C, D, E, F, G)) -> Self::Output {
                input
            }
        }

        let curried = ManyArgs.curry_n();
        let curried = curried.call(1);
        let curried = curried.call(2.0);
        let curried = curried.call("three");
        let curried = curried.call(4);
        let curried = curried.call(5.0);
        let curried = curried.call("six");
        let curried = curried.call(7);
        assert_eq!(curried, (1, 2.0, "three", 4, 5.0, "six", 7));
    }
}
