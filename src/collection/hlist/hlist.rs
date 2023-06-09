use crate::collection::hlist::{Cons, HListLength, Nil, ToTuple};

/// The base HList type.
/// Describes the Head / CAR, Tail / CDR structure via associated types.
pub trait HList: HListLength + ToTuple {
    type Head;
    type Tail: HList;

    fn head(self) -> Self::Head;
    fn tail(self) -> Self::Tail;
}

impl<T, N> HList for Cons<T, N>
where
    Self: ToTuple,
    N: HList,
{
    type Head = T;
    type Tail = N;

    fn head(self) -> Self::Head {
        self.0
    }

    fn tail(self) -> Self::Tail {
        self.1
    }
}

impl HList for Nil {
    type Head = Nil;
    type Tail = Nil;

    fn head(self) -> Self::Head {
        self
    }

    fn tail(self) -> Self::Tail {
        self
    }
}
