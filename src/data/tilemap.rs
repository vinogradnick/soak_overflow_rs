use crate::data::position::Position;

#[derive(Debug, Clone)]
pub struct TileMap {
    height: usize,
    width: usize,
    tiles: Vec<i32>,
    enemies: Vec<i32>,
}

pub struct TileMapIter<'a> {
    pub map: &'a TileMap,
    id: usize,
}

pub type TileView = (i32, i32, Position);

impl TileMap {
    pub fn new(w: usize, h: usize) -> Self {
        TileMap {
            height: h,
            width: w,
            tiles: vec![0; w * h],
            enemies: vec![-1; w * h],
        }
    }
    #[inline(always)]
    pub fn get_index_raw(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }
    #[inline(always)]
    pub fn get_index(&self, pos: impl Into<Position>) -> usize {
        let pos = pos.into();
        self.get_index_raw((pos.x, pos.y))
    }
    pub fn from_index(&self, idx: usize) -> (usize, usize) {
        let y = idx / self.width; // целочисленное деление
        let x = idx % self.width; // остаток от деления
        (x, y)
    }

    pub fn enumerate(&self) -> impl Iterator<Item = usize> {
        0..(self.height * self.width)
    }

    pub fn get_view(&self, pos: impl Into<Position>) -> TileView {
        let id = self.get_index(pos);
        let tile = self.tiles[id];
        let enemy = self.enemies[id];
        let to_index = self.from_index(id);

        (tile, enemy, to_index.into())
    }

    pub fn iter(&self) -> TileMapIter<'_> {
        TileMapIter { map: self, id: 0 }
    }

    pub fn set_tile_enemy(&mut self, id: usize, enemy: i32) {
        self.enemies[id] = enemy;
    }
    pub fn set_tile(&mut self, id: usize, tile: &[i32]) {
        self.tiles[id] = tile[0];

        if tile.len() > 1 {
            self.enemies[id] = tile[1];
        }
    }

    pub fn neighbors(&self, pos: &Position) -> Vec<Position> {
        pos.neighbors(self.width, self.height)
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
}

impl<'a> Iterator for TileMapIter<'a> {
    type Item = TileView;

    fn next(&mut self) -> Option<Self::Item> {
        if self.id >= self.map.height * self.map.width {
            return None;
        }
        let tile = self.map.tiles[self.id];
        let enemy = self.map.enemies[self.id];

        let (x, y) = self.map.from_index(self.id);

        self.id += 1;
        Some((tile, enemy, Position { x, y }))
    }
}
