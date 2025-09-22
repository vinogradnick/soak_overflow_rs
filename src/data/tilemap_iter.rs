use crate::data::{position::Position, tile::TileView, tilemap::TileMap};

pub struct Neighbors<'a> {
    map: &'a TileMap, // доступ к карте
    pos: Position,    // центр
    diag: bool,       // 4 или 8 направлений
    idx: usize,       // текущий шаг
}

impl<'a> Neighbors<'a> {
    pub fn new(map: &'a TileMap, pos: Position, diag: bool) -> Self {
        Self {
            map,
            pos,
            diag,
            idx: 0,
        }
    }
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = &'a TileView;

    fn next(&mut self) -> Option<Self::Item> {
        // список смещений для 4 или 8 соседей
        const DIRS_4: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        const DIRS_8: [(i32, i32); 8] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        let dirs: &[(i32, i32)] = if self.diag { &DIRS_8 } else { &DIRS_4 };

        while self.idx < dirs.len() {
            let (dx, dy) = dirs[self.idx];
            self.idx += 1;

            let x = self.pos.x as i32 + dx;
            let y = self.pos.y as i32 + dy;

            if x < 0
                || y < 0
                || x >= self.map.get_width() as i32
                || y >= self.map.get_height() as i32
            {
                continue;
            }

            let id = (y as usize) * self.map.get_width() + (x as usize);
            if let Some(tile) = self.map.find_by_id(id) {
                return Some(tile);
            }
        }
        None
    }
}
pub struct NeighborsRange<'a> {
    map: &'a TileMap,
    center: Position,
    diag: bool,
    range: usize,
    curr_y: isize,
    curr_x: isize,
}

impl<'a> NeighborsRange<'a> {
    pub fn new(map: &'a TileMap, pos: Position, diag: bool, range: usize) -> Self {
        let r = range as isize;
        Self {
            map,
            center: pos,
            diag,
            range,
            curr_y: -r,
            curr_x: -r,
        }
    }
}

impl<'a> Iterator for NeighborsRange<'a> {
    type Item = &'a TileView;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.range as isize;

        while self.curr_y <= r {
            while self.curr_x <= r {
                let dx = self.curr_x;
                let dy = self.curr_y;
                self.curr_x += 1;

                if dx == 0 && dy == 0 {
                    continue; // пропускаем центр
                }
                if !self.diag && dx.abs() + dy.abs() > 1 {
                    continue; // только 4 стороны
                }

                let nx = self.center.x as isize + dx;
                let ny = self.center.y as isize + dy;

                if nx >= 0
                    && ny >= 0
                    && nx < self.map.get_width() as isize
                    && ny < self.map.get_height() as isize
                {
                    let index = (ny as usize) * self.map.get_width() + (nx as usize);
                    return Some(&self.map.tiles[index]);
                }
                // если вне карты → продолжаем цикл
            }
            self.curr_x = -r; // сброс x
            self.curr_y += 1;
        }
        None
    }
}
