use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    WHITE,
    GREEN,
    RED,
    BLUE,
    ORANGE,
    YELLOW,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Color::WHITE => 'W',
            Color::GREEN => 'G',
            Color::RED => 'R',
            Color::BLUE => 'B',
            Color::ORANGE => 'O',
            Color::YELLOW => 'Y',
        };
        let color = match self {
            Color::WHITE => "\x1b[0;37m",
            Color::GREEN => "\x1b[0;32m",
            Color::RED => "\x1b[0;31m",
            Color::BLUE => "\x1b[0;34m",
            Color::ORANGE => "\x1b[38;5;202m",
            Color::YELLOW => "\x1b[0;33m",
        };
        let reset = "\x1b[0m";
        write!(f, "{}{}{}", color, char, reset)
    }
}