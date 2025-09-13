use std::collections::BinaryHeap;

use crate::{
    context::GameContext,
    hero_profile::HeroEntity,
    map_state::{MapState, Tile},
    position::Position,
};

pub struct Pathfinder;

impl Pathfinder {
    #[inline]
    pub fn to_index(width: usize, pos: &Position) -> i32 {
        pos.x * width as i32 + pos.y
    }
    #[inline]
    pub fn to_index_raw(width: usize, x: i32, y: i32) -> i32 {
        x * width as i32 + y
    }

    const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    pub fn nearest_enemy<'a>(
        pos: &Position,
        all_heroes: impl Iterator<Item = &'a HeroEntity>,
    ) -> Option<&'a HeroEntity> {
        all_heroes
            .filter(|h| !h.is_owner) // только враги
            .min_by_key(|h| h.position.m_dist(pos)) // выбираем минимальное расстояние
    }
    ///  ищет таргет до цели перпендикулярно
    pub fn find_shoot_target<'a>(
        pos: &Position,
        all_heroes: impl Iterator<Item = &'a HeroEntity>,
    ) -> Option<&'a HeroEntity> {
        all_heroes
            .filter(|h| !h.is_owner)
            .filter(|hero| hero.position.x.eq(&pos.x) || hero.position.y.eq(&pos.y))
            .min_by_key(|hero| hero.position.m_dist(pos))
    }
    pub fn find_shoot_target_by<'a>(
        pos: &Position,
        all_heroes: impl Iterator<Item = &'a HeroEntity>,
    ) -> Option<&'a HeroEntity> {
        all_heroes
            .filter(|h| !h.is_owner)
            .filter(|hero| hero.position.x.eq(&pos.x) || hero.position.y.eq(&pos.y))
            .min_by(|l, r| {
                let dl = l.position.m_dist(pos);
                let dr = r.position.m_dist(pos);

                dl.cmp(&dr).then_with(|| l.wetness.cmp(&r.wetness))
            })
    }

    pub fn get_cover_against<'a>(
        hero: &HeroEntity,
        enemy: &HeroEntity,
        tiles: &[Tile],
        size: (usize, usize),
    ) -> Option<&'a Tile> {
        for (dx, dy) in Pathfinder::DIRECTIONS {
            let target_x = hero.position.x + dx;
            let target_y = hero.position.y + dy;
            let pos: Position = Position::new(target_x, target_y);

            if !MapState::is_in_map(&pos, size) {
                continue;
            }

            let index = MapState::to_index(size.0, target_x as usize, target_y as usize);

            if tiles[index].get_cover_value() > 0.0 {}

            let direction = hero.position.dir(&enemy.position);
        }
        None
    }

    pub fn k_closest_enemies<'a>(
        hero: &HeroEntity,
        enemies: impl Iterator<Item = &'a HeroEntity>,
        k: usize,
        distance: usize,
    ) -> Vec<&'a HeroEntity> {
        let mut vec: Vec<&HeroEntity> = enemies.collect();

        vec.sort_by_key(|e| e.position.m_dist(&hero.position));
        vec.truncate(k); // если врагов меньше k — просто вернёт всех
        vec
    }
    pub fn check_enemy_cover<'a>(
        ctx: &'a GameContext,
        hero: &HeroEntity,
        enemy: &HeroEntity,
    ) -> Option<&'a Tile> {
        let is_linear_with = Position::is_linear(&enemy.position, &hero.position);

        for (x_pos, y_pos) in Pathfinder::DIRECTIONS {
            let dx = enemy.position.x + x_pos;
            let dy = enemy.position.y + y_pos;
            if MapState::is_in_map(ctx.map_state, dx as usize, dy as usize) {
                continue;
            }

            let index = Pathfinder::to_index_raw(size.0, dx, dy);
            let tile = tiles[index as usize];
            if tile.tile_type < 1 || Position::is_linear(&tile.position, &hero.position) {
                continue;
            }
        }
        None
    }
    pub fn near_enemies_pos<'a>(
        ctx: &'a GameContext,
        hero: &'a HeroEntity,
        max_dist: i32,
    ) -> Vec<&'a HeroEntity> {
        ctx.hero_service
            .enemy_list()
            .filter(|x| x.position.m_dist(&hero.position) <= max_dist)
            .collect()
    }

    pub fn closet_cover_point<'a>(ctx: &GameContext, hero: &HeroEntity) -> (Position, i32) {
        let near_tile = ctx.map_state.near_tile_pos(&hero.position, 2);

        let near_enemies = ctx
            .hero_service
            .enemy_list()
            .filter(|x| x.position.m_dist(&hero.position) <= 5)
            .collect::<Vec<_>>();

        let mut target_tile = near_tile.unwrap().position.clone();
        let mut t_dir = 1;

        for en in near_enemies {
            let ta = en.position.dir(&hero.position);

            if ta.x > 0 {
                t_dir = 1;
            } else {
                t_dir = -1;
            }
        }
        target_tile.x += t_dir;

        return (target_tile, 0);
    }
}
