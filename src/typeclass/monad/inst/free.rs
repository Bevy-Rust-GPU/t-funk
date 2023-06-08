use crate::{
    closure::{Closure, Curry2, Curry2B},
    function::Function,
    macros::{arrow::Arrow, category::Category, monad::Then, Closure},
    typeclass::{
        functor::Fmap,
        monad::{r#return::Return, Chain, ChainF},
    },
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Then)]
pub struct Free<F>(pub F);

impl<T, F> Chain<F> for Free<T>
where
    T: Fmap<Curry2B<ChainF, F>>,
{
    type Chain = Free<<T as Fmap<Curry2B<ChainF, F>>>::Fmap>;

    fn chain(self, f: F) -> Self::Chain {
        Free(self.0.fmap(ChainF.suffix2(f)))
    }
}

impl<T, U> Return<U> for Free<T> {
    type Return = Free<U>;

    fn r#return(t: U) -> Self::Return {
        Free(t)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Then)]
pub struct ReturnS<F>(pub F);

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct MakeReturn;

impl<T> Function<T> for MakeReturn {
    type Output = ReturnS<T>;

    fn call(input: T) -> Self::Output {
        ReturnS(input)
    }
}
impl<T, F> Chain<F> for ReturnS<T>
where
    F: Closure<T>,
{
    type Chain = F::Output;

    fn chain(self, f: F) -> Self::Chain {
        f.call(self.0)
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct LiftFree;

impl<T> Function<T> for LiftFree
where
    T: Fmap<MakeReturn>,
{
    type Output = Free<T::Fmap>;

    fn call(input: T) -> Self::Output {
        Free(input.fmap(MakeReturn))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        do_monad,
        {
            closure::{Closure, Compose, Composed},
            function::{Function, Id},
            macros::Closure,
            typeclass::{
                functor::Fmap,
                monad::{Chain, Free, LiftFree, MakeReturn, Then},
            },
        },
    };

    use super::ReturnS;

    // DSL
    type String = &'static str;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
    pub struct Get<F>(pub String, pub F);

    impl<T, F> Fmap<F> for Get<T> {
        type Fmap = Get<Composed<F, T>>;

        fn fmap(self, f: F) -> Self::Fmap {
            Get(self.0, f.compose(self.1))
        }
    }

    fn get(key: String) -> Free<Get<Composed<MakeReturn, Id>>> {
        LiftFree.call(Get(key, Id))
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
    pub struct Set<T>(pub String, pub String, pub T);

    impl<T, F> Fmap<F> for Set<T>
    where
        F: Closure<T>,
    {
        type Fmap = Set<F::Output>;

        fn fmap(self, f: F) -> Self::Fmap {
            Set(self.0, self.1, f.call(self.2))
        }
    }

    fn set(key: String, value: String) -> Free<Set<ReturnS<()>>> {
        LiftFree.call(Set(key, value, ()))
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
    pub struct End;

    impl<F> Fmap<F> for End {
        type Fmap = End;

        fn fmap(self, _: F) -> Self::Fmap {
            End
        }
    }

    fn end() -> Free<End> {
        LiftFree.call(End)
    }

    // Concrete interpreter
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
    struct RunIo;

    impl<T> Function<Free<Get<T>>> for RunIo
    where
        T: Closure<String>,
        RunIo: Closure<T::Output>,
    {
        type Output = <RunIo as Closure<T::Output>>::Output;

        fn call(Free(Get(_key, next)): Free<Get<T>>) -> Self::Output {
            //panic!("Get {}", key);
            RunIo.call(next.call("foo"))
        }
    }

    impl<T> Function<Free<Set<T>>> for RunIo
    where
        RunIo: Closure<T>,
    {
        type Output = <RunIo as Closure<T>>::Output;

        fn call(Free(Set(_key, _value, next)): Free<Set<T>>) -> Self::Output {
            //panic!("Set {} to {}", key, value);
            RunIo.call(next)
        }
    }

    impl Function<Free<End>> for RunIo {
        type Output = ();

        fn call(_: Free<End>) -> Self::Output {
            //panic!("End");
        }
    }

    impl<T> Function<ReturnS<T>> for RunIo
    where
        T: core::fmt::Debug,
    {
        type Output = Free<()>;

        fn call(_: ReturnS<T>) -> Self::Output {
            Free(())
        }
    }

    #[test]
    fn foo() {
        let mon = do_monad! {
            foo <- get("foo");
            set("bar", foo);
            end()
        };

        let _zap = RunIo.call(mon);
    }
}
