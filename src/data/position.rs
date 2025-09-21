use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Position> for (usize, usize) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}

impl Position {
    pub const WAYPOINTS: [(i32, i32); 8] = Position::generate_directions();

    const fn generate_directions() -> [(i32, i32); 8] {
        let mut dirs = [(0, 0); 8];
        let mut i = 0;
        let mut dx = -1;
        while dx <= 1 {
            let mut dy = -1;
            while dy <= 1 {
                if dx != 0 || dy != 0 {
                    dirs[i] = (dx, dy);
                    i += 1;
                }
                dy += 1;
            }
            dx += 1;
        }
        dirs
    }

    pub const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    pub fn neighbors_range(&self, width: usize, height: usize, range: usize) -> Vec<Position> {
        let mut result = Vec::new();
        let x = self.x as isize;
        let y = self.y as isize;
        let r = range as isize;

        for dy in -r..=r {
            for dx in -r..=r {
                if dx == 0 && dy == 0 {
                    continue; // пропускаем саму точку
                }

                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                    result.push(Position {
                        x: nx as usize,
                        y: ny as usize,
                    });
                }
            }
        }

        result
    }
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

    pub fn new_tuple((x, y): (i32, i32)) -> Self {
        Self {
            x: x as usize,
            y: y as usize,
        }
    }

    /// дистанция используемая для бомбы потому что диагонали утываются
    pub fn multi_distance(&self, other: &Position) -> usize {
        (self.x as isize - other.x as isize)
            .abs()
            .max((self.y as isize - other.y as isize).abs()) as usize
    }

    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn dist_raw(&self, x: usize, y: usize) -> i32 {
        (self.x as i32 - x as i32).abs() + (self.y as i32 - y as i32).abs()
    }

    pub fn dist(&self, other: &Position) -> i32 {
        (self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()
    }

    /// `include_diagonal = true` — проверять 8 направлений, иначе только 4
    pub fn is_neighbor(&self, other: &Position, include_diagonal: bool) -> bool {
        let dx = (self.x as isize - other.x as isize).abs();
        let dy = (self.y as isize - other.y as isize).abs();

        if dx == 0 && dy == 0 {
            return false; // сама клетка не считается
        }

        if include_diagonal {
            dx <= 1 && dy <= 1
        } else {
            dx + dy == 1
        }
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

pub struct PositionUtil;

impl PositionUtil {
    pub fn multi_distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
        (x1 as isize - x2 as isize)
            .abs()
            .max((y1 as isize - y2 as isize).abs()) as usize
    }
    pub fn dist_raw((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> i32 {
        (x2 as i32 - x1 as i32).abs() + (y2 as i32 - y1 as i32).abs()
    }
}
