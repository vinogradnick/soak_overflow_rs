use crate::{
    data::context::GameContext, data::position::Position, hero::hero_entity::HeroEntity, utils,
};

const RADIUS: i32 = 2; // радиус вокруг тайла для подсчёта врагов

fn find_enemy_cluster(ctx: &GameContext) -> Option<Position> {
    let mut best_tile = None;
    let mut max_count = 0;

    for y in 0..ctx.map_state.height as i32 {
        for x in 0..ctx.map_state.width as i32 {
            let tile_pos = Position::new_tuple((x, y));

            // подсчёт врагов в радиусе RADIUS
            let count = ctx
                .hero_service
                .enemy_list()
                .filter(|en| {
                    let dist = en.position.dist(&tile_pos);
                    dist > 0 && dist <= RADIUS
                })
                .count();

            if count > max_count {
                max_count = count;
                best_tile = Some(tile_pos);
            }
        }
    }

    best_tile
}

pub fn find_bombing_position<'a>(
    ctx: &'a GameContext,
    hero: &'a HeroEntity,
) -> Option<(Position, Position)> {
    for tile in &ctx.map_state.tiles {
        if tile.is_cover()
            || tile.is_occupied()
            || !utils::pathfinder::can_reach(ctx, &hero.position, &tile.position)
        {
            continue;
        }

        if !ctx.hero_service.enemy_list().any(|en| {
            let p = en.position.dist(&tile.position);
            return p > 0 && p < 3;
        }) {
            continue;
        }

        let p = find_enemy_cluster(ctx);

        if let Some(t) = p {
            if t.dist(&tile.position) <= 3 {
                crate::viz::render::debug_position(
                    ctx,
                    &tile.position,
                    "#9722b4ff",
                    format!("H:{}", hero.agent_id),
                );

                crate::viz::render::debug_position(
                    ctx,
                    &t,
                    "#9722b4ff",
                    format!("H:{}", hero.agent_id),
                );
                return Some((tile.position, t));
            }
        }
    }
    None
}

pub fn find_save_bombing_position<'a>(ctx: &'a GameContext, hero: &'a HeroEntity) {}
