use std::fmt::Debug;

use crate::cube::Cube;

pub trait Move: Debug + Clone {
    fn apply(&self, cube: &Cube) -> Cube;

    fn all_moves() -> [Self; 18];
}
