use std::fmt::Display;

use crate::cube::r#move::{Side, Turn};

use super::r#move::Move;

#[derive(Debug)]
pub struct Sequence {
    moves: Vec<Move>,
}
impl Sequence {
    pub fn new() -> Self {
        Self {
            moves: Vec::with_capacity(8),
        }
    }
    pub fn apply(&self, r#move: &Move) -> Self {
        let mut new_moves = self.moves.clone();
        new_moves.push(r#move.clone());
        Self { moves: new_moves }
    }
    pub fn invert(&self) -> Self {
        Sequence {
            moves: self.into_iter().rev().map(|m| m.invert()).collect(),
        }
    }
    pub fn parse(input: String) -> Option<Sequence> {
        Some(Sequence {
            moves: input
                .split(' ')
                .map(|s| Move::parse(s.to_string()))
                .collect::<Option<Vec<Move>>>()?,
        })
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.into_iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>()
                .join(" ")
                .as_str(),
        )
    }
}

impl IntoIterator for &Sequence {
    type Item = Move;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.clone().into_iter()
    }
}

impl Sequence {
    pub fn random_scramble(len: u32) -> Sequence {
        // Sequence::new()
        let mut moves: Vec<Move> = Vec::new();

        for _ in 0..len {
            let side: Side = [
                Side::UP,
                Side::DOWN,
                Side::FRONT,
                Side::BACK,
                Side::RIGHT,
                Side::LEFT,
            ][rand::random_range(0..6)]
            .to_owned();
            let turns: Turn =
                [Turn::LEFT, Turn::RIGHT, Turn::TWO][rand::random_range(0..3)].to_owned();
            moves.push(Move { side, turns })
        }

        Sequence { moves }
    }
}
