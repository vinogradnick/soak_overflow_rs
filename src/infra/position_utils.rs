use crate::data::{game_context::GameContext, hero::Hero, position::Position};

pub fn is_cover_from_any_enemy(
    position: &Position,
    cover: &Position,
    enemies: &[Position],
) -> bool {
    enemies.iter().any(|enemy| {
        // Горизонтальная линия
        if position.y == enemy.y && cover.y == position.y {
            let min_x = position.x.min(enemy.x);
            let max_x = position.x.max(enemy.x);
            return cover.x >= min_x && cover.x <= max_x;
        }
        // Вертикальная линия
        if position.x == enemy.x && cover.x == position.x {
            let min_y = position.y.min(enemy.y);
            let max_y = position.y.max(enemy.y);
            return cover.y >= min_y && cover.y <= max_y;
        }

        false
    })
}

pub fn hero_is_protected<'a>(ctx: &'a GameContext, hero: &'a Hero) -> bool {
    // берем враговы
    let enemies: Vec<_> = ctx
        .hero_store
        .heroes
        .iter()
        .filter(|x| x.player != hero.agent_id)
        .map(|p| p.position)
        .collect();

    // проходим по всем тайлам
    for cover_tile in ctx.tilemap.neighbors(&hero.position) {
        //берем соседние с позицией героя
        let tile_data = ctx.tilemap.get_tile(&cover_tile);

        if let Some(t) = tile_data {
            if t.is_cover() && is_cover_from_any_enemy(&hero.position, &t.position, &enemies) {
                return true;
            }
        }
    }
    return false;
}

pub fn find_cover_position<'a>(ctx: &'a GameContext, enemy_target: i32) -> Vec<Position> {
    let mut all_covers = vec![];

    let enemies: Vec<_> = ctx
        .hero_store
        .heroes
        .iter()
        .filter(|x| x.player == enemy_target)
        .map(|p| p.position)
        .collect();

    for cover_tile in ctx.tilemap.tiles.iter().filter(|x| x.is_cover()) {
        let nearest_tiles = ctx.tilemap.neighbors(&cover_tile.position);

        for near_position in nearest_tiles {
            let near_tile = ctx.tilemap.get_tile(&near_position);

            if let Some(near_unwrapped) = near_tile {
                if !near_unwrapped.is_free() {
                    continue;
                }

                if is_cover_from_any_enemy(&near_position, &cover_tile.position, &enemies) {
                    all_covers.push(near_position);
                }
            }
        }
    }

    all_covers.iter().for_each(|d| {
        // crate::viz::simple::viz_simple_debug("CIRCLE", &d, "#f5f5f5", None);
        crate::viz::render::debug_position(ctx, d, "#f5f5f5", "R2d2");
    });

    return all_covers;
}
