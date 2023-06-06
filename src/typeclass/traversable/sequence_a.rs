use crate::macros::{functions, types};

#[functions]
#[types]
pub trait SequenceA<P> {
    type SequenceA;

    fn sequence_a(self) -> Self::SequenceA;
}
