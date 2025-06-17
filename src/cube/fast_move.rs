use std::{fmt::Display, str::FromStr, sync::LazyLock};

use crate::cube::{
    Cube,
    r#move::Move,
    simple_move::{Side, Turn},
};

#[derive(Debug, Clone)]
pub struct FastMove {
    pub side: Side,
    pub turns: Turn,
}


impl FastMove {
    fn rotate(&self, cube: &Cube) -> Cube {
        let corner_permutation = match self.turns {
            Turn::RIGHT => CORNER_PERMUTATION_RIGHT,
            Turn::LEFT => CORNER_PERMUTATION_LEFT,
            Turn::TWO => CORNER_PERMUTATION_TWO,
        }[self.side as usize];
        let corner_rotation = match self.turns {
            Turn::RIGHT => CORNER_ORIENTATION_ONE,
            Turn::LEFT => CORNER_ORIENTATION_ONE,
            Turn::TWO => CORNER_ORIENTATION_ZERO,
        }[self.side as usize];

        let edge_permutation = match self.turns {
            Turn::RIGHT => EDGE_PERMUTATION_RIGHT,
            Turn::LEFT => EDGE_PERMUTATION_LEFT,
            Turn::TWO => EDGE_PERMUTATION_TWO,
        }[self.side as usize];
        let edge_rotation = match self.turns {
            Turn::RIGHT => EDGE_ORIENTATION_ONE,
            Turn::LEFT => EDGE_ORIENTATION_ONE,
            Turn::TWO => EDGE_ORIENTATION_ZERO,
        }[self.side as usize];

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

impl Move for FastMove {
    fn apply(&self, cube: &Cube) -> Cube {
        self.rotate(cube)
    }

    fn all_moves() -> [Self; 18] {
        ALL_MOVES
    }
}

impl FromStr for FastMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = ALL_MOVES_STR.iter().position(|m| s == *m).ok_or(())?;
        Ok(ALL_MOVES[index].clone())
    }
}

impl Display for FastMove {
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

const ALL_MOVES_STR: [&'static str; 18] = [
    "U", "U'", "U2", "D", "D'", "D2", "F", "F'", "F2", "B", "B'", "B2", "R", "R'", "R2", "L", "L'",
    "L2",
];
pub const ALL_MOVES: [FastMove; 18] = [
    FastMove {
        side: Side::UP,
        turns: Turn::RIGHT,
    },
    FastMove {
        side: Side::UP,
        turns: Turn::LEFT,
    },
    FastMove {
        side: Side::UP,
        turns: Turn::TWO,
    },
    FastMove {
        side: Side::DOWN,
        turns: Turn::RIGHT,
    },
    FastMove {
        side: Side::DOWN,
        turns: Turn::LEFT,
    },
    FastMove {
        side: Side::DOWN,
        turns: Turn::TWO,
    },
    FastMove {
        side: Side::FRONT,
        turns: Turn::RIGHT,
    },
    FastMove {
        side: Side::FRONT,
        turns: Turn::LEFT,
    },
    FastMove {
        side: Side::FRONT,
        turns: Turn::TWO,
    },
    FastMove {
        side: Side::BACK,
        turns: Turn::RIGHT,
    },
    FastMove {
        side: Side::BACK,
        turns: Turn::LEFT,
    },
    FastMove {
        side: Side::BACK,
        turns: Turn::TWO,
    },
    FastMove {
        side: Side::RIGHT,
        turns: Turn::RIGHT,
    },
    FastMove {
        side: Side::RIGHT,
        turns: Turn::LEFT,
    },
    FastMove {
        side: Side::RIGHT,
        turns: Turn::TWO,
    },
    FastMove {
        side: Side::LEFT,
        turns: Turn::RIGHT,
    },
    FastMove {
        side: Side::LEFT,
        turns: Turn::LEFT,
    },
    FastMove {
        side: Side::LEFT,
        turns: Turn::TWO,
    },
];

//ABCD EFGH IJKL MNOP QRST UVWX
//0123 ____ ____ ____ ____ 4567
static CORNER_PERMUTATION_RIGHT: [[usize; 8]; 6] = [
    [1, 2, 3, 0, 4, 5, 6, 7],
    [0, 1, 2, 3, 7, 4, 5, 6],
    [3, 1, 2, 7, 0, 5, 6, 4],
    [0, 5, 1, 3, 4, 6, 2, 7],
    [4, 0, 2, 3, 5, 1, 6, 7],
    [0, 1, 6, 2, 4, 5, 7, 3],
];

static CORNER_PERMUTATION_TWO: [[usize; 8]; 6] = [
    [2, 3, 0, 1, 4, 5, 6, 7],
    [0, 1, 2, 3, 6, 7, 4, 5],
    [7, 1, 2, 4, 3, 5, 6, 0],
    [0, 6, 5, 3, 4, 2, 1, 7],
    [5, 4, 2, 3, 1, 0, 6, 7],
    [0, 1, 7, 6, 4, 5, 3, 2],
];

static CORNER_PERMUTATION_LEFT: [[usize; 8]; 6] = [
    [3, 0, 1, 2, 4, 5, 6, 7],
    [0, 1, 2, 3, 5, 6, 7, 4],
    [4, 1, 2, 0, 7, 5, 6, 3],
    [0, 2, 6, 3, 4, 1, 5, 7],
    [1, 5, 2, 3, 0, 4, 6, 7],
    [0, 1, 3, 7, 4, 5, 2, 6],
];

static CORNER_ORIENTATION_ONE: [[u8; 8]; 6] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 2, 2, 0, 0, 1],
    [0, 2, 1, 0, 0, 1, 2, 0],
    [2, 1, 0, 0, 1, 2, 0, 0],
    [0, 0, 2, 1, 0, 0, 1, 2],
];

