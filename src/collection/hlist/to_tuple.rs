use crate::{
    collection::{hlist::Nil, tlist::Tuple},
    function::Function,
};

/// A cons list that can be converted into a flat tuple,
/// ex. `(1, (2, (3, (4, ())))) -> (1, 2, 3, 4)`
///
/// This is a special case, in that it must be implemented via macro
/// for the sake of having a known fixed-arity tuple type to return.
pub trait ToTuple {
    type Tuple: Tuple<HList = Self>;

    fn to_tuple(self) -> Self::Tuple;
}

impl ToTuple for Nil {
    type Tuple = ();

    fn to_tuple(self) -> Self::Tuple {
        ()
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::{hlist::ToTuple, tlist::ToHList};

    #[test]
    fn test_uncons() {
        let consable = (1, 2, 3);
        assert_eq!(consable, consable.to_hlist().to_tuple());
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct UnconsF;

impl<I> Function<I> for UnconsF
where
    I: ToTuple,
{
    type Output = I::Tuple;

    fn call(input: I) -> Self::Output {
        input.to_tuple()
    }
}
