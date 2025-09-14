use crate::{context::GameContext, hero::hero_entity::HeroEntity, position::Position};

pub fn find_cover_tile<'a>(ctx: &'a GameContext<'a>, hero: &'a HeroEntity) -> Option<Position> {
    let mut nearby_covers: Vec<_> = ctx
        .map_state
        .tiles
        .iter()
        .filter_map(|tile| {
            let dist = tile.position.dist(&hero.position);
            (tile.is_cover()).then_some((dist, tile))
        })
        .collect();

    nearby_covers.sort_by_key(|(dist, tile)| *dist / tile.get_cover_int());

    for (_, tile) in &nearby_covers {
        let mut dx = 0;
        let mut dy = 0;
        for (enemy, _) in ctx.hero_service.nearby_enemies(hero, 5) {
            let (_dx, _dy) = tile.position.dir(&enemy.position);
            dx += _dx;
            dy += _dy;
        }
        let mut position_clone = tile.position.clone();
        if dx > 0 {
            position_clone.x -= 1;
        } else {
            position_clone.x += 1;
        }
        return Some(position_clone);
    }

    None
}

pub fn is_covered_hero(ctx: &GameContext, hero_position: &Position) -> bool {
    ctx.map_state
        .neighbors(hero_position)
        .any(|tile| tile.is_cover())
}

pub fn get_hero_cover_quality<'a>(ctx: &'a GameContext<'a>, hero_position: &'a Position) -> i32 {
    let mut value = 0;
    for tile in ctx.map_state.neighbors(hero_position) {
        if !tile.is_cover() && !Position::is_linear(&tile.position, hero_position) {
            continue;
        }
        value += tile.get_cover_int();
    }
    return value;
}

pub fn is_hero_icopued(ctx: &GameContext, hero_position: &Position) -> bool {
    let mut value = 0;
    for tile in ctx.map_state.neighbors(hero_position) {
        if tile.tile_type != 3 {
            continue;
        }
        value += tile.get_cover_int();
    }
    value > 0
}
