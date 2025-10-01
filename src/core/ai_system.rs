use crate::{
    core::{predict_system::PredictSystem, shooter_system::ShooterSystem},
    data::{
        game_context::GameContext,
        hero::{Hero, HeroAction, HeroActionVariant},
        position::Position,
        tile::TileView,
    },
    infra::{logger, lru::LruCache, pathfinder, position_utils::find_cover_position},
};

#[derive(Debug, Default)]
pub struct AiSystem {
    tile_cache: LruCache<&'static str, Vec<TileView>>,
    hero_cache: LruCache<&'static str, Vec<Hero>>,
}

impl AiSystem {
    pub fn new() -> Self {
        AiSystem {
            tile_cache: LruCache::new(10),
            hero_cache: LruCache::new(10),
        }
    }
    pub fn process(&mut self, ctx: &GameContext) -> Vec<HeroAction> {
        logger::log("", "AiSystem::process");

        if self.tile_cache.get(&"cover_tiles").is_none() {
            logger::log("", "AiSystem::process:find_cover_position");
            let covered = find_cover_position(ctx, 1);
            self.tile_cache.put(
                "cover_tiles",
                covered
                    .iter()
                    .map(|tile| ctx.tilemap.get_tile(tile).unwrap().clone())
                    .collect(),
            );
        }

        let (my_score, enemy_score) = PredictSystem::predict(ctx);

        let is_enemy_winner = my_score < enemy_score;

        let covers = self.tile_cache.get(&"cover_tiles").unwrap();
        let mut filtered: Vec<&TileView> = covers.iter().filter(|x| x.is_free()).collect();

        let mut hero_actions = vec![];

        let limit = 2;

        for hero in ctx.hero_store.heroes.iter().filter(|x| x.is_owner) {
            let mut inner_actions = vec![];

            // то ищем более агресивную позицию
            if is_enemy_winner {
                let target = filtered
                    .iter()
                    .max_by_key(|tile| tile.position.distance(&hero.position));

                if let Some(t_tile) = target {
                    inner_actions.push(HeroActionVariant::Move(t_tile.position));
                    filtered.retain(|t| t.position == t_tile.position);
                }
            }

            let mut is_shooted = false;
            // прворяем стреляли мы уже или нет
            for item in &inner_actions {
                match item {
                    HeroActionVariant::Shoot { id } => {
                        is_shooted = true;
                        break;
                    }
                    _ => {}
                }
            }

            if !is_shooted {
                match ShooterSystem::find_enemy(ctx, hero) {
                    Some(t) => {
                        logger::log(t, "ShooterSystem::find_enemy::Result");

                        inner_actions.push(HeroActionVariant::Shoot { id: t.agent_id });
                    }
                    None => {}
                }
            }

            if ctx
                .hero_store
                .heroes
                .iter()
                .filter(|x| !x.is_owner)
                .all(|x| x.cooldown > 0 || x.optimal_range < hero.position.distance(&x.position))
            {
                let mut clone_position = hero.position.clone();

                clone_position.x = ctx.tilemap.get_width() / 2;

                let center = pathfinder::find_path(ctx, &hero.position, &clone_position);

                if let Some(values) = center {
                    for point in values.iter().skip(1) {
                        inner_actions.push(HeroActionVariant::Move(point.clone()));
                        break;
                    }
                }
            }

            if inner_actions.len() == 0 {
                inner_actions.push(HeroActionVariant::Message {
                    text: format!("Think"),
                });
            }

            hero_actions.push(HeroAction(hero.agent_id, inner_actions));
        }

        return hero_actions;

        // проверить что каждый герой защищен
        // выбрать одного героя который будет продвигаться
    }
}
