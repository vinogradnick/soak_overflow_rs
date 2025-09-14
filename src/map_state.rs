use std::fmt::{self, Display};

use crate::{position::Position, reader::Reader};

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub position: Position,
    pub tile_type: i32,
    pub entity_id: i32,
}

impl Tile {
    pub fn get_cover_int(&self) -> i32 {
        if self.tile_type != 0 {
            self.tile_type + 1
        } else {
            1
        }
    }
    pub fn get_cover_value(&self) -> f32 {
        match self.tile_type {
            1 => 0.5,
            2 => 0.7,
            _ => 0.0,
        }
    }
    pub fn is_free(&self) -> bool {
        self.tile_type == 0
    }
    pub fn is_occupied(&self) -> bool {
        self.entity_id != -1
    }
    #[inline]
    pub fn is_cover(&self) -> bool {
        return self.tile_type == 1 || self.tile_type == 2;
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tile({}, {}) type={} entity={}",
            self.position.x, self.position.y, self.tile_type, self.entity_id
        )
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TileScore {
    pub danger: i32,
    pub safety: i32,
    pub position: i32,
}

#[derive(Debug)]
pub struct MapState {
    pub height: usize,
    pub width: usize,
    pub tiles: Vec<Tile>, // плоский массив
    pub scoring: Vec<TileScore>,
}

impl MapState {
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
    fn find_nearest_tile(&self, position: &Position, t_type: i32) -> Option<&Tile> {
        self.tiles
            .iter()
            .filter(|h| h.tile_type == t_type)
            .min_by_key(|h| h.position.dist(position)) // выбираем минимальное расстояние
    }

    #[inline]
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
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

    pub fn update_tile(&mut self, x: usize, y: usize, tile_type: i32, entity_id: i32) {
        if self.in_bounds(x, y) {
            let index = y * self.width + x;
            self.tiles[index].tile_type = tile_type;
            self.tiles[index].entity_id = entity_id;
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
                tiles.push(Tile {
                    position: Position { x: x, y: y },
                    tile_type: val,
                    entity_id: -1,
                });
            }
        }

        Self {
            height,
            width,
            tiles,
            scoring: vec![],
        }
    }
}
