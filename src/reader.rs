use std::{fmt::Debug, io, str::FromStr};

use crate::data::{
    game_context::GameContext,
    hero::{Hero, HeroCommand, HeroStore},
    tilemap::TileMap,
};

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
            Reader::SimulatorReader(_) => ctx
                .hero_store
                .iter()
                .filter(|x| x.player == ctx.player_id)
                .count(),
        }
    }

    pub fn read_entities(&self, ctx: &mut GameContext) {
        eprintln!("read_entities");
        match self {
            Reader::CodeingameReader => {
                let agent_count = read_number::<usize>(); // Total number of agents still in the game

                for i in 0..agent_count as usize {
                    let mut input_line = String::new();
                    io::stdin().read_line(&mut input_line).unwrap();
                    eprintln!("{}", &input_line);
                    let inputs = input_line
                        .split(" ")
                        .map(|f| read_number_str::<i32>(f))
                        .collect::<Vec<_>>();

                    ctx.hero_store.update_hero(
                        inputs[0] as usize,
                        &Hero {
                            agent_id: inputs[0],
                            player: -1,
                            shoot_cooldown: -1,
                            optimal_range: -1,
                            soaking_power: -1,
                            splash_bombs: inputs[4],
                            position: (inputs[1] as usize, inputs[2] as usize).into(),
                            cooldown: inputs[3],
                            wetness: inputs[5],
                            initialized: true,
                        },
                    );
                }
            }
            Reader::SimulatorReader(_) => {
                if ctx.hero_store.initialized.iter().filter(|&&x| x).count() == 0 {
                    let agent_count = read_number::<usize>(); // Total number of agents still in the game

                    for i in 0..agent_count as usize {
                        let mut input_line = String::new();
                        io::stdin().read_line(&mut input_line).unwrap();
                        eprintln!("{}", &input_line);
                        let inputs = input_line
                            .split(" ")
                            .map(|f| read_number_str::<i32>(f))
                            .collect::<Vec<_>>();

                        ctx.hero_store.update_hero(
                            inputs[0] as usize,
                            &Hero {
                                agent_id: inputs[0],
                                player: -1,
                                shoot_cooldown: -1,
                                optimal_range: -1,
                                soaking_power: -1,
                                splash_bombs: inputs[4],
                                position: (inputs[1] as usize, inputs[2] as usize).into(),
                                cooldown: inputs[3],
                                wetness: inputs[5],
                                initialized: true,
                            },
                        );
                    }
                }

                ctx.hero_store.iter().for_each(|hero| {
                    if ctx.player_id != hero.agent_id {
                        ctx.tilemap.set_tile_enemy(
                            ctx.tilemap.get_index_raw(hero.position.into()),
                            hero.agent_id,
                        );
                    }
                });
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

                ctx.tilemap
                    .set_tile(ctx.tilemap.get_index((x, y)), &[tile_type]);
            }
        }
    }
    pub fn read_profiles(&self, ctx: &mut GameContext) {
        eprintln!("read_profiles");
        let agent_data_count = read_number::<usize>(); // Total number of agents in the game

        ctx.hero_store = HeroStore::new_count(agent_data_count + 1);
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
                agent_id: inputs[0],
                player: inputs[1],
                shoot_cooldown: inputs[2],
                optimal_range: inputs[3],
                soaking_power: inputs[4],
                splash_bombs: inputs[5],
                position: (0, 0).into(), // если позиция читается позже
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

    pub fn receive_action(&self, ctx: &mut GameContext, actions: Vec<HeroCommand>) {
        match self {
            Reader::CodeingameReader => {
                for act in actions {
                    println!("{}", act);
                }
            }
            Reader::SimulatorReader(_) => {
                for act in actions {
                    let id = act.0;
                    let loc_actions = act.1;

                    for action in loc_actions {
                        match action {
                            crate::data::hero::HeroAction::Move(position) => todo!(),
                            crate::data::hero::HeroAction::Throw(position) => todo!(),
                            crate::data::hero::HeroAction::Shoot(_) => todo!(),
                            crate::data::hero::HeroAction::Wait => todo!(),
                        }
                    }
                }
            }
        }
    }
}
