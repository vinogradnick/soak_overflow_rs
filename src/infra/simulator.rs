use crate::{
    data::{
        game_context::GameContext,
        hero::{Hero, HeroAction, HeroActionVariant},
        position::Position,
        tile::Occupant,
    },
    infra::{logger, pathfinder},
};

pub fn simulator_action(ctx: &mut GameContext, actions: Vec<HeroAction>) -> Result<(), String> {
    for act in actions {
        if act.1.len() == 0 {
            return Err("Undefined Length".to_owned());
        }

        eprintln!("ACTION: {}", act.to_string());
        let id = act.0;
        let loc_actions = act.1;

        let heroes = ctx.hero_store.heroes.clone();

        let hero_entity = heroes.iter().find(|x| x.agent_id == id);

        for action in loc_actions {
            match action {
                HeroActionVariant::Move(position) => {
                    apply_move_action(ctx, position, hero_entity.clone());
                }
                HeroActionVariant::Throw(position) => {
                    apply_throw_action(ctx, position, hero_entity.clone());
                }
                HeroActionVariant::Shoot { id } => {
                    let hero = ctx
                        .hero_store
                        .heroes
                        .iter()
                        .find(|x| x.agent_id == id as i32);
                }
                HeroActionVariant::HunkerDown => {
                    let hero = ctx
                        .hero_store
                        .heroes
                        .iter()
                        .find(|x| x.agent_id == id as i32);
                }
                HeroActionVariant::Message { text } => {}
            }
        }
    }
    Ok(())
}

pub fn apply_throw_action(ctx: &mut GameContext, position: Position, hero: Option<&Hero>) {
    let target_hero = hero.unwrap();

    for tile in ctx
        .tilemap
        .tiles
        .iter_mut()
        .filter(|x| x.position.distance_8x(&position) <= 1)
    {
        match tile.occupant {
            Occupant::Enemy(id) => {
                ctx.hero_store.heroes.retain(|x| x.agent_id == id as i32);
                tile.occupant = Occupant::Nil;
            }
            Occupant::Owner(id) => {
                ctx.hero_store.heroes.retain(|x| x.agent_id == id as i32);
                tile.occupant = Occupant::Nil;
            }
            Occupant::Nil => continue,
        };
    }
}

pub fn apply_move_action(
    ctx: &mut GameContext,
    position: Position,
    source_hero: Option<&Hero>,
) -> Result<(), String> {
    let hero = source_hero.unwrap();

    if ctx
        .tilemap
        .get_tile(&position)
        .is_none_or(|tile| !tile.is_free())
    {
        return Err("Tile Occuped".to_owned());
    }

    // eprintln!("apply_move_action:[{}] [{}]", position, hero.position);

    if position.distance(&hero.position) > 1 {
        let path = pathfinder::find_path(ctx, &hero.position, &position);

        if let Some(t) = path {
            for p in t {
                if hero.position != p {
                    let new_hero = ctx
                        .hero_store
                        .heroes
                        .iter_mut()
                        .find(|x| x.agent_id == hero.agent_id);

                    match new_hero {
                        Some(hiro) => {
                            hiro.position = p;
                            let tile = ctx.tilemap.get_tile_mut(&p);

                            match tile {
                                Some(tile_mut) => {
                                    if tile_mut.occupant == Occupant::Nil {
                                        tile_mut.occupant = Occupant::Owner(hiro.agent_id as usize);
                                    }
                                }
                                None => {}
                            }
                        }
                        None => {}
                    }

                    return Ok(());
                }
            }
        }
    } else {
        let new_hero = ctx
            .hero_store
            .heroes
            .iter_mut()
            .find(|x| x.agent_id == hero.agent_id);

        match new_hero {
            Some(hiro) => {
                hiro.position = position.clone();
                let tile = ctx.tilemap.get_tile_mut(&position);

                match tile {
                    Some(tile_mut) => {
                        if tile_mut.occupant == Occupant::Nil {
                            tile_mut.occupant = Occupant::Owner(hiro.agent_id as usize);
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
    };
    Ok(())
}
