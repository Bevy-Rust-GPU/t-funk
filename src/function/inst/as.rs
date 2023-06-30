use core::marker::PhantomData;

use t_funk_macros::Closure;

use crate::function::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct As<T>(PhantomData<T>);

macro_rules! impl_as {
    ($f:ident, $t:ident) => {
        impl Function<$f> for As<$t> {
            type Output = $t;

            fn call(input: $f) -> Self::Output {
                input as $t
            }
        }
    };
}

// bool
impl_as!(bool, u8);
impl_as!(bool, u16);
impl_as!(bool, u32);
impl_as!(bool, u64);
impl_as!(bool, usize);

impl_as!(bool, i8);
impl_as!(bool, i16);
impl_as!(bool, i32);
impl_as!(bool, i64);
impl_as!(bool, isize);

// u8
impl_as!(u8, u8);
impl_as!(u8, u16);
impl_as!(u8, u32);
impl_as!(u8, u64);
impl_as!(u8, usize);

impl_as!(u8, i8);
impl_as!(u8, i16);
impl_as!(u8, i32);
impl_as!(u8, i64);
impl_as!(u8, isize);

impl_as!(u8, f32);
impl_as!(u8, f64);

// u16
impl_as!(u16, u8);
impl_as!(u16, u16);
impl_as!(u16, u32);
impl_as!(u16, u64);
impl_as!(u16, usize);

impl_as!(u16, i8);
impl_as!(u16, i16);
impl_as!(u16, i32);
impl_as!(u16, i64);
impl_as!(u16, isize);

impl_as!(u16, f32);
impl_as!(u16, f64);

// u32
impl_as!(u32, u8);
impl_as!(u32, u16);
impl_as!(u32, u32);
impl_as!(u32, u64);
impl_as!(u32, usize);

impl_as!(u32, i8);
impl_as!(u32, i16);
impl_as!(u32, i32);
impl_as!(u32, i64);
impl_as!(u32, isize);

impl_as!(u32, f32);
impl_as!(u32, f64);

// u64
impl_as!(u64, u8);
impl_as!(u64, u16);
impl_as!(u64, u32);
impl_as!(u64, u64);
impl_as!(u64, usize);

impl_as!(u64, i8);
impl_as!(u64, i16);
impl_as!(u64, i32);
impl_as!(u64, i64);
impl_as!(u64, isize);

impl_as!(u64, f32);
impl_as!(u64, f64);

// usize
impl_as!(usize, u8);
impl_as!(usize, u16);
impl_as!(usize, u32);
impl_as!(usize, u64);
impl_as!(usize, usize);

impl_as!(usize, i8);
impl_as!(usize, i16);
impl_as!(usize, i32);
impl_as!(usize, i64);
impl_as!(usize, isize);

impl_as!(usize, f32);
impl_as!(usize, f64);

// i8
impl_as!(i8, u8);
impl_as!(i8, u16);
impl_as!(i8, u32);
impl_as!(i8, u64);
impl_as!(i8, usize);

impl_as!(i8, i8);
impl_as!(i8, i16);
impl_as!(i8, i32);
impl_as!(i8, i64);
impl_as!(i8, isize);

impl_as!(i8, f32);
impl_as!(i8, f64);

// i16
impl_as!(i16, u8);
impl_as!(i16, u16);
impl_as!(i16, u32);
impl_as!(i16, u64);
impl_as!(i16, usize);

impl_as!(i16, i8);
impl_as!(i16, i16);
impl_as!(i16, i32);
impl_as!(i16, i64);
impl_as!(i16, isize);

impl_as!(i16, f32);
impl_as!(i16, f64);

// i32
impl_as!(i32, u8);
impl_as!(i32, u16);
impl_as!(i32, u32);
impl_as!(i32, u64);
impl_as!(i32, usize);

impl_as!(i32, i8);
impl_as!(i32, i16);
impl_as!(i32, i32);
impl_as!(i32, i64);
impl_as!(i32, isize);

impl_as!(i32, f32);
impl_as!(i32, f64);

// i64
impl_as!(i64, u8);
impl_as!(i64, u16);
impl_as!(i64, u32);
impl_as!(i64, u64);
impl_as!(i64, usize);

impl_as!(i64, i8);
impl_as!(i64, i16);
impl_as!(i64, i32);
impl_as!(i64, i64);
impl_as!(i64, isize);

impl_as!(i64, f32);
impl_as!(i64, f64);

// isize
impl_as!(isize, u8);
impl_as!(isize, u16);
impl_as!(isize, u32);
impl_as!(isize, u64);
impl_as!(isize, usize);

impl_as!(isize, i8);
impl_as!(isize, i16);
impl_as!(isize, i32);
impl_as!(isize, i64);
impl_as!(isize, isize);

impl_as!(isize, f32);
impl_as!(isize, f64);
