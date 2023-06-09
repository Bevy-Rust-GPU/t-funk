use crate::{
    closure::{Closure, Curried2},
    macros::{arrow::Arrow, category::Category, Copointed, Pointed},
    typeclass::functor::Fmap,
};

/// A lens over getter `G` and setter `S`
pub type Lens<G, S> = Curried2<Lensed<G, Curried2<S>>>;

/// Construct a lens given instances of getter `G` and setter `S`
pub const fn lens<G, S>(get: G, set: S) -> Lens<G, S> {
    Curried2(Lensed(get, Curried2(set)))
}

/// Closure constructing a lens from a getter and setter
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Category,
    Arrow,
)]
pub struct Lensed<G, S>(pub G, pub S);

impl<G, S, F, T> Closure<(F, T)> for Lensed<G, S>
where
    T: Clone,
    G: Closure<T>,
    S: Closure<T>,
    F: Closure<G::Output>,
    F::Output: Fmap<S::Output>,
{
    type Output = <F::Output as Fmap<S::Output>>::Fmap;

    fn call(self, (f, t): (F, T)) -> Self::Output {
        f.call(self.0.call(t.clone())).fmap(self.1.call(t))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        closure::{Closure, Compose, Curry2},
        function::Const,
        macros::Lenses,
        typeclass::{functor::Const as FConst, monad::Identity},
    };

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Lenses)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Lenses)]
    struct Atom {
        element: &'static str,
        point: Point,
    }

    #[test]
    fn test_lens() {
        let atom = Atom {
            element: "foo",
            point: Point { x: 0.0, y: 0.0 },
        };

        let point = Point { x: 1.0, y: 2.0 };

        let identity = Atom::point.call(Identity).call(atom);
        let get = Atom::point.call(FConst).call(atom);
        let set = Atom::point
            .call(Identity.compose(Const.prefix2(point)))
            .call(atom);

        assert_eq!(identity, Identity(atom));
        assert_eq!(get, FConst(Point { x: 0.0, y: 0.0 }));
        assert_eq!(
            set,
            Identity(Atom {
                element: "foo",
                point
            })
        );
    }

    #[test]
    fn test_lens_composition() {
        let atom = Atom {
            element: "foo",
            point: Point { x: 0.0, y: 0.0 },
        };

        let atom_point_x_lens = Atom::point.compose(Point::x);

        let identity = atom_point_x_lens.call(Identity).call(atom);
        let get = atom_point_x_lens.call(FConst).call(atom);
        let set = atom_point_x_lens
            .call(Identity.compose(Const.prefix2(3.0)))
            .call(atom);

        assert_eq!(identity, Identity(atom));
        assert_eq!(get, FConst(0.0));
        assert_eq!(
            set,
            Identity(Atom {
                element: "foo",
                point: Point { x: 3.0, y: 0.0 }
            })
        );
    }
}
