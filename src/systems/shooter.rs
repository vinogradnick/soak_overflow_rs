use crate::data::{game_context::GameContext, hero::Hero, position::Position, tile::TileType};

pub struct ShooterQuery;

impl ShooterQuery {
    pub fn get_target<'a>(ctx: &'a GameContext, hero: &'a Hero) -> Option<&'a Hero> {
        return ctx
            .hero_store
            .enemies()
            .find(|enemy| enemy.position.distance(&hero.position) < hero.optimal_range);
    }
}
