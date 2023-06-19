//! Type-level map, which associates type keys with values

use crate::{
    macros::{functions, types},
    peano::*,
};

#[functions]
#[types]
pub trait Get<K> {
    type Get;

    fn get(self) -> Self::Get;
}

#[functions]
#[types]
pub trait Insert<K, V> {
    type Insert;

    fn insert(self, t: V) -> Self::Insert;
}

#[functions]
#[types]
pub trait Remove<K> {
    type Removed;
    type Remove;

    fn remove(self) -> (Self::Removed, Self::Remove);
}

#[functions]
#[types]
pub trait Drop<K> {
    type Drop;

    fn drop(self) -> Self::Drop;
}

impl<T, K> Drop<K> for T
where
    T: Remove<K>,
{
    type Drop = RemovedT<T, K>;

    fn drop(self) -> Self::Drop {
        self.remove().0
    }
}

pub trait Empty {
    type Empty;

    fn empty() -> Self::Empty;
}

macro_rules! impl_get {
    (($($ts:tt)*) => ($p:ident, $i:ident)) => {
        impl<$($ts)*> Get<$p> for ($($ts)*) {
            type Get = $i;

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            fn get(self) -> Self::Get {
                let ($($ts)*) = self;
                $i
            }
        }
    };
}

impl_get!((A,) => (P0, A));

impl_get!((A, B) => (P0, A));
impl_get!((A, B) => (P1, B));

impl_get!((A, B, C) => (P0, A));
impl_get!((A, B, C) => (P1, B));
impl_get!((A, B, C) => (P2, C));

impl_get!((A, B, C, D) => (P0, A));
impl_get!((A, B, C, D) => (P1, B));
impl_get!((A, B, C, D) => (P2, C));
impl_get!((A, B, C, D) => (P3, D));

impl_get!((A, B, C, D, E) => (P0, A));
impl_get!((A, B, C, D, E) => (P1, B));
impl_get!((A, B, C, D, E) => (P2, C));
impl_get!((A, B, C, D, E) => (P3, D));
impl_get!((A, B, C, D, E) => (P4, E));

impl_get!((A, B, C, D, E, F) => (P0, A));
impl_get!((A, B, C, D, E, F) => (P1, B));
impl_get!((A, B, C, D, E, F) => (P2, C));
impl_get!((A, B, C, D, E, F) => (P3, D));
impl_get!((A, B, C, D, E, F) => (P4, E));
impl_get!((A, B, C, D, E, F) => (P5, F));

#[cfg(test)]
mod test {
    use super::*;

    enum Int {}
    enum Float {}
    enum Char {}
    enum String {}

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    //#[map]
    struct ContextInner<A, B> {
        //#[key = Int]
        int: A,
        //#[key = Float]
        float: B,
    }

    impl<A, B> Get<Int> for ContextInner<A, B> {
        type Get = A;

        fn get(self) -> Self::Get {
            self.int
        }
    }

    impl<A, B> Get<Float> for ContextInner<A, B> {
        type Get = B;

        fn get(self) -> Self::Get {
            self.float
        }
    }

    impl<A, B, T> Insert<Int, T> for ContextInner<A, B> {
        type Insert = ContextInner<T, B>;

        fn insert(self, t: T) -> Self::Insert {
            ContextInner {
                int: t,
                float: self.float,
            }
        }
    }

    impl<A, B, T> Insert<Float, T> for ContextInner<A, B> {
        type Insert = ContextInner<A, T>;

        fn insert(self, t: T) -> Self::Insert {
            ContextInner {
                int: self.int,
                float: t,
            }
        }
    }

    impl<A, B> Remove<Int> for ContextInner<A, B> {
        type Removed = ContextInner<(), B>;
        type Remove = A;

        fn remove(self) -> (Self::Removed, Self::Remove) {
            let ContextInner { int, float } = self;

            (ContextInner { int: (), float }, int)
        }
    }

    impl<A, B> Remove<Float> for ContextInner<A, B> {
        type Removed = ContextInner<A, ()>;
        type Remove = B;

        fn remove(self) -> (Self::Removed, Self::Remove) {
            let ContextInner { int, float } = self;

            (ContextInner { int, float: () }, float)
        }
    }

    impl<A, B> Empty for ContextInner<A, B> {
        type Empty = ContextInner<(), ()>;

        fn empty() -> Self::Empty {
            ContextInner { int: (), float: () }
        }
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    //#[map]
    struct ContextOuter<A, B, C, D> {
        //#[map = [A = Int, B = Float]]
        context_inner: ContextInner<A, B>,
        //#[key = Char]
        char: C,
        //#[key = String]
        string: D,
    }

    impl<A, B, C, D> Get<Int> for ContextOuter<A, B, C, D> {
        type Get = A;

        fn get(self) -> Self::Get {
            Get::<Int>::get(self.context_inner)
        }
    }

    impl<A, B, C, D> Get<Float> for ContextOuter<A, B, C, D> {
        type Get = B;

        fn get(self) -> Self::Get {
            Get::<Float>::get(self.context_inner)
        }
    }

    impl<A, B, C, D> Get<Char> for ContextOuter<A, B, C, D> {
        type Get = C;

        fn get(self) -> Self::Get {
            self.char
        }
    }

    impl<A, B, C, D> Get<String> for ContextOuter<A, B, C, D> {
        type Get = D;

        fn get(self) -> Self::Get {
            self.string
        }
    }

