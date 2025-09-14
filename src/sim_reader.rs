use std::{collections::HashMap, fs, io};

use crate::{
    hero::{
        hero_cmd::{HeroAction, HeroCommand},
        hero_entity::HeroEntity,
        hero_profile::HeroProfile,
    },
    map_state::MapState,
    position::Position,
    reader::Reader,
};

// ---------- SimReader ----------
pub struct SimReader {
    values: Vec<i32>,
    profiles: Vec<HeroProfile>,
    entities: Vec<HeroEntity>,
    map: MapState,
    cursor: usize,
}
impl SimReader {
    fn apply_hero_commands(&mut self, cmd: &HeroCommand) -> Result<(), Box<dyn std::error::Error>> {
        for action in &cmd.1 {
            self.apply_action(cmd.0, action)?;
        }
        self.evaluate()?;
        Ok(())
    }

    fn apply_action(
        &mut self,
        id: i32,
        action: &HeroAction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let idx = self
            .entities
            .iter()
            .position(|e| e.agent_id == id)
            .ok_or_else(|| format!("Entity with id {} not found", id))?;

        match action {
            HeroAction::Move(pos) => self.move_entity(idx, *pos),
            HeroAction::Shoot(target_id) => self.shoot_entity(idx, *target_id),
            HeroAction::Wait => self.wait_entity(idx),
            HeroAction::Throw(position) => self.throw_entities(position.clone()),
        }
    }

    fn evaluate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.entities.retain(|x| x.wetness <= 200);

        Ok(())
    }

    fn throw_entities(&mut self, pos: Position) -> Result<(), Box<dyn std::error::Error>> {
        let radius = 1;

        for e in self.entities.iter_mut().filter(|e| {
            let dx = e.position.x as i32 - pos.x as i32;
            let dy = e.position.y as i32 - pos.y as i32;
            dx.abs() <= radius && dy.abs() <= radius
        }) {
            e.wetness = 999;
        }

        Ok(())
    }

    fn move_entity(&mut self, idx: usize, pos: Position) -> Result<(), Box<dyn std::error::Error>> {
        let entity = &mut self.entities[idx];
        if let Some(tile) = self.map.get_tile_mut(pos.x as usize, pos.y as usize) {
            if !tile.is_occupied() {
                tile.entity_id = entity.agent_id;
                entity.position = pos;
                eprintln!("Entity {} moved to {}", entity.agent_id, pos);
            } else {
                eprintln!("Tile at {} is occupied", pos);
            }
        } else {
            eprintln!("Invalid position {}", pos);
        }
        Ok(())
    }

    fn shoot_entity(
        &mut self,
        shooter_idx: usize,
        target_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let shooter = self.entities[shooter_idx];
        if let Some(target) = self.entities.iter_mut().find(|e| e.agent_id == target_id) {
            target.wetness += 6;
            eprintln!(
                "Entity {} shot Entity {}",
                shooter.agent_id, target.agent_id
            );
        } else {
            eprintln!("Cannot shoot Entity {}", target_id);
        }
        Ok(())
    }

    fn wait_entity(&self, idx: usize) -> Result<(), Box<dyn std::error::Error>> {
        let entity = &self.entities[idx];
        println!("Entity {} waits", entity.agent_id);
        Ok(())
    }
}

impl Reader for SimReader {
    fn read_i32(&mut self) -> i32 {
        let v = self.values[self.cursor];
        self.cursor += 1;
        return v;
    }

    fn get_count(&mut self) -> usize {
        2
    }

    fn step(&mut self, cmd: &HeroCommand) -> Result<(), Box<dyn std::error::Error>> {
        // Применяем все действия героя
        self.apply_hero_commands(cmd)?;

        Ok(())
    }

    fn read_map(&mut self) -> MapState {
        let map_container = fs::read_to_string("./data_source/map.txt").unwrap();

        self.map = MapState::from_str(&map_container);

        MapState::from_str(&map_container)
    }

    fn read_profiles(&mut self, _owner_id: i32) -> Vec<HeroProfile> {
        let profiles = fs::read_to_string("./data_source/profile.txt").unwrap();
        for line in profiles.lines() {
            // убираем префикс "Profile:"
            let line = line.strip_prefix("Profile:").unwrap_or(line);

            let inputs: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            if inputs.len() != 6 {
                continue;
            }

            let agent_id = inputs[0];
            let player = inputs[1];
            let shoot_cooldown = inputs[2];
            let optimal_range = inputs[3];
            let soaking_power = inputs[4];
            let splash_bombs = inputs[5];

            self.profiles.push(HeroProfile {
                is_owner: _owner_id == player,
                agent_id,
                player,
                shoot_cooldown,
                optimal_range,
                soaking_power,
                splash_bombs,
            });
        }
        self.profiles.clone()
    }

    fn read_entities(&mut self, _profiles: &HashMap<i32, HeroProfile>) -> Vec<HeroEntity> {
        if !self.entities.is_empty() {
            return self.entities.clone();
        }

        let entities = fs::read_to_string("./data_source/entities.txt").unwrap();

        for line in entities.lines() {
            let inputs: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            if inputs.len() < 6 {
                continue;
            }

            let agent_id = inputs[0];
            let x = inputs[1] as usize;
            let y = inputs[2] as usize;
            let cooldown = inputs[3];
            let splash_bombs = inputs[4];
            let wetness = inputs[5];

            if let Some(profile) = _profiles.get(&agent_id) {
                self.entities.push(HeroEntity {
                    position: Position::new(x, y),
                    is_owner: profile.is_owner,
                    agent_id,
                    cooldown,
                    splash_bombs,
                    wetness,
                });
            }
        }

        self.entities.clone()
    }

    fn new() -> Self {
        Self {
            profiles: vec![],
            entities: vec![],

            map: MapState {
                height: 1,
                width: 0,
                tiles: vec![],
                scoring: vec![],
            },
            values: vec![0],
            cursor: 0,
        }
    }
}
