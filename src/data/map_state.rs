use std::fmt::{self, Display};

use crate::{data::position::Position, io::reader::Reader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TileType {
    Empty = 0,
    HighWall = 2,
    LowWall = 1,
}

impl From<TileType> for i32 {
    fn from(value: TileType) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for TileType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TileType::Empty),
            2 => Ok(TileType::HighWall),
            1 => Ok(TileType::LowWall),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TileType::Empty => "0",
            TileType::HighWall => "2",
            TileType::LowWall => "1",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Occupant {
    None,
    OwnerHero(i32), // можно хранить id
    EnemyHero(i32),
}

impl Display for Occupant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Occupant::EnemyHero(v) => write!(f, "EnemyHero({})", v),
            Occupant::OwnerHero(v) => write!(f, "OwnerHero({})", v),
            _ => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub position: Position,
    pub tile_type: TileType,
    pub occupant: Occupant,
}

impl Tile {
    #[inline]
    pub fn is_walkable(&self) -> bool {
        !self.is_cover() && !self.is_occupied()
    }

    pub fn is_occupied(&self) -> bool {
        self.occupant != Occupant::None
    }
    #[inline]
    pub fn is_cover(&self) -> bool {
        return self.tile_type != TileType::Empty;
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tile({}, {}) type={} entity={}",
            self.position.x, self.position.y, self.tile_type, self.occupant
        )
    }
}

#[derive(Debug, Default)]
pub struct MapState {
    pub height: usize,
    pub width: usize,
    pub tiles: Vec<Tile>, // плоский массив
}

impl MapState {
    pub fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Self {
        Self {
            height,
            width,
            tiles,
        }
    }
    pub fn neighbors_range(&self, pos: &Position) -> impl Iterator<Item = &Tile> {
        pos.neighbors_range(self.width, self.height)
            .into_iter()
            .filter_map(move |p| self.get_tile(p.x, p.y))
    }

    pub fn neighbors(&self, pos: &Position) -> impl Iterator<Item = &Tile> {
        pos.neighbors(self.width, self.height)
            .into_iter()
            .filter_map(move |p| self.get_tile(p.x, p.y))
    }

    pub fn get_sizes(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[inline]
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    #[inline]
    pub fn in_bounds_i32(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn is_in_map(&self, pos: &Position) -> bool {
        self.in_bounds(pos.x, pos.y)
    }
    pub fn from_input<R: Reader>(reader: &mut R) -> Self {
        reader.read_map()
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let tile = &self.tiles[idx];

                eprint!("{}", tile.tile_type);
            }
            eprintln!();
        }
    }

    pub fn update_tile(&mut self, x: usize, y: usize, tile_type: TileType, occupant: Occupant) {
        if self.in_bounds(x, y) {
            let index = y * self.width + x;
            self.tiles[index].tile_type = tile_type;
            self.tiles[index].occupant = occupant;
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if self.in_bounds(x, y) {
            let index = y * self.width + x;
            self.tiles.get(index)
        } else {
            None
        }
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if self.in_bounds(x, y) {
            let index = y * self.width + x;
            self.tiles.get_mut(index)
        } else {
            None
        }
    }

    pub fn from_str(s: &str) -> Self {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len();
        let width = lines[0].len();

        let mut tiles = Vec::with_capacity(height * width);

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let val = ch.to_digit(10).unwrap() as i32;
                let tile = TileType::try_from(val).unwrap_or(TileType::Empty);
                tiles.push(Tile {
                    position: Position { x: x, y: y },
                    tile_type: tile,
                    occupant: Occupant::None,
                });
            }
        }

        MapState::new(width, height, tiles)
    }
}
