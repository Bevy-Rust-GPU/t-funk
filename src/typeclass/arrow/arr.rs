use t_funk_macros::types;

#[types]
pub trait Arr<F> {
    type Arr;

    fn arr(f: F) -> Self::Arr;
}
