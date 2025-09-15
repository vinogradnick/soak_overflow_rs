use std::{collections::HashMap, io};

use crate::{
    hero::{hero_cmd::HeroCommand, hero_entity::HeroEntity, hero_profile::HeroProfile},
    map_state::{MapState, Tile},
    position::Position,
    reader::{Reader, read_value},
};

// ---------- CodinGame Reader ----------
pub struct CGReader;
impl Reader for CGReader {
    fn read_i32(&mut self) -> i32 {
        read_value::<i32>()
    }
    fn new() -> Self {
        return CGReader;
    }

    fn get_count(&mut self) -> usize {
        read_value::<usize>()
    }

    fn step(&mut self, cmd: &HeroCommand) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", cmd);
        Ok(())
    }

    fn read_map(&mut self) -> MapState {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.trim().split_whitespace().collect::<Vec<_>>();
        let width: usize = inputs[0].parse().unwrap();
        let height: usize = inputs[1].parse().unwrap();

        let mut tiles = Vec::with_capacity(width * height);
        eprintln!("{} {}", width, height);

        for _ in 0..height {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.trim().split_whitespace().collect::<Vec<_>>();

            for j in 0..width {
                let x: usize = inputs[3 * j].parse().unwrap();
                let y: usize = inputs[3 * j + 1].parse().unwrap();
                let tile_type: i32 = inputs[3 * j + 2].parse().unwrap();
                tiles.push(Tile {
                    position: Position::new(x, y),
                    tile_type,
                    entity_id: 0,
                });
            }
        }

        MapState::new(width, height, tiles)
    }

    fn read_profiles(&mut self, owner_id: i32) -> Vec<HeroProfile> {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let agent_data_count: i32 = s.trim().parse().unwrap();

        let mut profiles = Vec::new();
        for _ in 0..agent_data_count {
            s.clear();
            io::stdin().read_line(&mut s).unwrap();

            eprintln!("Profile:{}", &s);

            let inputs: Vec<_> = s.split_whitespace().collect();

            let agent_id: i32 = inputs[0].parse().unwrap();
            let player: i32 = inputs[1].parse().unwrap();
            let shoot_cooldown: i32 = inputs[2].parse().unwrap();
            let optimal_range: i32 = inputs[3].parse().unwrap();
            let soaking_power: i32 = inputs[4].parse().unwrap();
            let splash_bombs: i32 = inputs[5].parse().unwrap();

            profiles.push(HeroProfile {
                is_owner: owner_id == player,
                agent_id,
                player,
                shoot_cooldown,
                optimal_range,
                soaking_power,
                splash_bombs,
            });
        }
        profiles
    }

    fn read_entities(&mut self, profiles: &HashMap<i32, HeroProfile>) -> Vec<HeroEntity> {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let agent_count: i32 = s.trim().parse().unwrap();

        let mut entities = Vec::new();
        for _ in 0..agent_count {
            s.clear();
            io::stdin().read_line(&mut s).unwrap();

            eprintln!("{}", &s);

            let inputs: Vec<_> = s.split_whitespace().collect();

            let agent_id: i32 = inputs[0].parse().unwrap();
            let x: usize = inputs[1].parse().unwrap();
            let y: usize = inputs[2].parse().unwrap();
            let cooldown: i32 = inputs[3].parse().unwrap();
            let splash_bombs: i32 = inputs[4].parse().unwrap();
            let wetness: i32 = inputs[5].parse().unwrap();

            if let Some(profile) = profiles.get(&agent_id) {
                entities.push(HeroEntity {
                    position: Position::new(x, y),
                    is_owner: profile.is_owner,
                    agent_id,
                    cooldown,
                    splash_bombs,
                    wetness,
                });
            }
        }
        entities
    }
}
