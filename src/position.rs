use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn m_dist(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
    pub fn dir(&self, other: &Position) -> Position {
        Position::new(other.x - self.x, other.y - self.y)
    }
    pub fn direction(lhs: &Position, rhs: &Position) -> Position {
        Position::new(rhs.x - lhs.x, rhs.y - lhs.y)
    }

    pub fn is_linear(lhs: &Position, rhs: &Position) -> bool {
        lhs.x == rhs.x || lhs.y == rhs.y
    }

    pub fn is_valid(&self, size: (usize, usize)) -> bool {
        self.x >= 0 && self.x < size.0 as i32 && self.y >= 0 && self.y < size.1 as i32
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
