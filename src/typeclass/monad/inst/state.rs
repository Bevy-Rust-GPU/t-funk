use crate::{
    closure::{Closure, Curry2, Curry2A, Spread, Spreaded},
    function::{Const, Function, Id, MakePair},
    macros::{arrow::Arrow, category::Category, Closure, Copointed, Pointed},
    typeclass::{
        applicative::{Apply, Pure},
        functor::{Fmap, Replace},
        monad::{Chain, Then, Return},
    },
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct State<F>(pub F);

impl<F1, F2> Fmap<F2> for State<F1> {
    type Fmap = State<StateFunctor<F1, F2>>;

    fn fmap(self, f: F2) -> Self::Fmap {
        State(StateFunctor(self.0, f))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateFunctor<F1, F2>(F1, F2);

impl<F1, F2, S1, S2, O> Closure<S1> for StateFunctor<F1, F2>
where
    F1: Closure<S1, Output = (O, S2)>,
    F2: Closure<O>,
{
    type Output = (F2::Output, S2);

    fn call(self, input: S1) -> Self::Output {
        let (result, s2) = self.0.call(input);
        (self.1.call(result), s2)
    }
}

impl<F1, F2> Replace<F2> for State<F1> {
    type Replace = State<F2>;

    fn replace(self, f: F2) -> Self::Replace {
        State(f)
    }
}

impl<F, T> Pure<T> for State<F> {
    type Pure = State<Curry2A<MakePair, T>>;

    fn pure(t: T) -> Self::Pure {
        State(MakePair.prefix2(t))
    }
}

impl<F1, F2> Apply<State<F2>> for State<F1> {
    type Apply = State<StateApplicative<F1, F2>>;

    fn apply(self, f: State<F2>) -> Self::Apply {
        State(StateApplicative(self, f))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateApplicative<F1, F2>(State<F1>, State<F2>);

impl<F1, F2, S1, S2, O1, S3, O2> Closure<S1> for StateApplicative<F1, F2>
where
    F1: Closure<S1, Output = (O1, S2)>,
    F2: Closure<S2, Output = (O2, S3)>,
    O1: Closure<O2>,
{
    type Output = (O1::Output, S3);

    fn call(self, s1: S1) -> Self::Output {
        let (fx, s2) = self.0 .0.call(s1);
        let (x, s3) = self.1 .0.call(s2);
        (fx.call(x), s3)
    }
}

impl<T, F> Chain<F> for State<T> {
    type Chain = State<StateMonad<T, F>>;

    fn chain(self, f: F) -> Self::Chain {
        State(StateMonad(self, f))
    }
}

impl<F, T> Return<T> for State<F> {
    type Return = State<Curry2A<MakePair, T>>;

    fn r#return(t: T) -> Self::Return {
        State(MakePair.prefix2(t))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateMonad<T, F>(State<T>, F);

impl<T, F, S1, O, S2, F2> Closure<S1> for StateMonad<T, F>
where
    T: Closure<S1, Output = (O, S2)>,
    F: Closure<O, Output = State<F2>>,
    F2: Closure<S2>,
{
    type Output = F2::Output;

    fn call(self, s: S1) -> Self::Output {
        let (x, s2) = self.0 .0.call(s);
        self.1.call(x).0.call(s2)
    }
}

impl<T, _Function> Then<_Function> for State<T>
where
    State<Id>: Apply<_Function>,
{
    type Then = <<Self as Replace<Id>>::Replace as Apply<_Function>>::Apply;

    fn then(self, f: _Function) -> Self::Then {
        self.replace(Id).apply(f)
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Put;

impl<I> Function<I> for Put {
    type Output = State<Curry2A<Const, ((), I)>>;

    fn call(input: I) -> Self::Output {
        State(Const.prefix2(((), input)))
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Get;

impl Function<()> for Get {
    type Output = State<Spreaded<MakePair>>;

    fn call(_: ()) -> Self::Output {
        State(MakePair.spread())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        closure::{Closure, Curry2, Curry2A},
        collection::{
            hlist::{Cons, Nil},
            tlist::ToHList,
        },
        function::{Const, Function, ReplicateM},
        macros::Closure,
        typeclass::{
            monad::{Chain, Put, State},
            traversable::{SequenceA, Traverse},
        },
    };

    use super::Get;

    #[test]
    fn test_state() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Locked;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Unlocked;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Thank;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Open;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Tut;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
        struct Coin;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
        struct Push;

        impl<I> Function<I> for Coin {
            type Output = (Thank, Unlocked);

            fn call(_: I) -> Self::Output {
                (Thank, Unlocked)
            }
        }

        impl Function<Locked> for Push {
            type Output = (Tut, Locked);

            fn call(_: Locked) -> Self::Output {
                (Tut, Locked)
            }
        }

        impl Function<Unlocked> for Push {
            type Output = (Open, Locked);

            fn call(_: Unlocked) -> Self::Output {
                (Open, Locked)
            }
        }

        let coin_s = State(Coin);
        let push_s = State(Push);

        let arr = Coin.call(Locked);
        assert_eq!(arr, (Thank, Unlocked));

        let State(arr) = coin_s.chain(Const.prefix2(push_s));
        let arr = arr.call(Unlocked);

        assert_eq!(arr, (Open, Locked));

        // Chaining
        let monday_s = (coin_s, push_s, push_s, coin_s, push_s).to_hlist();
        let State(res) = SequenceA::<State<()>>::sequence_a(monday_s);
        let res = res.call(Unlocked);
        assert_eq!(
            res,
            (
                Cons(Thank, Cons(Open, Cons(Tut, Cons(Thank, Cons(Open, Nil))))),
                Locked
            )
        );

        // Put
        let State(put) = SequenceA::<State<()>>::sequence_a(
            (
                Put.call(Locked),
                push_s,
                Put.call(Unlocked),
                push_s,
                Put.call(Locked),
            )
                .to_hlist(),
        );
        let res = put.call(Unlocked);
        assert_eq!(res, (((), Tut, (), Open, ()).to_hlist(), Locked));

        // Get
        let State(get) = SequenceA::<State<()>>::sequence_a(
            (Get.call(()), push_s, Get.call(()), push_s, Get.call(())).to_hlist(),
        );
        let res = get.call(Unlocked);
        assert_eq!(
            res,
            ((Unlocked, Open, Locked, Tut, Locked).to_hlist(), Locked)
        );

        // ReplicateM
        let State(res) = ReplicateM::<(((((((),),),),),),), State<()>>::default().call(push_s);
        let res = res.call(Unlocked);
        assert_eq!(res, ((Open, Tut, Tut, Tut, Tut, Tut).to_hlist(), Locked));

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
        struct TurnSImpl;

        impl<T> Function<(Coin, T)> for TurnSImpl {
            type Output = (Thank, Unlocked);

            fn call(_: (Coin, T)) -> Self::Output {
                (Thank, Unlocked)
            }
        }

        impl Function<(Push, Unlocked)> for TurnSImpl {
            type Output = (Open, Locked);

            fn call(_: (Push, Unlocked)) -> Self::Output {
                (Open, Locked)
            }
        }

        impl Function<(Push, Locked)> for TurnSImpl {
            type Output = (Tut, Locked);

            fn call(_: (Push, Locked)) -> Self::Output {
                (Tut, Locked)
            }
        }

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
        struct TurnS;

        impl<A> Function<A> for TurnS {
            type Output = State<Curry2A<TurnSImpl, A>>;

            fn call(a: A) -> Self::Output {
                State(TurnSImpl.prefix2(a))
            }
        }

        let State(res) = TurnS.call(Coin);
        let res = res.call(Locked);
        assert_eq!(res, (Thank, Unlocked));

        let list = (Coin, Push, Push, Coin, Push).to_hlist();
        let State(res) = Traverse::<TurnS, State<()>>::traverse(list, TurnS);
        let res = res.call(Locked);
        assert_eq!(
            res,
            (
                Cons(Thank, Cons(Open, Cons(Tut, Cons(Thank, Cons(Open, Nil))))),
                Locked
            )
        );
    }
}
