use crate::{
    collection::hlist::{Cons, Nil},
    typeclass::functor::Replace,
};
impl<T, N, U> Replace<U> for Cons<T, N>
where
    U: Clone,
    N: Replace<U>,
{
    type Replace = Cons<U, N::Replace>;

    fn replace(self, t: U) -> Self::Replace {
        Cons(t.clone(), self.1.replace(t))
    }
}

impl<T> Replace<T> for Nil {
    type Replace = Nil;

    fn replace(self, _: T) -> Self::Replace {
        self
    }
}

