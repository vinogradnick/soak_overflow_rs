use crate::data::{
    game_context::GameContext,
    hero::Hero,
    position::{is_between, Position},
};

pub struct CoverQuery;

impl CoverQuery {
    pub fn is_covered_hero_position(ctx: &GameContext, hero: &Hero) -> bool {
        let covers = CoverQuery::all_cover_positions(ctx);

        for cov in &covers {
            if hero.position.eq(cov) {
                return true;
            }
        }

        false
    }

    pub fn all_cover_positions(ctx: &GameContext) -> Vec<Position> {
        let mut items = vec![];
        for item in &ctx.tilemap.tiles {
            if !item.is_cover() || item.is_ocuped() {
                continue;
            }

            for near in ctx.tilemap.neighbors(&item.position) {
                if !Position::is_linear(&item.position, &near) {
                    continue;
                }
                items.push(near);
            }
        }

        return items;
    }

    pub fn evaluate(ctx: &GameContext) -> Vec<(Position, i32)> {
        let mut score = vec![];

        let enemies: Vec<_> = ctx.hero_store.enemies().collect();

        // eprintln!("{:?}", &enemies);

        for item in &ctx.tilemap.tiles {
            if !item.is_cover() || item.is_ocuped() {
                continue;
            }

            for near in ctx.tilemap.neighbors(&item.position) {
                if enemies.iter().any(|x| {
                    x.position.distance(&near) <= 2
                        || is_between(near.x, item.position.x, x.position.x)
                }) {
                    continue;
                }

                if ctx.tilemap.get_tile(&near).is_some_and(|x| x.is_ocuped()) {
                    continue;
                }

                score.push((near, item.tile_type.into()));
            }
        }
        for item in &score {
            // crate::viz::render::debug_position(ctx, &item.0, "#7e1b1bdd", format!("S:{}", item.1));
        }
        return score;
    }
}
