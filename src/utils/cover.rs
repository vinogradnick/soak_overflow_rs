use crate::{data::context::GameContext, data::position::Position, hero::hero_entity::HeroEntity};

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

    nearby_covers.sort_by_key(|(dist, tile)| *dist / tile.tile_type as i32);

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
