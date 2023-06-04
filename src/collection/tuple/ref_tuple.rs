use crate::collection::hlist::{HListRef, ToTuple};

use super::{ToHList, Tuple};

pub trait RefTuple<'a>: Tuple {
    type HeadRef: 'a;
    type TailRef: RefTuple<'a>;

    fn head_ref(self) -> Self::HeadRef;
    fn tail_ref(self) -> Self::TailRef;
}

impl<'a, T> RefTuple<'a> for T
where
    T: ToHList,
    T::HList: HListRef<'a>,
{
    type HeadRef = <T::HList as HListRef<'a>>::HeadRef;
    type TailRef = <<T::HList as HListRef<'a>>::TailRef as ToTuple>::Tuple;

    fn head_ref(self) -> Self::HeadRef {
        self.to_hlist().head_ref()
    }

    fn tail_ref(self) -> Self::TailRef {
        self.to_hlist().tail_ref().to_tuple()
    }
}

#[cfg(test)]
mod tests {
    use super::RefTuple;

    #[test]
    fn test_tuple_list_ref() {
        let _foo = core::any::type_name::<<(&usize, &f32, &&str) as RefTuple>::HeadRef>();
        let _bar = core::any::type_name::<<(&usize, &f32, &&str) as RefTuple>::TailRef>();

        let _foo =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as RefTuple>::HeadRef>();
        let _bar =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as RefTuple>::TailRef>();
    }
}
