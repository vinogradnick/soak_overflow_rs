use std::{fmt::Debug, io, str::FromStr};

use crate::data::{
    game_context::GameContext,
    hero::{Hero, HeroCommand, HeroStore},
    position::Position,
    tile::{Occupant, TileType, TileView},
    tilemap::TileMap,
};
use crate::simulator::simulator_action;

pub enum Reader {
    CodeingameReader,
    SimulatorReader(bool),
}

pub fn read_number_str<T>(str_view: impl AsRef<str>) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    str_view
        .as_ref()
        .trim()
        .parse::<T>()
        .expect("Failed to parse input")
}

pub fn read_number<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    eprintln!("{}", input_line);
    input_line
        .trim()
        .parse::<T>()
        .expect("Failed to parse input")
}

impl Reader {
    pub fn read_number(&self, ctx: &mut GameContext) -> usize {
        match self {
            Reader::CodeingameReader => read_number(),
            Reader::SimulatorReader(_) => ctx.hero_store.heroes.len(),
        }
    }

    pub fn read_entities(&self, ctx: &mut GameContext) {
        for it in ctx.tilemap.tiles.iter_mut() {
            it.occupant = Occupant::Nil;
        }

        match self {
            Reader::CodeingameReader => {
                let agent_count = read_number::<usize>(); // Total number of agents still in the game

                let heroes = ctx.hero_store.heroes.clone();

                ctx.hero_store.heroes.clear();

                for i in 0..agent_count as usize {
                    let mut input_line = String::new();
                    io::stdin().read_line(&mut input_line).unwrap();
                    eprintln!("{}", &input_line);
                    let inputs = input_line
                        .split(" ")
                        .map(|f| read_number_str::<i32>(f))
                        .collect::<Vec<_>>();

                    let hero_id = inputs[0] as usize;
                    let current = heroes.get(&hero_id).unwrap();

                    ctx.hero_store.update_hero(
                        hero_id,
                        &Hero {
                            agent_id: hero_id,
                            is_owner: current.is_owner,
                            shoot_cooldown: current.shoot_cooldown,
                            optimal_range: current.optimal_range,
                            soaking_power: current.soaking_power,
                            splash_bombs: inputs[4],
                            position: Position {
                                x: inputs[1] as usize,
                                y: inputs[2] as usize,
                            },
                            cooldown: inputs[3],
                            wetness: inputs[5],
                            initialized: true,
                        },
                    );
                }
            }
            Reader::SimulatorReader(_) => {
                if ctx
                    .hero_store
                    .heroes
                    .values()
                    .filter(|x| x.initialized)
                    .count()
                    == 0
                {
                    let agent_count = read_number::<usize>(); // Total number of agents still in the game

                    for i in 0..agent_count as usize {
                        let mut input_line = String::new();
                        io::stdin().read_line(&mut input_line).unwrap();
                        eprintln!("{}", &input_line);
                        let inputs = input_line
                            .split(" ")
                            .map(|f| read_number_str::<i32>(f))
                            .collect::<Vec<_>>();
                        let hero_id = inputs[0] as usize;

                        let current = ctx.hero_store.heroes.get(&hero_id).unwrap();

                        ctx.hero_store.update_hero(
                            current.agent_id,
                            &Hero {
                                agent_id: current.agent_id,
                                is_owner: current.is_owner,
                                shoot_cooldown: current.shoot_cooldown,
                                optimal_range: current.optimal_range,
                                soaking_power: current.soaking_power,
                                splash_bombs: inputs[4],
                                position: Position {
                                    x: inputs[1] as usize,
                                    y: inputs[2] as usize,
                                },
                                cooldown: inputs[3],
                                wetness: inputs[5],
                                initialized: true,
                            },
                        );
                    }
                }
            }
        };

        for hero in ctx.hero_store.heroes.values() {
            let tile = ctx.tilemap.get_tile_mut(&hero.position);

            if let Some(t_mut) = tile {
                if hero.is_owner {
                    t_mut.occupant = Occupant::Owner(hero.agent_id);
                } else {
                    t_mut.occupant = Occupant::Enemy(hero.agent_id);
                }
            }
        }
    }
    pub fn read_tilemap(&self, ctx: &mut GameContext) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        eprintln!("{}", &input_line);
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let width: usize = read_number_str(inputs[0]); // Width of the game map
        let height: usize = read_number_str(inputs[1]); // Height of the game map

        ctx.tilemap = TileMap::new(width, height);
        for i in 0..height as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            eprintln!("{}", &input_line);
            let inputs = input_line.split_whitespace().collect::<Vec<_>>();
            for j in 0..width as usize {
                let x: usize = read_number_str(inputs[3 * j]); // X coordinate, 0 is left edge
                let y: usize = read_number_str(inputs[3 * j + 1]); // Y coordinate, 0 is top edge
                let tile_type = read_number_str(inputs[3 * j + 2]);

                ctx.tilemap.tiles[y * width + x] = TileView {
                    position: Position { x, y },
                    tile_type: match tile_type {
                        1 => TileType::LowWall,
                        2 => TileType::HighWall,
                        _ => TileType::Empty,
                    },
                    occupant: Occupant::Nil,
                };
            }
        }
    }
    pub fn read_profiles(&self, ctx: &mut GameContext) {
        let agent_data_count = read_number::<usize>(); // Total number of agents in the game

        ctx.hero_store = HeroStore::new();
        for i in 0..agent_data_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            eprintln!("{}", &input_line);
            let inputs = input_line
                .split(" ")
                .map(read_number_str::<i32>)
                .collect::<Vec<_>>();
            // Создаём героя
            let hero = Hero {
                agent_id: inputs[0] as usize,
                is_owner: ctx.player_id == inputs[1],
                shoot_cooldown: inputs[2],
                optimal_range: inputs[3],
                soaking_power: inputs[4],
                splash_bombs: inputs[5],
                position: Position::default(), // если позиция читается позже
                cooldown: 0,
                wetness: 0,
                initialized: false,
            };

            // Обновляем HeroStore по agent_id
            ctx.hero_store.update_hero(inputs[0] as usize, &hero);
        }
    }

    pub fn read_id(&self, ctx: &mut GameContext) {
        let id = read_number::<i32>();
        ctx.player_id = id;
    }

    pub fn receive_action(
        &self,
        ctx: &mut GameContext,
        actions: Vec<HeroCommand>,
    ) -> Result<(), String> {
        match self {
            Reader::CodeingameReader => codeingame_action(ctx, actions),
            Reader::SimulatorReader(_) => simulator_action(ctx, actions),
        }
    }
}

fn codeingame_action(ctx: &mut GameContext, actions: Vec<HeroCommand>) -> Result<(), String> {
    for act in actions {
        println!("{}", act);
    }
    Ok(())
}
