mod chain;
mod inst;
mod r#return;
mod then;

pub mod trans;

use core::marker::PhantomData;

pub use chain::*;
pub use inst::*;
pub use r#return::*;
use t_funk_macros::lift;
pub use then::*;

use crate::closure::{Closure, Curry3, Curry3AB, OutputT};

/// Apply a monadic function to a monadic value
#[lift]
pub fn ap<M1, M2>(m1: M1, m2: M2) -> ChainT<M1, Curry3AB<ApInner, PhantomData<M1>, M2>>
where
    M1: Chain<Curry3AB<ApInner, PhantomData<M1>, M2>>,
{
    m1.chain(ApInner.prefix3(PhantomData::<M1>).call(m2))
}

#[lift]
pub fn ap_inner<M, X1, M2>(
    m: PhantomData<M>,
    m2: M2,
    x1: X1,
) -> ChainT<M2, Curry3AB<ApInner2, PhantomData<M>, X1>>
where
    M2: Chain<Curry3AB<ApInner2, PhantomData<M>, X1>>,
{
    m2.chain(ApInner2.prefix3(m).call(x1))
}

#[lift]
pub fn ap_inner2<M, X1, X2>(_m: PhantomData<M>, x1: X1, x2: X2) -> ReturnT<M, OutputT<X1, X2>>
where
    X1: Closure<X2>,
    M: Return<OutputT<X1, X2>>,
{
    M::r#return(x1.call(x2))
}

/// Promote a function to a monad
#[lift]
pub fn lift_m<F, M1>(f: F, m1: M1) -> ChainT<M1, Curry3AB<LiftMInner, PhantomData<M1>, F>>
where
    M1: Chain<Curry3AB<LiftMInner, PhantomData<M1>, F>>,
{
    m1.chain(LiftMInner.prefix3(PhantomData::<M1>).call(f))
}

#[lift]
pub fn lift_m_inner<M, F, X1>(_m: PhantomData<M>, f: F, x1: X1) -> ReturnT<M, OutputT<F, X1>>
where
    F: Closure<X1>,
    M: Return<OutputT<F, X1>>,
{
    M::r#return(f.call(x1))
}
