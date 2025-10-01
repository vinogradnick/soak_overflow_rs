use crate::{
    data::{game_context::GameContext, hero::Hero},
    infra::logger,
};

pub struct ShooterSystem;

impl ShooterSystem {
    pub fn find_enemy<'a>(ctx: &'a GameContext, hero: &'a Hero) -> Option<&'a Hero> {
        logger::log("", "ShooterSystem:find_enemy");
        ctx.hero_store
            .heroes
            .iter()
            .filter(|e| {
                e.player != hero.player
                    && e.position.distance_8x(&hero.position) < hero.optimal_range
            }) // только враги
            .min_by_key(|e| e.position.distance(&hero.position)) // ближайший
    }
}
