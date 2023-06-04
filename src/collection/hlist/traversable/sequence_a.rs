use crate::{
    collection::hlist::{Cons, Nil},
    function::Id,
    typeclass::traversable::{SequenceA, Traverse},
};

impl<T, N, P> SequenceA<P> for Cons<T, N>
where
    Self: Traverse<Id, P>,
{
    type SequenceA = <Self as Traverse<Id, P>>::Traverse;

    fn sequence_a(self) -> Self::SequenceA {
        self.traverse(Id)
    }
}

impl<P> SequenceA<P> for Nil
where
    Self: Traverse<Id, P>,
{
    type SequenceA = <Self as Traverse<Id, P>>::Traverse;

    fn sequence_a(self) -> Self::SequenceA {
        self.traverse(Id)
    }
}
