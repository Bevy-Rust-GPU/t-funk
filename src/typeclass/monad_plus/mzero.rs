use t_funk_macros::{functions, types};

#[functions]
#[types]
pub trait Mzero {
    type Mzero;

    fn mzero() -> Self::Mzero;
}

