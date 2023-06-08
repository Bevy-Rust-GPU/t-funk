mod inst;
pub use inst::*;

pub trait Lift<T> {
    type Lift;

    fn lift(t: T) -> Self::Lift;
}
