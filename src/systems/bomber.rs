use crate::{
    data::{game_context::GameContext, hero::Hero, position::Position},
    systems::pathfinder,
};

pub fn find_bomb_target(ctx: &GameContext, hero: &Hero) -> Option<Position> {
    let mut candidates = vec![];

    for tile in &ctx.tilemap.tiles {
        if tile.is_cover() {
            continue;
        }

        let mut score = 0;
        for near in ctx.tilemap.neighbors_8x(&tile.position) {
            if let Some(t) = ctx.tilemap.get_tile(&near) {
                match t.occupant {
                    crate::data::tile::Occupant::Enemy(_) => {
                        score += 1;
                    }
                    crate::data::tile::Occupant::Owner(_) => {
                        score -= 1000;
                    }
                    crate::data::tile::Occupant::Nil => {}
                }
            }
        }

        // for item in ctx.hero_store.owns() {
        //     if tile.position.distance_8x(&item.position) <= 1 {
        //         score -= 10;
        //     }
        // }

        candidates.push((score, tile));
    }

    match candidates.iter().max_by_key(|(score, _)| score) {
        Some((score, tile)) => {
            // crate::viz::render::debug_position(ctx, &tile.position, "#fbbbcc", format!("{}", score));

            Some(tile.position)
        }
        None => None,
    }
}

pub fn find_bomb_source(ctx: &GameContext, hero: &Hero, target: &Position) -> Option<Position> {
    for tile in &ctx.tilemap.tiles {
        if tile.is_cover() || tile.is_ocuped() {
            continue;
        }

        let distance = tile.position.distance(target);

        if !pathfinder::can_reach(ctx, &hero.position, &tile.position) {
            // eprintln!("{:?}", (hero.position, tile.position));
            continue;
        }

        // eprintln!("Distance:{} Tile:{:?} Target:{:?}", distance, tile, target);

        if distance < 1 || distance >= 4 {
            continue;
        }
        return Some(tile.position);
    }
    None
}

pub fn count_adjacent_units(ctx: &GameContext, hero: &Hero) -> usize {
    let mut counter = 0;

    for tile in &ctx.tilemap.tiles {
        let distance = tile.position.distance_8x(&hero.position);

        if distance != 1 || !tile.is_enemy_occuped() {
            continue;
        }
        counter += 1;
        // debug_position(ctx, &tile.position, "#2eb18aff", format!("S"));
    }
    return counter;
}

pub fn occupantion_bombing(ctx: &GameContext, hero: &Hero) -> Option<[Position; 2]> {
    let mut candidates = vec![];

    for tile in &ctx.tilemap.tiles {
        if tile.is_cover()
            || tile.position.distance(&hero.position) > 2
            || tile.position == hero.position
        {
            continue;
        }

        let mut score = 0;
        for near in ctx.tilemap.neighbors_8x(&tile.position) {
            if let Some(t) = ctx.tilemap.get_tile(&near) {
                match t.occupant {
                    crate::data::tile::Occupant::Enemy(_) => {
                        score += 1;
                    }
                    crate::data::tile::Occupant::Owner(_) => {
                        score -= 1000;
                    }
                    crate::data::tile::Occupant::Nil => {}
                }
            }
        }

        candidates.push((score, tile));
    }

    let item = candidates.iter().max_by_key(|(score, _)| score);

    match item {
        Some((score, tile_view)) => {
            // debug_position(ctx, &tile_view.position, "#e9c110ff", format!("-"));

            return Some([hero.position, tile_view.position]);
        }
        None => None,
    }
}

pub fn find_bomb_all(ctx: &GameContext, hero: &Hero) -> Option<[Position; 2]> {
    let closet_value = count_adjacent_units(ctx, hero);

    if closet_value > 1 {
        // eprintln!("Hero[{0}]->{closet_value}", hero.agent_id);
        return occupantion_bombing(ctx, hero);
    }

    let target = find_bomb_target(ctx, hero);

    if let Some(target_value) = target {
        match find_bomb_source(ctx, hero, &target_value) {
            Some(source_pos) => {
                // debug_position(ctx, &source_pos, "#774ac0ff", format!("S"));
                return Some([source_pos, target_value]);
            }
            None => return None,
        }
    }
    None
}
