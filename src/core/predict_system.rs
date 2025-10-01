use std::collections::{HashSet, VecDeque};

use crate::{
    data::{game_context::GameContext, position::Position},
    infra::logger,
};

pub struct PredictSystem {}

impl PredictSystem {
    pub fn process(ctx: &GameContext) {}

    pub fn predict(ctx: &GameContext) -> (i32, i32) {
        let my_heroes: Vec<_> = ctx
            .hero_store
            .heroes
            .iter()
            .filter(|x| x.is_owner)
            .collect();
        let enemies: Vec<_> = ctx
            .hero_store
            .heroes
            .iter()
            .filter(|x| !x.is_owner)
            .collect();

        let mut enemy_score = 0;
        let mut my_score = 0;

        for tile in &ctx.tilemap.tiles {
            // if visited.contains(&tile.position) {
            //     continue;
            // }
            // visited.insert(tile.position);

            let my_min = my_heroes
                .iter()
                .map(|h| h.position.distance(&tile.position))
                .min()
                .unwrap_or(i32::MAX);

            let enemy_min = enemies
                .iter()
                .map(|h| h.position.distance(&tile.position))
                .min()
                .unwrap_or(i32::MAX);

            if my_min == enemy_min {
                continue;
            } else if my_min < enemy_min {
                my_score += 1;
            } else {
                enemy_score += 1;
            }
        }

        logger::log(&(my_score, enemy_score), "PredictSystem:predict");

        return (my_score, enemy_score);
    }
}
