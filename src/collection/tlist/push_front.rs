use crate::collection::hlist::{PushFront as HListPushFront, ToTuple};

use super::{Tuple, ToHList};

pub trait PushFront<Head> {
    type TuplePushFront: Tuple;

    fn push_front(self, head: Head) -> Self::TuplePushFront;
}

impl<T, Head> PushFront<Head> for T
where
    T: ToHList,
    T::HList: HListPushFront<Head>,
{
    type TuplePushFront = <<T::HList as HListPushFront<Head>>::PushFront as ToTuple>::Tuple;

    fn push_front(self, head: Head) -> Self::TuplePushFront {
        self.to_hlist().push_front(head).to_tuple()
    }
}

#[cfg(test)]
mod tests {
    use super::PushFront;

    #[test]
    fn test_tuple_push_front() {
        let list = ().push_front(3).push_front(2).push_front(1);
        assert_eq!((1, 2, 3), list);
    }
}
