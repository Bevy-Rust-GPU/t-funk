//! Type-level Set, which can contain at most one instance of a given type

use crate::{
    closure::{Closure, Compose, Composed, Curried2, Curry2, Flip, Flipped},
    macros::{functions, types},
};

#[functions]
pub trait Get<T> {
    fn get(self) -> T;
}

impl<T> Get<()> for T {
    fn get(self) -> () {
        ()
    }
}

impl<T, A, B> Get<(A, B)> for T
where
    T: Clone + Get<A> + Get<B>,
{
    fn get(self) -> (A, B) {
        (self.clone().get(), self.get())
    }
}

impl<T, A, B, C> Get<(A, B, C)> for T
where
    T: Clone + Get<A> + Get<B> + Get<C>,
{
    fn get(self) -> (A, B, C) {
        (self.clone().get(), self.clone().get(), self.clone().get())
    }
}

#[functions]
#[types]
pub trait Insert<T> {
    type Insert;

    fn insert(self, t: T) -> Self::Insert;
}

impl<T> Insert<()> for T {
    type Insert = Self;

    fn insert(self, (): ()) -> Self::Insert {
        self
    }
}

impl<T, A, B> Insert<(A, B)> for T
where
    T: Insert<A>,
    InsertT<T, A>: Insert<B>,
{
    type Insert = InsertT<InsertT<T, A>, B>;

    fn insert(self, (a, b): (A, B)) -> Self::Insert {
        self.insert(a).insert(b)
    }
}

impl<T, A, B, C> Insert<(A, B, C)> for T
where
    T: Insert<A>,
    InsertT<T, A>: Insert<B>,
    InsertT<InsertT<T, A>, B>: Insert<C>,
{
    type Insert = InsertT<InsertT<InsertT<T, A>, B>, C>;

    fn insert(self, (a, b, c): (A, B, C)) -> Self::Insert {
        self.insert(a).insert(b).insert(c)
    }
}

#[functions]
#[types]
pub trait Remove<T> {
    type Remove;
    fn remove(self) -> (Self::Remove, T);
}

impl<T> Remove<()> for T {
    type Remove = T;

    fn remove(self) -> (Self::Remove, ()) {
        (self, ())
    }
}

impl<T, A, B> Remove<(A, B)> for T
where
    T: Remove<A>,
    RemoveT<T, A>: Remove<B>,
{
    type Remove = RemoveT<RemoveT<T, A>, B>;

    fn remove(self) -> (Self::Remove, (A, B)) {
        let this = self;
        let (this, a) = Remove::<A>::remove(this);
        let (this, b) = Remove::<B>::remove(this);
        (this, (a, b))
    }
}

impl<T, A, B, C> Remove<(A, B, C)> for T
where
    T: Remove<A>,
    RemoveT<T, A>: Remove<B>,
    RemoveT<RemoveT<T, A>, B>: Remove<C>,
{
    type Remove = RemoveT<RemoveT<RemoveT<T, A>, B>, C>;

    fn remove(self) -> (Self::Remove, (A, B, C)) {
        let this = self;
        let (this, a) = Remove::<A>::remove(this);
        let (this, b) = Remove::<B>::remove(this);
        let (this, c) = Remove::<C>::remove(this);
        (this, (a, b, c))
    }
}

#[functions]
#[types]
pub trait Drop<U> {
    type Drop;

    fn drop(self) -> Self::Drop;
}

impl<T, U> Drop<U> for T
where
    T: Remove<U>,
{
    type Drop = RemoveT<T, U>;

    fn drop(self) -> Self::Drop {
        self.remove().0
    }
}

#[functions]
#[types]
pub trait Empty {
    type Empty;

    fn empty() -> Self::Empty;
}

#[functions]
#[types]
pub trait UnionWith<U> {
    type UnionWith;

    fn union_with(self, u: U) -> Self::UnionWith;
}

#[functions]
#[types]
pub trait Union<U> {
    type Union;

    fn union(self, u: U) -> Self::Union;
}

impl<T, U> Union<U> for T
where
    U: UnionWith<T>,
{
    type Union = UnionWithT<U, T>;

    fn union(self, u: U) -> Self::Union {
        u.union_with(self)
    }
}

#[functions]
#[types]
pub trait SubtractFrom<U> {
    type SubtractFrom;

