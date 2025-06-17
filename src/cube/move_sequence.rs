use std::{fmt::Display, str::FromStr};

use rand::distr::{Distribution, StandardUniform};

use crate::cube::r#move::Move;

#[derive(Debug)]
pub struct Sequence<T: Move> {
    moves: Vec<T>,
}
impl<T: Move> Sequence<T> {
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
        }
    }
    pub fn apply(&self, r#move: &T) -> Self {
        let mut new_moves = self.moves.clone();
        new_moves.push(r#move.clone());
        Self { moves: new_moves }
    }
    // pub fn invert(&self) -> Self {
    //     Sequence {
    //         moves: self.into_iter().rev().map(|m| m.invert()).collect(),
    //     }
    // }
}

impl<T: Move + FromStr> FromStr for Sequence<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Sequence {
            moves: input
                .split(' ')
                .map(|s| s.parse::<T>())
                .collect::<Result<Vec<T>, Self::Err>>()?,
        })
    }
}

impl<T: Move + Display> Display for Sequence<T> {
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

impl<T: Move> IntoIterator for &Sequence<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.clone().into_iter()
    }
}

impl<T: Move> Sequence<T> {
    pub fn random_scramble(len: u32) -> Sequence<T>
    where
        StandardUniform: Distribution<T>,
    {
        let mut moves: Vec<T> = Vec::new();

        for _ in 0..len {
            moves.push(rand::random())
        }

        Sequence { moves }
    }
}