    impl<A, B, C, D, T> Insert<Int, T> for ContextOuter<A, B, C, D> {
        type Insert = ContextOuter<T, B, C, D>;

        fn insert(self, t: T) -> Self::Insert {
            let ContextOuter {
                context_inner,
                char,
                string,
            } = self;

            ContextOuter {
                context_inner: Insert::<Int, T>::insert(context_inner, t),
                char,
                string,
            }
        }
    }

    impl<A, B, C, D, T> Insert<Float, T> for ContextOuter<A, B, C, D> {
        type Insert = ContextOuter<A, T, C, D>;

        fn insert(self, t: T) -> Self::Insert {
            let ContextOuter {
                context_inner,
                char,
                string,
            } = self;

            ContextOuter {
                context_inner: Insert::<Float, T>::insert(context_inner, t),
                char,
                string,
            }
        }
    }

    impl<A, B, C, D, T> Insert<Char, T> for ContextOuter<A, B, C, D> {
        type Insert = ContextOuter<A, B, T, D>;

        fn insert(self, t: T) -> Self::Insert {
            ContextOuter {
                context_inner: self.context_inner,
                char: t,
                string: self.string,
            }
        }
    }

    impl<A, B, C, D, T> Insert<String, T> for ContextOuter<A, B, C, D> {
        type Insert = ContextOuter<A, B, C, T>;

        fn insert(self, t: T) -> Self::Insert {
            ContextOuter {
                context_inner: self.context_inner,
                char: self.char,
                string: t,
            }
        }
    }

    impl<A, B, C, D> Remove<Int> for ContextOuter<A, B, C, D> {
        type Removed = ContextOuter<(), B, C, D>;
        type Remove = A;

        fn remove(self) -> (Self::Removed, Self::Remove) {
            let ContextOuter {
                context_inner,
                char,
                string,
            } = self;

            let (context_inner, int) = Remove::<Int>::remove(context_inner);

            (
                ContextOuter {
                    context_inner,
                    char,
                    string,
                },
                int,
            )
        }
    }

    impl<A, B, C, D> Remove<Float> for ContextOuter<A, B, C, D> {
        type Removed = ContextOuter<A, (), C, D>;
        type Remove = B;

        fn remove(self) -> (Self::Removed, Self::Remove) {
            let ContextOuter {
                context_inner,
                char,
                string,
            } = self;

            let (context_inner, float) = Remove::<Float>::remove(context_inner);

            (
                ContextOuter {
                    context_inner,
                    char,
                    string,
                },
                float,
            )
        }
    }

    impl<A, B, C, D> Remove<Char> for ContextOuter<A, B, C, D> {
        type Removed = ContextOuter<A, B, (), D>;
        type Remove = C;

        fn remove(self) -> (Self::Removed, Self::Remove) {
            let ContextOuter {
                context_inner,
                char,
                string,
            } = self;

            (
                ContextOuter {
                    context_inner,
                    char: (),
                    string,
                },
                char,
            )
        }
    }

    impl<A, B, C, D> Remove<String> for ContextOuter<A, B, C, D> {
        type Removed = ContextOuter<A, B, C, ()>;
        type Remove = D;

        fn remove(self) -> (Self::Removed, Self::Remove) {
            let ContextOuter {
                context_inner,
                char,
                string,
            } = self;

            (
                ContextOuter {
                    context_inner,
                    char,
                    string: (),
                },
                string,
            )
        }
    }

    impl<A, B, C, D> Empty for ContextOuter<A, B, C, D> {
        type Empty = ContextOuter<(), (), (), ()>;

        fn empty() -> Self::Empty {
            ContextOuter {
                context_inner: <ContextInner<A, B> as Empty>::empty(),
                char: (),
                string: (),
            }
        }
    }

    #[test]
    fn test_map() {
        let context = ContextOuter {
            context_inner: ContextInner { int: 1, float: 2.0 },
            char: '3',
            string: "4",
        };

        let int = Get::<Int>::get(context);
        let float = Get::<Float>::get(context);
        let char = Get::<Char>::get(context);
        let string = Get::<String>::get(context);

        assert_eq!(int, 1);
        assert_eq!(float, 2.0);
        assert_eq!(char, '3');
        assert_eq!(string, "4");

        let inserted = Insert::<Int, i32>::insert(context, 2);
        let inserted = Insert::<Float, _>::insert(inserted, 3.0);
        let inserted = Insert::<Char, _>::insert(inserted, '4');
        let inserted = Insert::<String, _>::insert(inserted, "5");

        assert_eq!(inserted.context_inner.int, 2);
        assert_eq!(inserted.context_inner.float, 3.0);
        assert_eq!(inserted.char, '4');
        assert_eq!(inserted.string, "5");

        let (removed, int) = Remove::<Int>::remove(context);
        let (removed, float) = Remove::<Float>::remove(removed);
        let (removed, char) = Remove::<Char>::remove(removed);
        let (removed, string) = Remove::<String>::remove(removed);

        assert_eq!(int, 1);
        assert_eq!(float, 2.0);
        assert_eq!(char, '3');
        assert_eq!(string, "4");

        assert_eq!(removed.context_inner.int, ());
        assert_eq!(removed.context_inner.float, ());
        assert_eq!(removed.char, ());
        assert_eq!(removed.string, ());

        let empty = ContextOuter::<i32, f32, char, &'static str>::empty();
        assert_eq!(
            empty,
            ContextOuter {
                context_inner: ContextInner { int: (), float: () },
                char: (),
                string: ()
            }
        );
    }
}
