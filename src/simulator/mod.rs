use crate::{
    data::{
        game_context::GameContext,
        hero::{Hero, HeroAction, HeroCommand},
        position::Position,
        tile::Occupant,
    },
    systems::pathfinder,
};

pub fn simulator_read_entities(ctx: &mut GameContext) {}

pub fn simulator_action(ctx: &mut GameContext, actions: Vec<HeroCommand>) -> Result<(), String> {
    for act in actions {
        eprintln!("ACTION: {}", act);
        let id = act.0;
        let loc_actions = act.1;
        let hero_entity = ctx.hero_store.heroes.get(&id).cloned();

        if let Some(hero) = hero_entity {
            for action in loc_actions {
                match action {
                    HeroAction::Move(position) => apply_move_action(ctx, &position, &hero)?,
                    HeroAction::Throw(position) => {
                        let hero_mut = ctx.hero_store.heroes.get_mut(&hero.agent_id).unwrap();

                        if hero_mut.splash_bombs < 1 || hero_mut.position.distance(&position) > 3 {
                            eprintln!(
                                "OUT ACTION ->{}->{:?}",
                                hero_mut.position.distance(&position),
                                hero
                            );
                            return Ok(());
                        }
                        hero_mut.splash_bombs -= 1;

                        for tile in ctx
                            .tilemap
                            .tiles
                            .iter_mut()
                            .filter(|x| x.position.distance_8x(&position) <= 1)
                        {
                            match tile.occupant {
                                Occupant::Enemy(id) => match ctx.hero_store.heroes.remove(&id) {
                                    Some(res) => {
                                        eprintln!("Removed {:?}", res);
                                        tile.occupant = Occupant::Nil;
                                    }
                                    None => {}
                                },
                                Occupant::Owner(id) => match ctx.hero_store.heroes.remove(&id) {
                                    Some(res) => {
                                        eprintln!("Removed {:?}", res);
                                        tile.occupant = Occupant::Nil;
                                    }
                                    None => {}
                                },
                                Occupant::Nil => continue,
                            };
                        }
                    }
                    HeroAction::Shoot(id) => {
                        let target = ctx.hero_store.heroes.get_mut(&(id as usize));
                        match target {
                            Some(hero_item) => {
                                hero_item.wetness += 6;
                            }
                            None => {}
                        }
                    }
                    HeroAction::Wait => {}
                }
            }
        }
    }
    Ok(())
}

pub fn apply_move_action(
    ctx: &mut GameContext,
    position: &Position,
    hero: &Hero,
) -> Result<(), String> {
    if !ctx
        .tilemap
        .get_tile(&position)
        .is_some_and(|tile| tile.is_free())
    {
        return Ok(());
    }

    // eprintln!("apply_move_action:[{}] [{}]", position, hero.position);

    if position.distance(&hero.position) > 1 {
        let path = pathfinder::find_path(ctx, &hero.position, &position);

        if let Some(t) = path {
            for p in t {
                if hero.position != p {
                    let mut hero_clone = hero.clone();

                    hero_clone.position = p;

                    ctx.hero_store.update_hero(hero.agent_id, &hero_clone);

                    let tile = ctx.tilemap.get_tile_mut(&p);
                    if let Some(t) = tile {
                        t.occupant = Occupant::Owner(hero.agent_id);
                    }

                    return Ok(());
                }
            }
        }
    } else {
        {
            let mut hero_clone = hero.clone();
            hero_clone.position = position.clone();
            ctx.hero_store.update_hero(hero.agent_id, &hero_clone);

            let tile = ctx.tilemap.get_tile_mut(position);
            if let Some(t) = tile {
                t.occupant = Occupant::Owner(hero.agent_id);
            }
        }
    };
    Ok(())
}
