use crate::{
    data::{context::GameContext, position::Position},
    hero::hero_entity::HeroEntity,
    systems::scoring::is_surrounded,
    utils::{self, pathfinder},
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
    for tile in ctx.map_state.neighbors(&hero.position) {
        if tile.is_occupied() {
            return find_save_bombing_position(ctx, hero);
        }
    }

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

pub fn bomb_evaluate<'a>(ctx: &'a GameContext, pos: &Position, hero_position: &Position) -> i32 {
    let mut count = 0;

    for enemy in ctx.hero_service.enemy_list() {
        if enemy.position.multi_distance(pos) < 2 {
            count += 1;
        }
    }
    return count;
}

pub fn find_save_bombing_position<'a>(
    ctx: &'a GameContext,
    hero: &'a HeroEntity,
) -> Option<(Position, Position)> {
    // debug_position(ctx, &hero.position, "#1dac7cff", format!("|EV {}", 1));

    let prepared = ctx
        .map_state
        .tiles
        .iter()
        .filter(|t| {
            let pdist = t.position.multi_distance(&hero.position);

            !t.is_cover() && pdist > 1 && pdist < 3
        })
        .collect::<Vec<_>>();

    let maxed = prepared
        .iter()
        .max_by_key(|t| bomb_evaluate(ctx, &t.position, &hero.position));

    if let Some(v) = maxed {
        return Some((hero.position.clone(), v.position.clone()));
        // debug_position(ctx, &v.position, "#12e47bff", format!("|S {}", 1));
    }

    None
}
