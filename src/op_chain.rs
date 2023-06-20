//! Extensible operator chaining

use core::ops::{Shl, Shr};

use t_funk::{
    closure::{Closure, OutputT},
    function::Id,
    typeclass::arrow::{Fanout, FanoutT},
};

use crate::{
    closure::{Compose, ComposeT},
    function::Snd,
};

/// Takes a binary function and maps it to the `Shl` and `Shr` operators.
/// Left-shift is analogous to a left fold, and right shift to a right fold.
///
/// Once a chain has started, the opposing shift operator is used to
/// begin a new chain with the current value of the computation.
///
/// ```
/// use t_funk::{function::{Add, Mul, Sub}, op_chain::{op_chain, Done}};
///
/// fn chain_do() {
///     let left = op_chain(Add) << 1 << 2 >> op_chain(Sub) << 2 << 2 >> op_chain(Mul) << 4 >> Done;
///     let right = op_chain(Add) >> 1 >> 2 << op_chain(Sub) >> 2 >> 2 << op_chain(Mul) >> 4 << Done;
///
///     assert_eq!(left, (((1 + 2) - 2) - 2) * 4);
///     assert_eq!(right, 4 * (2 - (2 - (2 + 1))));
/// }
/// ```
///
pub fn op_chain<F>(f: F) -> OpChain<Id, F> {
    OpChain(Id, f)
}

/// Like `Do`, but takes an additional function
/// and uses it to lift values before passing them into the computation chain.
pub fn op_chain_lift<L, F>(l: L, f: F) -> OpChain<L, F> {
    OpChain(l, f)
}

#[allow(non_snake_case)]
pub fn tap<L>(l: L) -> OpChain<ComposeT<Snd, FanoutT<L, Id>>, ()>
where
    L: Fanout<Id>,
{
    OpChain(l.fanout(Id).compose_l(Snd), ())
}

/// The beginning of a computation chain;
/// holds the lifting and binary functions.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpChain<L, F>(pub L, pub F);

impl<L, F, T> Shr<T> for OpChain<L, F>
where
    L: Clone + Closure<T>,
{
    type Output = OpChainRight<L, F, OutputT<L, T>>;

    fn shr(self, rhs: T) -> Self::Output {
        OpChainRight(self.0.clone(), self.1, self.0.call(rhs))
    }
}

impl<L, F, T> Shl<T> for OpChain<L, F>
where
    L: Clone + Closure<T>,
{
    type Output = OpChainLeft<L, F, OutputT<L, T>>;

    fn shl(self, rhs: T) -> Self::Output {
        OpChainLeft(self.0.clone(), self.1, self.0.call(rhs))
    }
}

/// An in-progress chain of computations with right-associativity.
pub struct OpChainRight<L, F, T>(L, F, T);

impl<L, F, T, U> Shr<U> for OpChainRight<L, F, T>
where
    L: Clone + Closure<U>,
    F: Clone + Closure<(OutputT<L, U>, T)>,
{
    type Output = OpChainRight<L, F, OutputT<F, (OutputT<L, U>, T)>>;

    fn shr(self, rhs: U) -> Self::Output {
        OpChainRight(
            self.0.clone(),
            self.1.clone(),
            self.1.call((self.0.call(rhs), self.2)),
        )
    }
}

impl<L, F, T, U> Shl<U> for OpChainRight<L, F, T>
where
    U: Shr<T>,
{
    type Output = U::Output;

    fn shl(self, rhs: U) -> Self::Output {
        rhs >> self.2
    }
}

/// An in-progress chain of computations with left-associativity.
pub struct OpChainLeft<L, F, T>(L, F, T);

impl<L, F, T, U> Shl<U> for OpChainLeft<L, F, T>
where
    L: Clone + Closure<U>,
    F: Clone + Closure<(T, OutputT<L, U>)>,
{
    type Output = OpChainLeft<L, F, OutputT<F, (T, OutputT<L, U>)>>;

    fn shl(self, rhs: U) -> Self::Output {
        OpChainLeft(
            self.0.clone(),
            self.1.clone(),
            self.1.call((self.2, self.0.call(rhs))),
        )
    }
}

impl<L, F, T, U> Shr<U> for OpChainLeft<L, F, T>
where
    U: Shl<T>,
{
    type Output = U::Output;

    fn shr(self, rhs: U) -> Self::Output {
        rhs << self.2
    }
}

/// Used to terminate a computation chain and unwrap its final value.
pub struct Done;

impl<T> Shl<T> for Done {
    type Output = T;

    fn shl(self, rhs: T) -> Self::Output {
        rhs
    }
}

impl<T> Shr<T> for Done {
    type Output = T;

    fn shr(self, rhs: T) -> Self::Output {
        rhs
    }
}

#[cfg(test)]
mod test {
    use t_funk::function::{Add, Mul, Sub};

    use super::{r#op_chain, Done};

    #[test]
    fn test_chainer() {
        let left =
            r#op_chain(Add) << 1 << 2 >> r#op_chain(Sub) << 2 << 2 >> r#op_chain(Mul) << 4 >> Done;
        let right =
            r#op_chain(Add) >> 1 >> 2 << r#op_chain(Sub) >> 2 >> 2 << r#op_chain(Mul) >> 4 << Done;

        assert_eq!(left, (((1 + 2) - 2) - 2) * 4);
        assert_eq!(right, 4 * (2 - (2 - (2 + 1))));
    }
}
