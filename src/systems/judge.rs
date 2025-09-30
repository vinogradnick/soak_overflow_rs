use crate::{
    data::{game_context::GameContext, hero::HeroCommand},
    simulator::simulator_action,
};

pub struct JudgeService;

impl JudgeService {
    pub fn apply_action_sim(ctx: &GameContext, action: HeroCommand) -> (i32, i32) {
        let mut cloned_context = ctx.clone();

        let res = simulator_action(&mut cloned_context, vec![action]);

        match res {
            Result::Ok(_) => JudgeService::evaluate_context(&cloned_context),
            _ => return (0, 0),
        }
    }

    pub fn evaluate_context(ctx: &GameContext) -> (i32, i32) {
        let mut enemy_score = 0;
        let mut own_score = 0;

        let enemies = ctx.hero_store.enemies().collect::<Vec<_>>();
        let heroes = ctx.hero_store.owns().collect::<Vec<_>>();

        for tile in &ctx.tilemap.tiles {
            let own_dist = heroes
                .iter()
                .map(|hero| hero.position.distance(&tile.position))
                .min()
                .unwrap_or(9999);

            let enemy_dist = enemies
                .iter()
                .map(|hero| hero.position.distance(&tile.position))
                .min()
                .unwrap_or(9999);

            if own_dist < enemy_dist {
                own_score += 1;
            } else if enemy_dist < own_dist {
                enemy_score += 1;
            }
        }
        return (own_score, enemy_score);
    }
}
