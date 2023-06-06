use t_funk_macros::types;

use crate::macros::functions;

/// A type that can take a wrapped function,
/// map it over a provided value, and wrap the result
///
/// To be definition-correct, `Applicative` types must also implement `Functor`,
/// but this cannot be strongly modeled without higher-ranked type bounds.
#[functions]
#[types]
pub trait Apply<T> {
    type Apply;

    /// <*>
    fn apply(self, a: T) -> Self::Apply;
}
