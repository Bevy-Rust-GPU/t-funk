use crate::collection::hlist::{Path, Set as HListSet, ToTuple};

use super::{Tuple, ToHList};

pub trait Set<T, P>: Tuple {
    fn set(self, t: T) -> Self;
}

impl<T, P, In> Set<In, P> for T
where
    T: ToHList,
    T::HList: HListSet<In, P> + ToTuple<Tuple = T>,
    P: Path,
{
    fn set(self, t: In) -> T {
        self.to_hlist().set(t).to_tuple()
    }
}

#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn test_tuple_set() {
        let list = (1, 2.0, "three").set(6).set(5.0).set("four");
        assert_eq!((6, 5.0, "four"), list);
    }
}
