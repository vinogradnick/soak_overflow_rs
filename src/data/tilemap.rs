use crate::data::{position::Position, tile::TileView};

#[derive(Debug, Clone)]
pub struct TileMap {
    height: usize,
    width: usize,
    pub tiles: Vec<TileView>,
}

impl TileMap {
    pub fn new(w: usize, h: usize) -> Self {
        TileMap {
            height: h,
            width: w,
            tiles: vec![TileView::default(); w * h],
        }
    }

    pub fn tiles_by_dist<'a>(
        &'a self,
        other: &'a Position,
        dist: i32,
    ) -> impl Iterator<Item = &'a TileView> {
        self.tiles
            .iter()
            .filter(move |tile| tile.position.distance(other) >= dist)
    }

    pub fn find_by_id(&self, id: usize) -> Option<&TileView> {
        self.tiles.get(id)
    }
    pub fn get_tile<'a>(&'a self, position: &Position) -> Option<&'a TileView> {
        self.tiles.get(position.y * self.width + position.x)
    }
    pub fn get_tile_mut(&mut self, position: &Position) -> Option<&mut TileView> {
        self.tiles.get_mut(position.y * self.width + position.x)
    }

    pub fn out_of_bounds(&self, x: impl Into<i32>, y: impl Into<i32>) -> bool {
        let x = x.into();
        let y = y.into();
        x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn neighbors(&self, from: &Position) -> Vec<Position> {
        let mut v = vec![];

        for (x, y) in Position::DIRECTIONS {
            let dx = from.x as i32 + x;
            let dy = from.y as i32 + y;

            if self.out_of_bounds(dx, dy) {
                continue;
            }

            v.push(Position {
                x: dx as usize,
                y: dy as usize,
            });
        }
        v
    }
    pub fn neighbors_8x(&self, from: &Position) -> Vec<Position> {
        let mut v = vec![];

        for (x, y) in Position::WAYPOINTS {
            let dx = from.x as i32 + x;
            let dy = from.y as i32 + y;

            if self.out_of_bounds(dx, dy) {
                continue;
            }

            v.push(Position {
                x: dx as usize,
                y: dy as usize,
            });
        }
        v
    }
}
