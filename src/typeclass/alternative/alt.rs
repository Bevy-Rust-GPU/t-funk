use t_funk_macros::{functions, types};

#[functions]
#[types]
pub trait Alt<T> {
    type Alt;

    fn alt(self, t: T) -> Self::Alt;
}

