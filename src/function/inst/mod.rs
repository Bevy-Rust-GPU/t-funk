mod r#as;
mod from;
mod into;
mod replicate_m;
mod unfold_m;

pub use from::*;
pub use into::*;
pub use r#as::*;
pub use replicate_m::*;
pub use unfold_m::*;

use core::fmt::{Debug, Display};

use t_funk::macros::lift;

/// Return the absolute value of a float
#[lift]
pub fn abs<T>(input: T) -> T
where
    T: Default + PartialOrd + core::ops::Neg<Output = T>,
{
    if input < Default::default() {
        -input
    } else {
        input
    }
}

/// Add two values
#[lift]
pub fn add<A, B>(a: A, b: B) -> AddT<A, B>
where
    A: core::ops::Add<B>,
{
    a + b
}

pub type AddT<A, B> = <A as core::ops::Add<B>>::Output;

/// Return the value of the first argument
#[lift]
pub fn r#const<A, B>(a: A, _b: B) -> A {
    a
}

/// Divide two values
#[lift]
pub fn div<A, B>(a: A, b: B) -> DivT<A, B>
where
    A: core::ops::Div<B>,
{
    a.div(b)
}

pub type DivT<A, B> = <A as core::ops::Div<B>>::Output;

/// Return true if both arguments are equal
#[lift]
pub fn eq<A, B>(a: A, b: B) -> bool
where
    A: PartialEq<B>,
{
    a.eq(&b)
}

/// Reverse a 2-tuple
#[lift]
pub fn flip_tuple<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

/// Extract the first component of a 2-tuple
#[lift]
pub fn fst<A, B>(a: A, _b: B) -> A {
    a
}

/// Return true if B is greater than A
#[lift]
pub fn gt<A, B>(a: A, b: B) -> bool
where
    A: PartialOrd<B>,
{
    a.gt(&b)
}

/// Return the provided input
#[lift]
pub fn id<T>(input: T) -> T {
    input
}

/// Return true if B is lesser than A
#[lift]
pub fn lt<A, B>(a: A, b: B) -> bool
where
    A: PartialOrd<B>,
{
    a.lt(&b)
}

/// Make a tuple from two values
#[lift]
pub fn make_pair<A, B>(a: A, b: B) -> (A, B) {
    (a, b)
}

/// Return the greater of the provided inputs
#[lift]
pub fn max(a: f32, b: f32) -> f32 {
    a.max(b)
}

/// Return the lesser of the provided inputs
#[lift]
pub fn min(a: f32, b: f32) -> f32 {
    a.min(b)
}

/// Multiply two values
#[lift]
pub fn mul<A, B>(a: A, b: B) -> MulT<A, B>
where
    A: core::ops::Mul<B>,
{
    a.mul(b)
}

pub type MulT<A, B> = <A as core::ops::Mul<B>>::Output;

/// Negate the provided input
#[lift]
pub fn neg<T>(t: T) -> NegT<T>
where
    T: core::ops::Neg,
{
    t.neg()
}

pub type NegT<T> = <T as core::ops::Neg>::Output;

/// Print the provided input to stdout
#[cfg(feature = "std")]
#[lift]
pub fn print_ln<T>(t: T)
where
    T: Display,
{
    std::println!("{t:}")
}

/// Format the provided input using Display
#[lift]
#[cfg(feature = "alloc")]
pub fn format_display<T>(t: T) -> alloc::string::String
where
    T: Display,
{
    std::format!("{t:}")
}

/// Format the provided input using Debug
#[lift]
#[cfg(feature = "alloc")]
pub fn format_debug<T>(t: T) -> alloc::string::String
where
    T: Debug,
{
    std::format!("{t:?}")
}

/// Format the provided input using Debug with multiline semantics
#[lift]
#[cfg(feature = "alloc")]
pub fn format_debug_multiline<T>(t: T) -> alloc::string::String
where
    T: Debug,
{
    std::format!("{t:#?}")
}

/// Unwrap the provided Result
#[lift]
pub fn result_unwrap<T, E>(t: Result<T, E>) -> T
where
    E: core::fmt::Debug,
{
    t.unwrap()
}

/// Shift the provided nested triple to the right
#[lift]
pub fn r_shift_tuple<A, B, C>(a: A, (b, c): (B, C)) -> ((A, B), C) {
    ((a, b), c)
}

/// Shift the provided nested triple to the left
#[lift]
pub fn l_shift_tuple<A, B, C>((a, b): (A, B), c: C) -> (A, (B, C)) {
    (a, (b, c))
}

/// Return the value of the second argument
#[lift]
pub fn snd<A, B>(_a: A, b: B) -> B {
    b
}

/// Subtract two values
#[lift]
pub fn sub<A, B>(a: A, b: B) -> SubT<A, B>
where
    A: core::ops::Sub<B>,
{
    a.sub(b)
}

pub type SubT<A, B> = <A as core::ops::Sub<B>>::Output;

/// Swap the elements of a 2-tuple
#[lift]
pub fn swap<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

/// Convert the provided input into a String
#[lift]
#[cfg(feature = "alloc")]
pub fn to_string<T>(t: T) -> alloc::string::String
where
    T: alloc::string::ToString,
{
    t.to_string()
}

/// Given a square matrix, transpose it
#[lift]
pub fn transpose<A, B, C, D>((a, b): (A, B), (c, d): (C, D)) -> ((A, C), (B, D)) {
    ((a, c), (b, d))
}
