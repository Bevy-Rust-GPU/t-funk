use core::marker::PhantomData;

use t_funk_macros::{define_adt, impl_adt};

use crate::{
    closure::Closure,
    collection::hlist::Nil,
    function::Id as IdF,
    typeclass::{
        arrow::{Arr, Fanout, FanoutT, First, Second, Split},
        category::{Compose, ComposeL, Id},
        copointed::Copointed,
        pointed::Pointed,
        semigroup::Mappend,
    },
};

define_adt! {
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pure<F>(pub F)
             | Effect<F1>(pub F1)
             | Seq<F1, F2>(pub F1, pub F2)
             | Par<F1, F2>(pub F1, pub F2);
}

impl_adt! {
    impl<F1, F2> Id for Pure<F1> | Effect<F1> | Seq<F1, F2> | Par<F1, F2> {
        type Id = Pure<IdF>;

        fn id() -> Self::Id {
            Pure(IdF)
        }
    }
}

impl_adt! {
    impl<F1, F2, F> Compose<F> for Pure<F1> | Effect<F1> | Seq<F1, F2> | Par<F1, F2> {
        type Compose = Seq<F, Self>;

        fn compose(self, f: F) -> Self::Compose {
            Seq(f, self)
        }
    }
}

impl_adt! {
    impl<F1, F2, F> Arr<F> for Pure<F1> | Effect<F1> | Seq<F1, F2> | Par<F1, F2> {
        type Arr = Pure<F>;

        fn arr(f: F) -> Self::Arr {
            Pure(f)
        }
    }
}

impl_adt! {
    impl<F1, F2> First for Pure<F1> | Effect<F1> | Seq<F1, F2> | Par<F1, F2> {
        type First = Par<Self, IdF>;

        fn first(self) -> Self::First {
            Par(self, IdF)
        }
    }
}

impl_adt! {
    impl<F1, F2> Second for Pure<F1> | Effect<F1> | Seq<F1, F2> | Par<F1, F2> {
        type Second = Par<IdF, Self>;

        fn second(self) -> Self::Second {
            Par(IdF, self)
        }
    }
}

impl_adt! {
    impl<F1, F2, F> Split<F> for Pure<F1> | Effect<F1> | Seq<F1, F2> | Par<F1, F2> {
        type Split = Par<Self, F>;

        fn split(self, g: F) -> Self::Split {
            Par(self, g)
        }
    }
}

impl_adt! {
    impl<F1, F2, F> Fanout<F> for Pure<F1> | Effect<F1> | Seq<F1, F2> | Par<F1, F2> {
    type Fanout = Seq<Pure<FanoutT<IdF, IdF>>, Par<Self, F>>;

        fn fanout(self, f: F) -> Self::Fanout {
            Self::arr(IdF.fanout(IdF)).compose_l(self.split(f))
        }
    }
}

/// Traverses a free arrow construct,
/// folding its contents using `Mappend`
pub struct Analyze<F, A>(F, PhantomData<A>);

impl<F, A> Pointed for Analyze<F, A> {
    type Pointed = F;

    fn point(unit: Self::Pointed) -> Self {
        Analyze(unit, PhantomData)
    }
}

impl<F, A> Copointed for Analyze<F, A> {
    type Copointed = F;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

impl<F, A> Default for Analyze<F, A>
where
    F: Default,
{
    fn default() -> Self {
        Analyze(F::default(), PhantomData)
    }
}

impl<F, A> Clone for Analyze<F, A>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Analyze(self.0.clone(), PhantomData)
    }
}

impl<F, A, P> Closure<Pure<P>> for Analyze<F, A> {
    type Output = Nil; //<MemptyF<A> as crate::applicative::Pure>::Pure<A>;

    fn call(self, _: Pure<P>) -> Self::Output {
        Nil
        //<MemptyF<A> as crate::applicative::Pure>::pure(Default::default())
    }
}

impl<F, A, E> Closure<Effect<E>> for Analyze<F, A>
where
    F: Closure<E>,
{
    type Output = F::Output;

    fn call(self, Effect(g): Effect<E>) -> Self::Output {
        self.0.call(g)
    }
}

impl<F, A, S1, S2> Closure<Seq<S1, S2>> for Analyze<F, A>
where
    Analyze<F, A>: Clone + Closure<S1> + Closure<S2>,
    <Analyze<F, A> as Closure<S1>>::Output: Mappend<<Analyze<F, A> as Closure<S2>>::Output>,
{
    type Output = <<Analyze<F, A> as Closure<S1>>::Output as Mappend<
        <Analyze<F, A> as Closure<S2>>::Output,
    >>::Mappend;

    fn call(self, Seq(s1, s2): Seq<S1, S2>) -> Self::Output {
        let fg = self.clone().call(s1);
        let fh = self.call(s2);

        fg.mappend(fh)
    }
}

