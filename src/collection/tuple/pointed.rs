use crate::typeclass::pointed::Pointed;

impl<T> Pointed for (T,) {
    type Pointed = T;

    fn point(unit: Self::Pointed) -> Self {
        (unit,)
    }
}

