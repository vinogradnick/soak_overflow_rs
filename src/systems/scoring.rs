use crate::{data::context::GameContext, hero::hero_entity::HeroEntity};

pub fn is_surrounded(ctx: &GameContext, hero: &HeroEntity) -> bool {
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let pos = hero.position;

    for (dx, dy) in dirs {
        let nx = pos.x as i32 + dx;
        let ny = pos.y as i32 + dy;

        if ctx.map_state.in_bounds(nx as usize, ny as usize) {
            if let Some(tile) = ctx.map_state.get_tile(nx as usize, ny as usize) {
                // Если хотя бы один сосед свободен и проходим
                if !tile.is_occupied() && !tile.is_cover() {
                    return false;
                }
            }
        }
    }
    true
}
