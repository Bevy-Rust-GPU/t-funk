use crate::collection::hlist::{PushBack as HListPushBack, ToTuple};

use super::{Tuple, ToHList};

pub trait PushBack<Tail> {
    type Path;
    type TuplePushBack: Tuple;

    fn push_back(self, tail: Tail) -> Self::TuplePushBack;
}

impl<T, Head> PushBack<Head> for T
where
    T: ToHList,
    T::HList: HListPushBack<Head>,
{
    type Path = <T::HList as HListPushBack<Head>>::Path;
    type TuplePushBack = <<T::HList as HListPushBack<Head>>::PushBack as ToTuple>::Tuple;

    fn push_back(self, tail: Head) -> Self::TuplePushBack {
        self.to_hlist().push_back(tail).to_tuple()
    }
}

#[cfg(test)]
mod tests {
    use super::PushBack;

    #[test]
    fn test_tuple_push_back() {
        let list = ().push_back(1).push_back(2).push_back(3);
        assert_eq!((1, 2, 3), list);
    }
}
