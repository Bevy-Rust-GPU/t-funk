use crate::collection::hlist::{HListMut, ToTuple, HList};

use super::{ref_tuple::RefTuple, ToHList};

pub trait MutTuple<'a>: RefTuple<'a> {
    type HeadMut: 'a;
    type TailMut: MutTuple<'a>;

    fn head_mut(self) -> Self::HeadMut;
    fn tail_mut(self) -> Self::TailMut;
}

impl<'a, T> MutTuple<'a> for T
where
    T: ToHList,
    T::HList: HListMut<'a>,
    <T::HList as HList>::Tail: ToTuple,
{
    type HeadMut = <T::HList as HListMut<'a>>::HeadMut;
    type TailMut = <<T::HList as HListMut<'a>>::TailMut as ToTuple>::Tuple;

    fn head_mut(self) -> Self::HeadMut {
        self.to_hlist().head_mut()
    }

    fn tail_mut(self) -> Self::TailMut {
        self.to_hlist().tail_mut().to_tuple()
    }
}

#[cfg(test)]
mod tests {
    use super::MutTuple;

    #[test]
    fn test_tuple_list_mut() {
        let _foo =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as MutTuple>::HeadMut>();
        let _bar =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as MutTuple>::TailMut>();
    }
}
