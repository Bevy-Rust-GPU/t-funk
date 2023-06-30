#[macro_export]
macro_rules! impl_tuple {
    ($macro:ident => ) => {
        $macro!();
    };
    ($macro:ident => $ident:ident $(, $rest:ident)*) => {
        $macro!($ident $(, $rest)*);
        impl_tuple!($macro => $($rest),*);
    };
}

mod applicative;
mod copointed;
mod foldable;
mod functor;
mod monad;
mod monoid;
mod pointed;
mod semigroup;

mod push_front;
pub use push_front::*;
