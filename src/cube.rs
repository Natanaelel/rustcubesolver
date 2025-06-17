pub mod color;
pub mod r#move;
pub mod simple_move;
pub mod move_sequence;
pub mod piece;

use std::fmt::Display;

use color::Color::*;

use crate::cube::r#move::Move;
use crate::cube::move_sequence::Sequence;
use crate::cube::piece::Corner;
use crate::cube::piece::Edge;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cube {
    corner_permutation: [u8; 8],
    corner_orientation: [u8; 8],
    edge_permutation: [u8; 12],
    edge_orientation: [u8; 12],
}

impl Cube {
    fn create_solved_corners() -> [Corner; 8] {
        [
            Corner(WHITE, RED, GREEN),
            Corner(WHITE, BLUE, RED),
            Corner(WHITE, ORANGE, BLUE),
            Corner(WHITE, GREEN, ORANGE),
            Corner(YELLOW, GREEN, RED),
            Corner(YELLOW, RED, BLUE),
            Corner(YELLOW, BLUE, ORANGE),
            Corner(YELLOW, ORANGE, GREEN),
        ]
    }
    fn create_solved_edges() -> [Edge; 12] {
        [
            Edge(WHITE, GREEN),
            Edge(WHITE, RED),
            Edge(WHITE, BLUE),
            Edge(WHITE, ORANGE),
            Edge(GREEN, RED),
            Edge(BLUE, RED),
            Edge(BLUE, ORANGE),
            Edge(GREEN, ORANGE),
            Edge(YELLOW, GREEN),
            Edge(YELLOW, RED),
            Edge(YELLOW, BLUE),
            Edge(YELLOW, ORANGE),
        ]
    }
    pub fn create_solved() -> Cube {
        Cube {
            corner_permutation: [0, 1, 2, 3, 4, 5, 6, 7],
            corner_orientation: [0; 8],
            edge_permutation: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            edge_orientation: [0; 12],
        }
    }

    pub fn apply_move<T: Move>(&self, r#move: &T) -> Self {
        r#move.apply(self)
    }

    

    pub fn apply<T: Move>(&self, scramble: &Sequence<T>) -> Self {
        let mut new: Cube = self.clone();
        for r#move in scramble.into_iter() {
            new = new.apply_move(&r#move);
        }
        new
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let corners: [Corner; 8] = Cube::create_solved_corners();
        let corners: [Corner; 8] = std::array::from_fn(|index| {
            corners[self.corner_permutation[index] as usize].rotate(self.corner_orientation[index])
        });

        let edges: [Edge; 12] = Cube::create_solved_edges();
        let edges: [Edge; 12] = std::array::from_fn(|index| {
            edges[self.edge_permutation[index] as usize].flip(self.edge_orientation[index])
        });

        write!(
            f,
            "   {}{}{}      \n   {}{}{}      \n   {}{}{}      \n{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}\n   {}{}{}      \n   {}{}{}      \n   {}{}{}      \n",
            corners[2].0,
            edges[2].0,
            corners[1].0,
            //
            edges[3].0,
            WHITE,
            edges[1].0,
            //
            corners[3].0,
            edges[0].0,
            corners[0].0,
            //
            //
            corners[2].1,
            edges[3].1,
            corners[3].2,
            //
            corners[3].1,
            edges[0].1,
            corners[0].2,
            //
            corners[0].1,
            edges[1].1,
            corners[1].2,
            //
            corners[1].1,
            edges[2].1,
            corners[2].2,
            //
            //
            edges[6].1,
            ORANGE,
            edges[7].1,
            //
            edges[7].0,
            GREEN,
            edges[4].0,
            //
            edges[4].1,
            RED,
            edges[5].1,
            //
            edges[5].0,
            BLUE,
            edges[6].0,
            //
            //
            corners[6].2,
            edges[11].1,
            corners[7].1,
            //
            corners[7].2,
            edges[8].1,
            corners[4].1,
            //
            corners[4].2,
            edges[9].1,
            corners[5].1,
            //
            corners[5].2,
            edges[10].1,
            corners[6].1,
            //
            //
            corners[7].0,
            edges[8].0,
            corners[4].0,
            //
            edges[11].0,
            YELLOW,
            edges[9].0,
            //
            corners[6].0,
            edges[10].0,
            corners[5].0,
        )
    }
}