static CORNER_ORIENTATION_ZERO: [[u8; 8]; 6] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

//ABCD EFGH IJKL MNOP QRST UVWX
//0123 _5_8 ____ _7_6 ____ 9012
static EDGE_PERMUTATION_RIGHT: [[usize; 12]; 6] = [
    [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11],
    [0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10],
    [7, 1, 2, 3, 0, 5, 6, 8, 4, 9, 10, 11],
    [0, 1, 5, 3, 4, 10, 2, 7, 8, 9, 6, 11],
    [0, 4, 2, 3, 9, 1, 6, 7, 8, 5, 10, 11],
    [0, 1, 2, 6, 4, 5, 11, 3, 8, 9, 10, 7],
];

static EDGE_PERMUTATION_TWO: [[usize; 12]; 6] = [
    [2, 3, 0, 1, 4, 5, 6, 7, 8, 9, 10, 11],
    [0, 1, 2, 3, 4, 5, 6, 7, 10, 11, 8, 9],
    [8, 1, 2, 3, 7, 5, 6, 4, 0, 9, 10, 11],
    [0, 1, 10, 3, 4, 6, 5, 7, 8, 9, 2, 11],
    [0, 9, 2, 3, 5, 4, 6, 7, 8, 1, 10, 11],
    [0, 1, 2, 11, 4, 5, 7, 6, 8, 9, 10, 3],
];

static EDGE_PERMUTATION_LEFT: [[usize; 12]; 6] = [
    [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
    [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
    [4, 1, 2, 3, 8, 5, 6, 0, 7, 9, 10, 11],
    [0, 1, 6, 3, 4, 2, 10, 7, 8, 9, 5, 11],
    [0, 5, 2, 3, 1, 9, 6, 7, 8, 4, 10, 11],
    [0, 1, 2, 7, 4, 5, 3, 11, 8, 9, 10, 6],
];

static EDGE_ORIENTATION_ONE: [[u8; 12]; 6] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

static EDGE_ORIENTATION_ZERO: [[u8; 12]; 6] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];
