use crate::t_funk::macros::functions;

#[functions]
pub trait SequenceA<P> {
    type SequenceA;

    fn sequence_a(self) -> Self::SequenceA;
}

