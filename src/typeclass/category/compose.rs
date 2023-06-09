use t_funk_macros::types;

use crate::macros::functions;

/// Right-to-left composition
/// (. or <<<)
#[functions]
#[types]
pub trait Compose<F>: Sized {
    type Compose;
    fn compose(self, f: F) -> Self::Compose;
}

/// Left-to-right composition
/// (>>>)
#[functions]
#[types]
pub trait ComposeL<F>: Sized {
    type ComposeL;
    fn compose_l(self, f: F) -> Self::ComposeL;
}

impl<T, F> ComposeL<F> for T
where
    F: Compose<T>,
{
    type ComposeL = F::Compose;

    fn compose_l(self, f: F) -> Self::ComposeL {
        f.compose(self)
    }
}
