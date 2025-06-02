pub mod color;
pub mod r#move;
pub mod move_sequence;
pub mod piece;

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::LazyLock;

use color::Color::*;

use crate::cube::r#move::Move;
use crate::cube::r#move::Side;
use crate::cube::move_sequence::Sequence;
use crate::cube::piece::Corner;
use crate::cube::piece::Edge;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cube {
    corners: [Corner; 8],
    edges: [Edge; 12],
}

impl Cube {
    pub fn create_solved() -> Cube {
        Cube {
            corners: [
                Corner(WHITE, RED, GREEN),
                Corner(WHITE, BLUE, RED),
                Corner(WHITE, ORANGE, BLUE),
                Corner(WHITE, GREEN, ORANGE),
                Corner(YELLOW, GREEN, RED),
                Corner(YELLOW, RED, BLUE),
                Corner(YELLOW, BLUE, ORANGE),
                Corner(YELLOW, ORANGE, GREEN),
            ],
            edges: [
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
            ],
        }
    }

    pub fn rotate(&self, r#move: &Move) -> Self {
        match r#move.turns {
            r#move::Turn::RIGHT => self.rotate1(r#move),
            r#move::Turn::LEFT => self.rotate1(r#move).rotate1(r#move).rotate1(r#move),
            r#move::Turn::TWO => self.rotate1(r#move).rotate1(r#move),
        }
    }

    fn rotate1(&self, r#move: &Move) -> Self {
        let corner_permutation = PERMUTE_CORNERS_RIGHT.get(&r#move.side).unwrap();
        let corner_rotation = ROTATE_CORNERS_RIGHT.get(&r#move.side).unwrap();
        let new_corners: [Corner; 8] = std::array::from_fn(|index| {
            self.corners[corner_permutation[index]].rotate(corner_rotation[index])
        });

        let edge_permutation = PERMUTE_EDGES_RIGHT.get(&r#move.side).unwrap();
        let edge_rotation = ROTATE_EDGES_RIGHT.get(&r#move.side).unwrap();
        let new_edges: [Edge; 12] = std::array::from_fn(|index| {
            self.edges[edge_permutation[index]].flip(edge_rotation[index])
        });

        Self {
            corners: new_corners,
            edges: new_edges,
        }
    }

    pub fn apply(&self, scramble: &Sequence) -> Self {
        let mut new: Cube = *self;
        for r#move in scramble.into_iter() {
            new = new.rotate(&r#move);
        }
        new
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "   {}{}{}      \n   {}{}{}      \n   {}{}{}      \n{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}\n   {}{}{}      \n   {}{}{}      \n   {}{}{}      \n",
            self.corners[2].0,
            self.edges[2].0,
            self.corners[1].0,
            //
            self.edges[3].0,
            WHITE,
            self.edges[1].0,
            //
            self.corners[3].0,
            self.edges[0].0,
            self.corners[0].0,
            //
            //
            self.corners[2].1,
            self.edges[3].1,
            self.corners[3].2,
            //
            self.corners[3].1,
            self.edges[0].1,
            self.corners[0].2,
            //
            self.corners[0].1,
            self.edges[1].1,
            self.corners[1].2,
            //
            self.corners[1].1,
            self.edges[2].1,
            self.corners[2].2,
            //
            //
            self.edges[6].1,
            ORANGE,
            self.edges[7].1,
            //
            self.edges[7].0,
            GREEN,
            self.edges[4].0,
            //
            self.edges[4].1,
            RED,
            self.edges[5].1,
            //
            self.edges[5].0,
            BLUE,
            self.edges[6].0,
            //
            //
            self.corners[6].2,
            self.edges[11].1,
            self.corners[7].1,
            //
            self.corners[7].2,
            self.edges[8].1,
            self.corners[4].1,
            //
            self.corners[4].2,
            self.edges[9].1,
            self.corners[5].1,
            // 
            self.corners[5].2,
            self.edges[10].1,
            self.corners[6].1,
            // 
            // 
            self.corners[7].0,
            self.edges[8].0,
            self.corners[4].0,
            // 
            self.edges[11].0,
            YELLOW,
            self.edges[9].0,
            // 
            self.corners[6].0,
            self.edges[10].0,
            self.corners[5].0,
            

        )
    }
}

//ABCD EFGH IJKL MNOP QRST UVWX
//0123 ____ ____ ____ ____ 4567
static PERMUTE_CORNERS_RIGHT: LazyLock<HashMap<Side, [usize; 8]>> = LazyLock::new(|| {
    let mut map: HashMap<Side, [usize; 8]> = HashMap::new();
    map.insert(Side::UP, [1, 2, 3, 0, 4, 5, 6, 7]);
    map.insert(Side::DOWN, [0, 1, 2, 3, 7, 4, 5, 6]);
    map.insert(Side::FRONT, [3, 1, 2, 7, 0, 5, 6, 4]);
    map.insert(Side::BACK, [0, 5, 1, 3, 4, 6, 2, 7]);
    map.insert(Side::RIGHT, [4, 0, 2, 3, 5, 1, 6, 7]);
    map.insert(Side::LEFT, [0, 1, 6, 2, 4, 5, 7, 3]);
    map
});

static ROTATE_CORNERS_RIGHT: LazyLock<HashMap<Side, [u8; 8]>> = LazyLock::new(|| {
    let mut map: HashMap<Side, [u8; 8]> = HashMap::new();
    map.insert(Side::UP, [0, 0, 0, 0, 0, 0, 0, 0]);
    map.insert(Side::DOWN, [0, 0, 0, 0, 0, 0, 0, 0]);
    map.insert(Side::FRONT, [1, 0, 0, 2, 2, 0, 0, 1]);
    map.insert(Side::BACK, [0, 2, 1, 0, 0, 1, 2, 0]);
    map.insert(Side::RIGHT, [2, 1, 0, 0, 1, 2, 0, 0]);
    map.insert(Side::LEFT, [0, 0, 2, 1, 0, 0, 1, 2]);
    map
});

//ABCD EFGH IJKL MNOP QRST UVWX
//0123 _5_8 ____ _7_6 ____ 9012
static PERMUTE_EDGES_RIGHT: LazyLock<HashMap<Side, [usize; 12]>> = LazyLock::new(|| {
    let mut map: HashMap<Side, [usize; 12]> = HashMap::new();
    map.insert(Side::UP, [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11]);
    map.insert(Side::DOWN, [0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10]);
    map.insert(Side::FRONT, [7, 1, 2, 3, 0, 5, 6, 8, 4, 9, 10, 11]);
    map.insert(Side::BACK, [0, 1, 5, 3, 4, 10, 2, 7, 8, 9, 6, 11]);
    map.insert(Side::RIGHT, [0, 4, 2, 3, 9, 1, 6, 7, 8, 5, 10, 11]);
    map.insert(Side::LEFT, [0, 1, 2, 6, 4, 5, 11, 3, 8, 9, 10, 7]);
    map
});

static ROTATE_EDGES_RIGHT: LazyLock<HashMap<Side, [u8; 12]>> = LazyLock::new(|| {
    let mut map: HashMap<Side, [u8; 12]> = HashMap::new();
    map.insert(Side::UP, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    map.insert(Side::DOWN, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    map.insert(Side::FRONT, [1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0]);
    map.insert(Side::BACK, [0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0]);
    map.insert(Side::RIGHT, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    map.insert(Side::LEFT, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    map
});
