pub use t_funk_macros::{
    define_adt, functions, impl_adt, lift, types, Closure, Copointed, Lenses, Pointed,
};

pub mod phantom {
    pub use t_funk_macros::{PhantomClone, PhantomCopy, PhantomDefault};
}

pub mod applicative {
    pub use t_funk_macros::{Applicative, Apply, Pure};
}

pub mod arrow {
    pub use t_funk_macros::{
        Arr, Arrow, ArrowFirst as First, ArrowSecond as Second, Fanout, Split,
    };
}

pub mod bifunctor {}

pub mod category {
    pub use t_funk_macros::{Category, Compose, Id};
}

pub mod foldable {
    pub use t_funk_macros::{Fold, FoldMap, Foldable, Foldl, Foldr};
}

pub mod functor {
    pub use t_funk_macros::{Fmap, Functor, Replace};
}

pub mod monad {
    pub use t_funk_macros::{Chain, Monad, Then};
}

pub mod monoid {
    pub use t_funk_macros::{Mconcat, Mempty, Monoid};
}

pub mod semigroup {
    pub use t_funk_macros::{Mappend, Semigroup};
}

pub mod traversable {}

/// Convert `do` notation into the equivalent
/// `x.chain(|y| z(y).chain(...))` composition.
///
/// Variable binding via `<-` uses unnameable closures,
/// so is not suitable for use in generic contexts.
#[macro_export]
macro_rules! do_monad {
    ($bind:ident <- $expr:expr; $($tt:tt)*) => {
        $expr.chain(move |$bind| do_monad!($($tt)*))
    };
    ($expr:expr; $($tt:tt)*) => {
        $expr.then(do_monad!($($tt)*))
    };
    ($expr:expr) => {
        $expr
    };
}
