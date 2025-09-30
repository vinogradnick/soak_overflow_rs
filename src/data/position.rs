use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub const WAYPOINTS: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    pub const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
impl Default for Position {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl Position {
    pub fn distance_8x(&self, other: &Position) -> i32 {
        let dx = (self.x as i32 - other.x as i32).abs();
        let dy = (self.y as i32 - other.y as i32).abs();
        dx.max(dy)
    }
    pub fn distance(&self, other: &Position) -> i32 {
        let x1 = self.x as i32;
        let y1 = self.y as i32;
        let x2 = other.x as i32;
        let y2 = other.y as i32;
        (x2 - x1).abs() + (y2 - y1).abs()
    }
    pub fn is_linear(lhs: &Position, rhs: &Position) -> bool {
        lhs.x == rhs.x || lhs.y == rhs.y
    }

    pub fn dir(&self, other: &Position) -> (i32, i32) {
        let x1 = self.x as i32;
        let y1 = self.y as i32;
        let x2 = other.x as i32;
        let y2 = other.y as i32;
        ((x2 - x1), (y2 - y1))
    }
}

pub fn is_between<T>(value: T, min: T, max: T) -> bool
where
    T: PartialEq + PartialOrd,
{
    value >= min && value <= max
}
