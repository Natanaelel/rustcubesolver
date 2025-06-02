use super::color::Color;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corner(pub Color, pub Color, pub Color);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge(pub Color, pub Color);
impl Edge {
    pub fn flip(self, times: u8) -> Edge {
        let mut new = self;
        for _ in 0..times {
            new = Self::flip1(new);
        }
        new
    }

    fn flip1(edge: Self) -> Self {
        Self(edge.1, edge.0)
    }
}

impl Corner {
    pub fn rotate(self, times: u8) -> Corner {
        let mut new = self;
        for _ in 0..times {
            new = Self::rotate1(new);
        }
        return new;
    }

    fn rotate1(corner: Self) -> Self {
        Self(corner.2, corner.0, corner.1)
    }
}
