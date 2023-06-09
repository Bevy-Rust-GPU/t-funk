/// Macros for composing HList literals.

/// Takes a flat list of tokens and nests it into a HList.
#[macro_export]
macro_rules! cons {
    ($($exprs:expr),*) => {
        crate::cons_expr!($($exprs),*)
    };
    ($($tys:ty),*) => {
        crate::cons_ty!($($tys),*)
    };
    ($($idents:ident),*) => {
        crate::cons_ident!($($idents),*)
    };
}

/// Takes a flat list of identifiers and nests it into a HList.
#[macro_export]
macro_rules! cons_ident {
    ($ident:ident $(, $idents:ident)+) => {
        crate::collection::hlist::Cons($ident, crate::cons_ident!($($idents),*))
    };
    ($ident:ident) => {
        crate::collection::hlist::Cons($ident, crate::collection::hlist::Nil)
    };
    () => {};
}

/// Takes a flat list of expressions and nests it into a HList.
#[macro_export]
macro_rules! cons_expr {
    ($expr:expr $(, $exprs:expr)+) => {
        Cons($expr, crate::cons_expr!($($exprs),*))
    };
    ($expr:expr) => {
        Cons($expr, ())
    };
    () => {};
}

/// Takes a flat list of types and nests it into a HList.
#[macro_export]
macro_rules! cons_ty {
    ($ty:ty $(, $tys:ty)+) => {
        crate::collection::hlist::Cons<$ty, crate::cons_ty!($($tys),*)>
    };
    ($ty:ty) => {
        crate::collection::hlist::Cons<$ty, crate::collection::hlist::Nil>
    };
    () => {};
}

/// Calls the provided macro once for each sub-list in the provided list.
#[macro_export]
macro_rules! macro_cons {
    ($call:ident !, $ident:tt, $($idents:tt),+) => {
        $call!($ident, $($idents),+);
        macro_cons!($call !, $($idents),+);
    };
    ($call:ident !, $ident:tt) => {
        $call!($ident);
    }
}
