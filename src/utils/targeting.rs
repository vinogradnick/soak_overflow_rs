use crate::{
    context::GameContext, hero::hero_entity::HeroEntity, position::Position,
    utils::cover::get_hero_cover_quality,
};

pub fn nearest_enemy<'a>(ctx: &GameContext) -> i32 {
    0
}

pub fn k_closest_enemies<'a>(ctx: &GameContext) -> Vec<i32> {
    vec![]
}
pub fn find_save_bomb_position(
    ctx: &GameContext,
    position: &Position,
) -> Option<(Position, Position)> {
    for nbh in ctx.map_state.neighbors(position) {
        if let Some(t) = find_bomb_target(ctx, &nbh.position) {
            return Some((nbh.position.clone(), t.clone()));
        }
    }
    None
}

pub fn find_bomb_target<'a>(ctx: &GameContext, position: &Position) -> Option<Position> {
    let enemies: Vec<_> = ctx
        .hero_service
        .enemy_list()
        .filter(|x| x.position.m_dist(&position) <= 4)
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

    if let Some(p) = best_pos {
        crate::viz::render::debug_position(
            ctx,
            &p,
            "#b31f44ff",
            format!("score:{}I", max_score).as_str(),
        )
    }

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
