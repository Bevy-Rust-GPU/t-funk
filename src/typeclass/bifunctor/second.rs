use t_funk_macros::types;

use crate::{closure::Closure, macros::functions};

#[functions]
#[types]
pub trait Second<F> {
    type Second;

    fn second(self, f: F) -> Self::Second;
}

impl<A, B, F> Second<F> for (A, B)
where
    F: Closure<B>,
{
    type Second = (A, F::Output);

    fn second(self, f: F) -> Self::Second {
        (self.0, f.call(self.1))
    }
}
