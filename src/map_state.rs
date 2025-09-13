use crate::{position::Position, reader::Reader};

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub position: Position,
    pub tile_type: i32,
    pub entity_id: i32,
}

impl Tile {
    pub fn get_cover_value(&self) -> f32 {
        match self.tile_type {
            1 => 0.5,
            2 => 0.7,
            _ => 0.0,
        }
    }
    pub fn is_occupied(&self) -> bool {
        self.entity_id != -1
    }
}

#[derive(Debug)]
pub struct MapState {
    pub height: usize,
    pub width: usize,
    pub tiles: Vec<Tile>, // плоский массив
}

impl MapState {
    pub fn near_tile_pos(&self, position: &Position, tile_type: i32) -> Option<&Tile> {
        self.tiles
            .iter()
            .filter(|&x| x.tile_type == tile_type && !x.is_occupied())
            .min_by_key(|x| x.position.m_dist(&position))
    }

    pub fn get_sizes(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    fn find_nearest_tile(&self, position: &Position, t_type: i32) -> Option<&Tile> {
        self.tiles
            .iter()
            .filter(|h| h.tile_type == t_type)
            .min_by_key(|h| h.position.m_dist(position)) // выбираем минимальное расстояние
    }
    #[inline]
    pub fn to_index(width: usize, x: usize, y: usize) -> usize {
        y * width + x
    }

    #[inline]
    pub fn in_bounds_pos(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < self.width && (pos.y as usize) < self.height
    }

    pub fn is_in_map(pos: &Position, sizes: (usize, usize)) -> bool {
        pos.x < 0 && pos.x < sizes.0 as i32 && pos.y < 0 && pos.y < sizes.1 as i32
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
                    position: Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    tile_type: val,
                    entity_id: -1,
                });
            }
        }

        Self {
            height,
            width,
            tiles,
        }
    }
}
