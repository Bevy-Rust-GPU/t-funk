use crate::{closure::Closure, macros::functions};

#[functions]
pub trait First<F> {
    type First;

    fn first(self, f: F) -> Self::First;
}

impl<A, B, F> First<F> for (A, B)
where
    F: Closure<A>,
{
    type First = (F::Output, B);

    fn first(self, f: F) -> Self::First {
        (f.call(self.0), self.1)
    }
}
