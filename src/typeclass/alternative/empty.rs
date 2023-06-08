use t_funk_macros::{functions, types};

#[functions]
#[types]
pub trait Empty {
    type Empty;

    fn empty() -> Self::Empty;
}

