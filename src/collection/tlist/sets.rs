use crate::collection::hlist::{Paths, Sets as HListSets, ToTuple};

use super::{Tuple, ToHList};

pub trait Sets<T, P>: Tuple {
    fn sets(self, t: T) -> Self;
}

impl<T, P, In> Sets<In, P> for T
where
    Self: Tuple,
    Self::HList: HListSets<In::HList, P>,
    In: ToHList,
    P: Paths,
{
    fn sets(self, t: In) -> Self {
        self.to_hlist().sets(t.to_hlist()).to_tuple()
    }
}

impl<T> Sets<(), ()> for T
where
    T: Tuple,
{
    fn sets(self, _: ()) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::tlist::Sets;

    #[test]
    fn test_tuple_sets() {
        let list = (1, 2.0, "three");
        let list = Sets::<(&str, f32, usize), _>::sets(list, ("hello", 7.0, 5));
        //let list = list.tuple_sets(("hello", 7.0, 5));
        assert_eq!((5, 7.0, "hello"), list);
    }
}
