use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Move {
    pub side: Side,
    pub turns: Turn,
}
impl Move {
    pub fn invert(&self) -> Move {
        Move {
            side: self.side.clone(),
            turns: match self.turns {
                Turn::RIGHT => Turn::LEFT,
                Turn::LEFT => Turn::RIGHT,
                Turn::TWO => Turn::TWO,
            },
        }
    }
    pub fn parse(input: String) -> Option<Move> {
        let mut chars = input.chars();
        let side: Side = match chars.next() {
            Some('U') => Side::UP,
            Some('D') => Side::DOWN,
            Some('F') => Side::FRONT,
            Some('B') => Side::BACK,
            Some('R') => Side::RIGHT,
            Some('L') => Side::LEFT,
            _ => return None,
        };
        let turns = match chars.next() {
            None => Turn::RIGHT,
            Some('\'') => Turn::LEFT,
            Some('2') => Turn::TWO,
            _ => return None,
        };

        Some(Move { side, turns })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Side {
    UP,
    DOWN,
    FRONT,
    BACK,
    RIGHT,
    LEFT,
}

#[derive(Debug, Clone)]
pub enum Turn {
    RIGHT = 1,
    LEFT = -1,
    TWO = 2,
}

impl Display for Move {
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

pub const ALL_MOVES: [Move; 18] = [
    Move {
        side: Side::UP,
        turns: Turn::RIGHT,
    },
    Move {
        side: Side::UP,
        turns: Turn::LEFT,
    },
    Move {
        side: Side::UP,
        turns: Turn::TWO,
    },
    Move {
        side: Side::DOWN,
        turns: Turn::RIGHT,
    },
    Move {
        side: Side::DOWN,
        turns: Turn::LEFT,
    },
    Move {
        side: Side::DOWN,
        turns: Turn::TWO,
    },
    Move {
        side: Side::FRONT,
        turns: Turn::RIGHT,
    },
    Move {
        side: Side::FRONT,
        turns: Turn::LEFT,
    },
    Move {
        side: Side::FRONT,
        turns: Turn::TWO,
    },
    Move {
        side: Side::BACK,
        turns: Turn::RIGHT,
    },
    Move {
        side: Side::BACK,
        turns: Turn::LEFT,
    },
    Move {
        side: Side::BACK,
        turns: Turn::TWO,
    },
    Move {
        side: Side::RIGHT,
        turns: Turn::RIGHT,
    },
    Move {
        side: Side::RIGHT,
        turns: Turn::LEFT,
    },
    Move {
        side: Side::RIGHT,
        turns: Turn::TWO,
    },
    Move {
        side: Side::LEFT,
        turns: Turn::RIGHT,
    },
    Move {
        side: Side::LEFT,
        turns: Turn::LEFT,
    },
    Move {
        side: Side::LEFT,
        turns: Turn::TWO,
    },
];
