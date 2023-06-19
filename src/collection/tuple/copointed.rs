use crate::typeclass::copointed::Copointed;

impl<T> Copointed for (T,) {
    type Copointed = T;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

