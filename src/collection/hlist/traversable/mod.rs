mod sequence_a;
mod traverse;

pub use sequence_a::*;
pub use traverse::*;

#[cfg(test)]
mod test {
    use crate::{
        collection::{hlist::Nil, tlist::ToHList},
        function::Id,
        typeclass::traversable::{SequenceA, Traverse},
    };

    #[test]
    fn test_traverse() {
        let list = ((0,).to_hlist(), (0, 1).to_hlist(), (0, 1, 2).to_hlist()).to_hlist();
        let decafisbad = Traverse::<Id, Nil>::traverse(list, Id);
        assert_eq!(
            decafisbad,
            (
                (0, 0, 0).to_hlist(),
                (0, 0, 1).to_hlist(),
                (0, 0, 2).to_hlist(),
                (0, 1, 0).to_hlist(),
                (0, 1, 1).to_hlist(),
                (0, 1, 2).to_hlist()
            )
                .to_hlist()
        );
    }

    #[test]
    fn test_sequence_a() {
        let list = ((0,).to_hlist(), (0, 1).to_hlist(), (0, 1, 2).to_hlist()).to_hlist();
        let decafisbad = SequenceA::<Nil>::sequence_a(list);
        assert_eq!(
            decafisbad,
            (
                (0, 0, 0).to_hlist(),
                (0, 0, 1).to_hlist(),
                (0, 0, 2).to_hlist(),
                (0, 1, 0).to_hlist(),
                (0, 1, 1).to_hlist(),
                (0, 1, 2).to_hlist()
            )
                .to_hlist()
        );
    }
}
