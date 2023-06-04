use crate::collection::hlist::{HList, ToTuple};

use super::{TListLength, ToHList};

pub trait Tuple: ToHList + TListLength {
    type Head;
    type Tail: Tuple;

    fn head(self) -> Self::Head;
    fn tail(self) -> Self::Tail;
}

impl<T> Tuple for T
where
    T: ToHList,
    <T::HList as HList>::Tail: ToTuple,
{
    type Head = <T::HList as HList>::Head;
    type Tail = <<T::HList as HList>::Tail as ToTuple>::Tuple;

    fn head(self) -> Self::Head {
        self.to_hlist().head()
    }

    fn tail(self) -> Self::Tail {
        self.to_hlist().tail().to_tuple()
    }
}

#[cfg(test)]
mod tests {
    use super::Tuple;

    #[test]
    fn test_tuple_list() {
        let _foo = <(usize, f32, &str) as Tuple>::Head::default();
        let _bar = <(usize, f32, &str) as Tuple>::Tail::default();
    }
}
