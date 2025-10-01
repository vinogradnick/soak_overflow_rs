use crate::{
    core::agg_system::AggSystem,
    data::{game_context::GameContext, tile::TileView},
};

#[derive(Debug, Default)]
pub struct CoverSystem {
    covers: Vec<TileView>,
}

impl CoverSystem {
    pub fn new() -> Self {
        Self { covers: vec![] }
    }

    pub fn process(&mut self, ctx: &GameContext) {}
}
