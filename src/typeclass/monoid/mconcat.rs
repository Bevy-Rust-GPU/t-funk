use crate::macros::{functions, types};

#[functions]
#[types]
pub trait Mconcat: Sized {
    type Mconcat;

    fn mconcat(self) -> Self::Mconcat;
}
