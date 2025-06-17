use std::{fmt::Display, str::FromStr};

use rand::distr::Distribution;

use crate::cube::{Cube, r#move::Move};

#[derive(Debug, Clone)]
pub struct SimpleMove {
    pub side: Side,
    pub turns: Turn,
}

impl SimpleMove {
    // pub fn invert(&self) -> SimpleMove {
    //     SimpleMove {
    //         side: self.side.clone(),
    //         turns: match self.turns {
    //             Turn::RIGHT => Turn::LEFT,
    //             Turn::LEFT => Turn::RIGHT,
    //             Turn::TWO => Turn::TWO,
    //         },
    //     }
    // }

    fn rotate1(&self, cube: &Cube) -> Cube {
        let corner_permutation = CORNER_PERMUTATION[self.side as usize];
        let corner_rotation = CORNER_ORIENTATION[self.side as usize];

        let edge_permutation = EDGE_PERMUTATION[self.side as usize];
        let edge_rotation = EDGE_ORIENTATION[self.side as usize];

        let new_corner_permutation: [u8; 8] =
            std::array::from_fn(|index| cube.corner_permutation[corner_permutation[index]]);
        let new_corner_orientation: [u8; 8] = std::array::from_fn(|index| {
            (cube.corner_orientation[corner_permutation[index]] + corner_rotation[index]) % 3
        });

        let new_edge_permutation: [u8; 12] =
            std::array::from_fn(|index| cube.edge_permutation[edge_permutation[index]]);
        let new_edge_orientation: [u8; 12] = std::array::from_fn(|index| {
            cube.edge_orientation[edge_permutation[index]] ^ edge_rotation[index]
        });

        Cube {
            corner_permutation: new_corner_permutation,
            corner_orientation: new_corner_orientation,
            edge_permutation: new_edge_permutation,
            edge_orientation: new_edge_orientation,
        }
    }
}

impl Move for SimpleMove {
    fn apply(&self, cube: &Cube) -> Cube {
        match self.turns {
            self::Turn::RIGHT => self.rotate1(cube),
            self::Turn::LEFT => self.rotate1(&self.rotate1(&self.rotate1(cube))),
            self::Turn::TWO => self.rotate1(&self.rotate1(cube)),
        }
    }
    fn all_moves() -> [Self; 18] {
        ALL_MOVES
    }
}

impl FromStr for SimpleMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let side: Side = match chars.next() {
            Some('U') => Side::UP,
            Some('D') => Side::DOWN,
            Some('F') => Side::FRONT,
            Some('B') => Side::BACK,
            Some('R') => Side::RIGHT,
            Some('L') => Side::LEFT,
            _ => return Err(()),
        };
        let turns = match chars.next() {
            None => Turn::RIGHT,
            Some('\'') => Turn::LEFT,
            Some('2') => Turn::TWO,
            _ => return Err(()),
        };
        Ok(SimpleMove { side, turns })
    }
}

impl Distribution<SimpleMove> for SimpleMove {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> SimpleMove {
        let side: Side = [
            Side::UP,
            Side::DOWN,
            Side::FRONT,
            Side::BACK,
            Side::RIGHT,
            Side::LEFT,
        ][rng.random_range(0..6)]
        .to_owned();
        let turns: Turn = [Turn::LEFT, Turn::RIGHT, Turn::TWO][rand::random_range(0..3)].to_owned();
        SimpleMove { side, turns }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Side {
    UP = 0,
    DOWN = 1,
    FRONT = 2,
    BACK = 3,
    RIGHT = 4,
    LEFT = 5,
}

#[derive(Debug, Clone)]
pub enum Turn {
    RIGHT = 1,
    LEFT = -1,
    TWO = 2,
}

impl Display for SimpleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let side = match self.side {
            Side::UP => "U",
            Side::DOWN => "D",
            Side::FRONT => "F",
            Side::BACK => "B",
            Side::RIGHT => "R",
            Side::LEFT => "L",
        };
        let turns = match self.turns {
            Turn::RIGHT => "",
            Turn::LEFT => "'",
            Turn::TWO => "2",
        };
        write!(f, "{}{}", side, turns)
    }
}

pub const ALL_MOVES: [SimpleMove; 18] = [
    SimpleMove {
        side: Side::UP,
        turns: Turn::RIGHT,
    },
    SimpleMove {
        side: Side::UP,
        turns: Turn::LEFT,
    },
    SimpleMove {
        side: Side::UP,
        turns: Turn::TWO,
    },
    SimpleMove {
        side: Side::DOWN,
        turns: Turn::RIGHT,
    },
    SimpleMove {
        side: Side::DOWN,
        turns: Turn::LEFT,
    },
    SimpleMove {
        side: Side::DOWN,
        turns: Turn::TWO,
    },
    SimpleMove {
        side: Side::FRONT,
        turns: Turn::RIGHT,
    },
    SimpleMove {
        side: Side::FRONT,
        turns: Turn::LEFT,
    },
    SimpleMove {
        side: Side::FRONT,
        turns: Turn::TWO,
    },
    SimpleMove {
        side: Side::BACK,
        turns: Turn::RIGHT,
    },
    SimpleMove {
        side: Side::BACK,
        turns: Turn::LEFT,
    },
    SimpleMove {
        side: Side::BACK,
        turns: Turn::TWO,
    },
    SimpleMove {
        side: Side::RIGHT,
        turns: Turn::RIGHT,
    },
    SimpleMove {
        side: Side::RIGHT,
        turns: Turn::LEFT,
    },
    SimpleMove {
        side: Side::RIGHT,
        turns: Turn::TWO,
    },
    SimpleMove {
        side: Side::LEFT,
        turns: Turn::RIGHT,
    },
    SimpleMove {
        side: Side::LEFT,
        turns: Turn::LEFT,
    },
    SimpleMove {
        side: Side::LEFT,
        turns: Turn::TWO,
    },
];

//ABCD EFGH IJKL MNOP QRST UVWX
//0123 ____ ____ ____ ____ 4567
static CORNER_PERMUTATION: [[usize; 8]; 6] = [
    [1, 2, 3, 0, 4, 5, 6, 7],
    [0, 1, 2, 3, 7, 4, 5, 6],
    [3, 1, 2, 7, 0, 5, 6, 4],
    [0, 5, 1, 3, 4, 6, 2, 7],
    [4, 0, 2, 3, 5, 1, 6, 7],
    [0, 1, 6, 2, 4, 5, 7, 3],
];

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
static EDGE_PERMUTATION: [[usize; 12]; 6] = [
    [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11],
    [0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10],
    [7, 1, 2, 3, 0, 5, 6, 8, 4, 9, 10, 11],
    [0, 1, 5, 3, 4, 10, 2, 7, 8, 9, 6, 11],
    [0, 4, 2, 3, 9, 1, 6, 7, 8, 5, 10, 11],
    [0, 1, 2, 6, 4, 5, 11, 3, 8, 9, 10, 7],
];

static EDGE_ORIENTATION: [[u8; 12]; 6] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];
