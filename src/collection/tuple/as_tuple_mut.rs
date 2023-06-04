use crate::collection::hlist::ToTuple;

use super::{AsHListMut, Tuple};

pub trait AsTupleMut: AsHListMut {
    type TupleMut<'a>: Tuple
    where
        Self: 'a;

    fn as_mut<'a>(&'a mut self) -> Self::TupleMut<'a>;
}

impl<T> AsTupleMut for T
where
    T: AsHListMut,
{
    type TupleMut<'a> = <T::HListMut<'a> as ToTuple>::Tuple where T: 'a;

    fn as_mut<'a>(&'a mut self) -> Self::TupleMut<'a> {
        self.as_hlist_mut().to_tuple()
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::tuple::AsTupleMut;

    #[test]
    fn test_tuple_mut() {
        let mut list = (1, 2.0, "three");
        let list_mut = list.as_mut();
        assert_eq!((&mut 1, &mut 2.0, &mut "three"), list_mut);
    }
}
