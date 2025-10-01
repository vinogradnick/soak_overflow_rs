use std::{error::Error, io};

use crate::{
    data::{
        game_context::GameContext,
        hero::{Hero, HeroAction},
        position::Position,
        tile::{Occupant, TileType, TileView},
        tilemap::TileMap,
    },
    infra::logger,
};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

pub fn read_input() -> GameContext {
    let mut context = GameContext::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_id = parse_input!(input_line, i32); // Your player id (0 or 1)

    context.player_id = my_id;
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let agent_data_count = parse_input!(input_line, i32); // Total number of agents in the game
    for i in 0..agent_data_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let agent_id = parse_input!(inputs[0], i32); // Unique identifier for this agent
        let player = parse_input!(inputs[1], i32); // Player id of this agent
        let shoot_cooldown = parse_input!(inputs[2], i32); // Number of turns between each of this agent's shots
        let optimal_range = parse_input!(inputs[3], i32); // Maximum manhattan distance for greatest damage output
        let soaking_power = parse_input!(inputs[4], i32); // Damage output within optimal conditions
        let splash_bombs = parse_input!(inputs[5], i32); // Number of splash bombs this can throw this game
        context.hero_store.heroes.push(Hero {
            is_owner: my_id == player,
            agent_id,
            shoot_cooldown,
            optimal_range,
            player,
            splash_bombs,
            position: Position::default(),
            cooldown: shoot_cooldown,
            soaking_power,
            wetness: 0,
            alive: false,
        });
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], usize); // Width of the game map
    let height = parse_input!(inputs[1], usize); // Height of the game map

    context.tilemap = TileMap::new(width, height);
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split_whitespace().collect::<Vec<_>>();
        for j in 0..width as usize {
            let x = parse_input!(inputs[3 * j], i32); // X coordinate, 0 is left edge
            let y = parse_input!(inputs[3 * j + 1], i32); // Y coordinate, 0 is top edge
            let tile_type = parse_input!(inputs[3 * j + 2], i32);

            context.tilemap.tiles.push(TileView {
                position: Position {
                    x: x as usize,
                    y: y as usize,
                },
                occupant: Occupant::Nil,
                tile_type: tile_type.into(),
            })
        }
    }
    return context;

    // game loop
}

pub fn read_for_loop_update(ctx: &mut GameContext) -> Result<(), Box<dyn std::error::Error>> {

    ctx.hero_store.heroes.iter_mut().for_each(|hero| {
        let tile = ctx.tilemap.get_tile_mut(&hero.position).unwrap();
        if hero.player == ctx.player_id {
            tile.occupant = Occupant::Owner(hero.agent_id as usize);
        } else {
            tile.occupant = Occupant::Enemy(hero.agent_id as usize);
        }
    });

    Ok(())
}

pub fn read_for_loop(ctx: &mut GameContext) -> Result<(), Box<dyn std::error::Error>> {
    logger::log_str("", "read_for_loop");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;

    ctx.tilemap.tiles.iter_mut().for_each(|tile| {
        tile.occupant = Occupant::Nil;
    });

    if input_line.is_empty() {
        return Ok(());
    }

    let agent_count = input_line.trim().parse::<i32>()?;

    for i in 0..agent_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let agent_id = parse_input!(inputs[0], i32);
        let x = parse_input!(inputs[1], i32);
        let y = parse_input!(inputs[2], i32);
        let cooldown = parse_input!(inputs[3], i32); // Number of turns before this agent can shoot
        let splash_bombs = parse_input!(inputs[4], i32);
        let wetness = parse_input!(inputs[5], i32); // Damage (0-100) this agent has taken

        let agent = ctx
            .hero_store
            .heroes
            .iter_mut()
            .find(|x| x.agent_id == agent_id)
            .unwrap();

        agent.position = Position {
            x: x as usize,
            y: y as usize,
        };
        agent.splash_bombs = splash_bombs;
        agent.cooldown = cooldown;
        agent.wetness = wetness;
        agent.alive = true;
        let tile_mut = ctx.tilemap.get_tile_mut(&agent.position).unwrap();

        if agent.player == ctx.player_id {
            tile_mut.occupant = Occupant::Owner(agent.agent_id as usize);
        } else {
            tile_mut.occupant = Occupant::Enemy(agent.agent_id as usize);
        }
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_agent_count = parse_input!(input_line, i32); // Number of alive agents controlled by you
    Ok(())
}
