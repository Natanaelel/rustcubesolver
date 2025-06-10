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

    pub fn rotate(&self, r#move: &Move) -> Self {
        match r#move.turns {
            r#move::Turn::RIGHT => self.rotate1(r#move),
            r#move::Turn::LEFT => self.rotate1(r#move).rotate1(r#move).rotate1(r#move),
            r#move::Turn::TWO => self.rotate1(r#move).rotate1(r#move),
        }
    }

    fn rotate1(&self, r#move: &Move) -> Self {
        // let corner_permutation = PERMUTE_CORNERS_RIGHT.get(&r#move.side).unwrap();
        let corner_permutation = CORNER_PERMUTATION[r#move.side as usize];
        // let corner_rotation = ROTATE_CORNERS_RIGHT.get(&r#move.side).unwrap();
        let corner_rotation = CORNER_ORIENTATION[r#move.side as usize];

        // let edge_permutation = PERMUTE_EDGES_RIGHT.get(&r#move.side).unwrap();
        let edge_permutation = EDGE_PERMUTATION[r#move.side as usize];
        // let edge_rotation = ROTATE_EDGES_RIGHT.get(&r#move.side).unwrap();
        let edge_rotation = EDGE_ORIENTATION[r#move.side as usize];

        let new_corner_permutation: [u8; 8] =
            std::array::from_fn(|index| self.corner_permutation[corner_permutation[index]]);
        let new_corner_orientation: [u8; 8] = std::array::from_fn(|index| {
            (self.corner_orientation[corner_permutation[index]] + corner_rotation[index]) % 3
        });

        let new_edge_permutation: [u8; 12] =
            std::array::from_fn(|index| self.edge_permutation[edge_permutation[index]]);
        let new_edge_orientation: [u8; 12] = std::array::from_fn(|index| {
            (self.edge_orientation[edge_permutation[index]] + edge_rotation[index]) % 2
        });

        // let new_edges: [Edge; 12] = std::array::from_fn(|index| {
        //     self.edges[edge_permutation[index]].flip(edge_rotation[index])
        // });

        // Self {
        //     corners: new_corners,
        //     edges: new_edges,
        // }

        Self {
            corner_permutation: new_corner_permutation,
            corner_orientation: new_corner_orientation,
            edge_permutation: new_edge_permutation,
            edge_orientation: new_edge_orientation,
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

static CORNER_PERMUTATION: [[usize; 8]; 6] = [
    [1, 2, 3, 0, 4, 5, 6, 7],
    [0, 1, 2, 3, 7, 4, 5, 6],
    [3, 1, 2, 7, 0, 5, 6, 4],
    [0, 5, 1, 3, 4, 6, 2, 7],
    [4, 0, 2, 3, 5, 1, 6, 7],
    [0, 1, 6, 2, 4, 5, 7, 3],
];

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

static CORNER_ORIENTATION: [[u8; 8]; 6] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 2, 2, 0, 0, 1],
    [0, 2, 1, 0, 0, 1, 2, 0],
    [2, 1, 0, 0, 1, 2, 0, 0],
    [0, 0, 2, 1, 0, 0, 1, 2],
];

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

static EDGE_PERMUTATION: [[usize; 12]; 6] = [
    [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11],
    [0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10],
    [7, 1, 2, 3, 0, 5, 6, 8, 4, 9, 10, 11],
    [0, 1, 5, 3, 4, 10, 2, 7, 8, 9, 6, 11],
    [0, 4, 2, 3, 9, 1, 6, 7, 8, 5, 10, 11],
    [0, 1, 2, 6, 4, 5, 11, 3, 8, 9, 10, 7],
];

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

static EDGE_ORIENTATION: [[u8; 12]; 6] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];
