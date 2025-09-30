use crate::data::position::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    HighWall = 0,
    LowWall = 1,
    Empty = 2,
}

impl From<TileType> for i32 {
    fn from(value: TileType) -> Self {
        match value {
            TileType::HighWall => 2,
            TileType::LowWall => 1,
            TileType::Empty => 0,
        }
    }
}

impl From<TileType> for f32 {
    fn from(value: TileType) -> Self {
        match value {
            TileType::HighWall => 0.75,
            TileType::LowWall => 0.5,
            TileType::Empty => 0.0,
        }
    }
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Empty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Occupant {
    Enemy(usize),
    Owner(usize),
    Nil,
}

impl Default for Occupant {
    fn default() -> Self {
        Self::Nil
    }
}

pub struct TileMeta {}

#[derive(Debug, Clone, Copy, Default)]
pub struct TileView {
    pub position: Position,
    pub occupant: Occupant,
    pub tile_type: TileType,
}

impl TileView {
    #[inline]
    pub fn is_cover(&self) -> bool {
        self.tile_type != TileType::Empty
    }
    #[inline]
    pub fn is_free(&self) -> bool {
        self.occupant == Occupant::Nil && self.tile_type == TileType::Empty
    }
    #[inline]
    pub fn is_ocuped(&self) -> bool {
        self.occupant != Occupant::Nil
    }
    pub fn is_enemy_occuped(&self) -> bool {
        match self.occupant {
            Occupant::Enemy(_) => true,
            _ => false,
        }
    }
    pub fn is_own_occuped(&self) -> bool {
        match self.occupant {
            Occupant::Owner(_) => true,
            _ => false,
        }
    }
}
