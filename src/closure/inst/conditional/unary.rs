use t_funk_macros::{lift, Arrow, Category};

use crate::closure::Closure;

/// Conditional closure that takes a single type, and returns that type directly
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow,
)]
pub struct If<T>(pub T, pub T);

impl<T> Closure<bool> for If<T> {
    type Output = T;

    fn call(self, input: bool) -> Self::Output {
        if input {
            self.0
        } else {
            self.1
        }
    }
}

#[lift]
pub fn make_if<T>(a: T, b: T) -> If<T> {
    If(a, b)
}

