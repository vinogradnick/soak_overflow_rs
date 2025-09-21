use crate::data::{hero::HeroStore, tilemap::TileMap};

#[derive(Debug, Clone)]
pub struct GameContext {
    pub player_id: i32,
    pub tilemap: TileMap,
    pub hero_store: HeroStore,
}

impl GameContext {
    pub fn new() -> Self {
        Self {
            player_id: 0,
            tilemap: TileMap::new(0, 0),
            hero_store: HeroStore::new(),
        }
    }
}