    fn subtract_from(self, u: U) -> Self::SubtractFrom;
}

#[functions]
#[types]
pub trait Subtraction<U> {
    type Subtraction;

    fn subtraction(self, u: U) -> Self::Subtraction;
}

impl<T, U> Subtraction<U> for T
where
    U: SubtractFrom<T>,
{
    type Subtraction = SubtractFromT<U, T>;

    fn subtraction(self, u: U) -> Self::Subtraction {
        u.subtract_from(self)
    }
}

/// Lift function F(P) -> O into function F(S) -> O,
/// where F(S) -> O reads P from the type-level set S.
#[functions]
pub trait LiftGet<U> {
    type LiftGet;

    fn lift_get(self) -> Self::LiftGet;
}

pub type LiftGetT<T, U> = <T as LiftGet<U>>::LiftGet;

impl<T, U> LiftGet<U> for T
where
    T: Closure<U>,
{
    type LiftGet = Composed<T, GetF<U>>;

    fn lift_get(self) -> Self::LiftGet {
        GetF::<U>::default().compose_l(self)
    }
}

/// Lift function F(P) -> O into function F(P) -> F(S) -> S;
/// where F(S) -> S writes O into the type-level set S.
pub trait LiftSet<U, O> {
    type LiftSet;

    fn lift_set(self) -> Self::LiftSet;
}

pub type LiftSetT<T, U, O> = <T as LiftSet<U, O>>::LiftSet;

impl<T, U, O> LiftSet<U, O> for T
where
    T: Closure<U, Output = O>,
{
    type LiftSet = Composed<Curried2<Flipped<InsertF>>, T>;

    fn lift_set(self) -> Self::LiftSet {
        self.compose_l(InsertF.flip().curry2())
    }
}

#[functions]
pub trait LiftContext<U> {
    type LiftContext;

    fn lift_context(self) -> Self::LiftContext;
}

pub type LiftContextT<T, U> = <T as LiftContext<U>>::LiftContext;

impl<T, U> LiftContext<U> for T {
    type LiftContext = Composed<Curried2<Flipped<InsertF>>, Composed<T, GetF<U>>>;

    fn lift_context(self) -> Self::LiftContext {
        GetF::<U>::default()
            .compose_l(self)
            .compose_l(InsertF.flip().curry2())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    struct Context {
        int: i32,
        float: f32,
        char: char,
        string: &'static str,
    }

    impl Get<i32> for Context {
        fn get(self) -> i32 {
            self.int
        }
    }

    impl Get<f32> for Context {
        fn get(self) -> f32 {
            self.float
        }
    }

    impl Get<char> for Context {
        fn get(self) -> char {
            self.char
        }
    }

    impl Get<&'static str> for Context {
        fn get(self) -> &'static str {
            self.string
        }
    }

    impl Insert<i32> for Context {
        type Insert = Self;

        fn insert(self, t: i32) -> Self {
            Context { int: t, ..self }
        }
    }

    impl Insert<f32> for Context {
        type Insert = Self;

        fn insert(self, t: f32) -> Self {
            Context { float: t, ..self }
        }
    }

    impl Insert<char> for Context {
        type Insert = Self;

        fn insert(self, t: char) -> Self {
            Context { char: t, ..self }
        }
    }

    impl Insert<&'static str> for Context {
        type Insert = Self;

        fn insert(self, t: &'static str) -> Self {
            Context { string: t, ..self }
        }
    }

    #[test]
    fn test_set() {
        let ctx = Context {
            int: 1,
            float: 2.0,
            char: '3',
            string: "4",
        };

        let int = Get::<i32>::get(ctx);
        let float = Get::<f32>::get(ctx);
        let char = Get::<char>::get(ctx);
        let string = Get::<&'static str>::get(ctx);

        assert_eq!(int, 1);
        assert_eq!(float, 2.0);
        assert_eq!(char, '3');
        assert_eq!(string, "4");

        let ctx = Insert::<i32>::insert(ctx, 2);
        let ctx = Insert::<f32>::insert(ctx, 3.0);
        let ctx = Insert::<char>::insert(ctx, '4');
        let ctx = Insert::<&'static str>::insert(ctx, "5");

        assert_eq!(ctx.int, 2);
        assert_eq!(ctx.float, 3.0);
        assert_eq!(ctx.char, '4');
        assert_eq!(ctx.string, "5");
    }
}