impl<F, A, P1, P2> Closure<Par<P1, P2>> for Analyze<F, A>
where
    Analyze<F, A>: Clone + Closure<P1> + Closure<P2>,
    <Analyze<F, A> as Closure<P1>>::Output: Mappend<<Analyze<F, A> as Closure<P2>>::Output>,
{
    type Output = <<Analyze<F, A> as Closure<P1>>::Output as Mappend<
        <Analyze<F, A> as Closure<P2>>::Output,
    >>::Mappend;

    fn call(self, Par(p1, p2): Par<P1, P2>) -> Self::Output {
        let fg = self.clone().call(p1);
        let fh = self.call(p2);

        fg.mappend(fh)
    }
}

/// Traverses a free arrow construct,
/// ...
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvalA<F>(pub F);

impl<F, G> Closure<Pure<G>> for EvalA<F> {
    type Output = G;

    fn call(self, Pure(g): Pure<G>) -> Self::Output {
        //<A as Arr<G>>::arr(g)
        g
    }
}

impl<F, G> Closure<Effect<G>> for EvalA<F>
where
    F: Closure<G>,
{
    type Output = F::Output;

    fn call(self, Effect(g): Effect<G>) -> Self::Output {
        self.0.call(g)
    }
}

impl<F, G, H> Closure<Seq<G, H>> for EvalA<F>
where
    EvalA<F>: Clone + Closure<G> + Closure<H>,
    <EvalA<F> as Closure<H>>::Output: Compose<<EvalA<F> as Closure<G>>::Output>,
{
    type Output =
        <<EvalA<F> as Closure<H>>::Output as Compose<<EvalA<F> as Closure<G>>::Output>>::Compose;

    fn call(self, Seq(g, h): Seq<G, H>) -> Self::Output {
        let fg = self.clone().call(g);
        let fh = self.call(h);
        fh.compose(fg)
    }
}

impl<F, G, H> Closure<Par<G, H>> for EvalA<F>
where
    EvalA<F>: Clone + Closure<G> + Closure<H>,
    <EvalA<F> as Closure<G>>::Output: Split<<EvalA<F> as Closure<H>>::Output>,
{
    type Output =
        <<EvalA<F> as Closure<G>>::Output as Split<<EvalA<F> as Closure<H>>::Output>>::Split;

    fn call(self, Par(g, h): Par<G, H>) -> Self::Output {
        let fg = self.clone().call(g);
        let fh = self.call(h);
        fg.split(fh)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        closure::{Closure, Curry2, Curry2A},
        function::{Add, Div, Function, Mul},
        macros::{
            category::{Compose, Id},
            Closure,
        },
        typeclass::{
            arrow::{Analyze, Fanout},
            category::ComposeL,
            semigroup::Sum,
        },
    };

    use super::{Effect, EvalA};

    fn add(n: f32) -> Effect<Curry2A<Add, f32>> {
        Effect(Add.prefix2(n))
    }

    fn mul(n: f32) -> Effect<Curry2A<Mul, f32>> {
        Effect(Mul.prefix2(n))
    }

    fn div() -> Effect<Div> {
        Effect(Div)
    }

    #[derive(
        Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Id, Compose,
    )]
    struct Interpret;

    impl Function<Curry2A<Add, f32>> for Interpret {
        type Output = Curry2A<Add, f32>;

        fn call(input: Curry2A<Add, f32>) -> Self::Output {
            input
        }
    }

    impl Function<Curry2A<Mul, f32>> for Interpret {
        type Output = Curry2A<Mul, f32>;

        fn call(input: Curry2A<Mul, f32>) -> Self::Output {
            input
        }
    }

    impl Function<Div> for Interpret {
        type Output = Div;

        fn call(input: Div) -> Self::Output {
            input
        }
    }

    #[derive(
        Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Id, Compose,
    )]
    struct CountOps;

    impl Function<Curry2A<Add, f32>> for CountOps {
        type Output = Sum<usize>;

        fn call(_: Curry2A<Add, f32>) -> Self::Output {
            Sum(1)
        }
    }

    impl Function<Curry2A<Mul, f32>> for CountOps {
        type Output = Sum<usize>;

        fn call(_: Curry2A<Mul, f32>) -> Self::Output {
            Sum(1)
        }
    }

    impl Function<Div> for CountOps {
        type Output = Sum<usize>;

        fn call(_: Div) -> Self::Output {
            Sum(1)
        }
    }

    #[test]
    fn test_free_arrow() {
        let arr = add(2.0)
            .fanout(mul(2.0))
            .compose_l(div())
            .compose_l(add(6.0).fanout(mul(6.0)))
            .compose_l(div());

        let ops = Analyze::<CountOps, Sum<usize>>::default().call(arr);
        assert_eq!(ops, Sum(6));

        let foo = EvalA(Interpret).call(arr);
        let res = foo.call(5.0);
        assert_eq!(res, 1.5952381)
    }
}
