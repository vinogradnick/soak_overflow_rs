use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<(usize, usize)> for Position {
    type Output = Position;

    fn add(self, other: (usize, usize)) -> Position {
        Position {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl Position {
    pub const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    pub fn neighbors(&self, width: usize, height: usize) -> Vec<Position> {
        let x = self.x;
        let y = self.y;

        Self::DIRECTIONS
            .iter()
            .filter_map(|(dx, dy)| {
                let nx = if *dx >= 0 {
                    x.checked_add(*dx as usize)
                } else {
                    x.checked_sub((-dx) as usize)
                };

                let ny = if *dy >= 0 {
                    y.checked_add(*dy as usize)
                } else {
                    y.checked_sub((-dy) as usize)
                };

                match (nx, ny) {
                    (Some(nx), Some(ny)) if nx < width && ny < height => {
                        Some(Position { x: nx, y: ny })
                    }
                    _ => None,
                }
            })
            .collect()
    }

    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn m_dist(&self, other: &Position) -> i32 {
        (self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()
    }
    pub fn dir(&self, other: &Position) -> (i32, i32) {
        (
            other.x as i32 - self.x as i32,
            other.y as i32 - self.y as i32,
        )
    }
    pub fn in_radius(&self, other: &Position, radius: usize) -> bool {
        let dx = (self.x as isize - other.x as isize).abs();
        let dy = (self.y as isize - other.y as isize).abs();
        dx as usize <= radius && dy as usize <= radius
    }
    pub fn direction(lhs: &Position, rhs: &Position) -> Position {
        Position::new(rhs.x - lhs.x, rhs.y - lhs.y)
    }

    pub fn is_linear(lhs: &Position, rhs: &Position) -> bool {
        lhs.x == rhs.x || lhs.y == rhs.y
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
