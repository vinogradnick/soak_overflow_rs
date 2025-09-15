use crate::{
    context::GameContext,
    hero::hero_entity::HeroEntity,
    position::Position,
    utils::{cover::get_hero_cover_quality, pathfinder::find_path_with_cost},
};

pub fn find_all_map_bomb_position<'a>(ctx: &'a GameContext, position: &'a Position) {
    let items = ctx.map_state.tiles.iter().filter(|&tile| !tile.is_cover());

    let mut step = 0;
    for tile in items {
        let has_path = find_path_with_cost(ctx, position, &tile.position);

        if let Some(c) = has_path {
            for p in c {
                let t = ctx.map_state.get_tile(p.0.x, p.0.y);

                if let Some(tow) = t {
                    if tow.is_free() {
                        crate::viz::render::debug_position(
                            ctx,
                            &p.0,
                            "#25dd00fb",
                            format!("cost:{}", p.1),
                        );
                    }
                }
            }
        }
        if step + 1 > 15 {
            step = 0;
        } else {
            step += 1;
        }
    }
}

pub fn find_safe_bomb_position<'a>(
    ctx: &'a GameContext,
    position: &'a Position,
) -> Option<(Position, Position)> {
    let items = ctx
        .map_state
        .neighbors_range(position)
        .filter(|&tile| tile.is_free())
        .min_by_key(|nbh| check_for_bomb(ctx, &nbh.position));

    if let Some(point) = items {
        // crate::viz::render::debug_position(
        //     ctx,
        //     &point.position,
        //     "#ce2743ff",
        //     format!("Count:{}", 1),
        // );

        if let Some(closest) = find_bomb_closest_target(ctx, &point.position) {
            if point.position.dist(closest) >= 2 {
                return Some((point.position, closest.clone()));
            }
        }
    }

    None
}

pub fn check_for_bomb(ctx: &GameContext, position: &Position) -> usize {
    let enemies = ctx
        .hero_service
        .enemy_list()
        .filter(|enemy| enemy.position.dist(position) <= 1)
        .count();

    return enemies;
}

pub fn find_bomb_closest_target<'a>(
    ctx: &'a GameContext<'a>,
    position: &'a Position,
) -> Option<&'a Position> {
    let min_x = 0;
    let min_y = 0;
    let max_x = position.x - 2;
    let max_y = position.y + 1;

    let enemies: Vec<_> = ctx
        .hero_service
        .enemy_list()
        .filter(|p| p.position.dist(position) <= 3)
        .collect();

    let mut out = (0, position);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(tile) = ctx.map_state.get_tile(x, y) {
                if !tile.is_cover() && position.dist(&tile.position) >= 2 {
                    let sum = enemies
                        .iter()
                        .filter(|en| en.position.dist(&tile.position) <= 2)
                        .count();

                    // crate::viz::render::debug_position(
                    //     ctx,
                    //     &tile.position,
                    //     "#92147dff",
                    //     format!("Count:{}", sum),
                    // );

                    if sum > out.0 {
                        out = (sum, &tile.position);
                    }
                }
            }
        }
    }

    return Some(out.1);
}

pub fn find_bomb_target<'a>(ctx: &GameContext, position: &Position) -> Option<Position> {
    let enemies: Vec<_> = ctx
        .hero_service
        .enemy_list()
        .filter(|x| x.position.dist(&position) <= 4)
        .collect();

    let width = ctx.map_state.width;
    let height = ctx.map_state.height;

    // Создаём карту счёта тайлов
    let mut score_map = vec![vec![0u8; width]; height];

    // Радиус взрыва бомбы
    let radius = 2;

    // Для каждого врага повышаем счёт в радиусе взрыва
    for e in &enemies {
        let min_x = e.position.x.saturating_sub(radius as usize);
        let max_x = (e.position.x + radius as usize).min(width - 1);

        let min_y = e.position.y.saturating_sub(radius as usize);
        let max_y = (e.position.y + radius as usize).min(height - 1);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                score_map[y][x] += 1;
            }
        }
    }
    let own_heroes: Vec<_> = ctx.hero_service.my_list().collect();

    // Ищем тайл с максимальным score
    let mut best_pos = None;
    let mut max_score = 0;

    for y in 0..height {
        for x in 0..width {
            if score_map[y][x] > max_score {
                let p: Position = Position { x, y };
                let mut can_bombed = true;

                for o_hero in &own_heroes {
                    if o_hero.position.in_radius(&p, 2) {
                        can_bombed = false;
                    }
                }
                if !can_bombed {
                    continue;
                }

                max_score = score_map[y][x];
                best_pos = Some(p);
            }
        }
    }

    // if let Some(p) = best_pos {
    //     // crate::viz::render::debug_position(ctx, &p, "#b31f44ff", format!("score:{}I", max_score))
    // }

    best_pos
}

pub fn find_shoot_target<'a>(ctx: &GameContext, hero: &HeroEntity) -> Option<i32> {
    let mut enemies = ctx
        .hero_service
        .nearby_enemies(hero, 6)
        .map(|(enemy, _)| enemy)
        .collect::<Vec<_>>();

    enemies.sort_by_key(|enemy| get_hero_cover_quality(ctx, &enemy.position));

    for en in enemies {
        return Some(en.agent_id);
    }
    return None;
}
