use t_funk_macros::{functions, types};

#[functions]
#[types]
pub trait Mplus<T> {
    type Mplus;

    fn mplus(self, t: T) -> Self::Mplus;
}
