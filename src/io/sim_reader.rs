use std::{
    collections::HashMap,
    fmt::format,
    fs::{self},
};

use crate::{
    data::{
        map_state::{MapState, Occupant, Tile, TileType},
        position::Position,
    },
    hero::{
        hero_cmd::{HeroAction, HeroCommand},
        hero_entity::HeroEntity,
        hero_profile::HeroProfile,
    },
    io::reader::Reader,
};

// ---------- SimReader ----------
pub struct SimReader {
    profiles: Vec<HeroProfile>,
    entities: Vec<HeroEntity>,
    map: MapState,

    raw_data: Vec<String>,
    raw_cursor: usize,
    readed_entity: bool,
}
impl SimReader {
    fn read_line(&mut self) -> String {
        let item = self.raw_data[self.raw_cursor].clone();
        self.raw_cursor += 1;
        return item;
    }

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
        let hero = self.entities.iter_mut().find(|hero| hero.agent_id == id);

        return match hero {
            Some(t_hero) => match action {
                HeroAction::Move(pos) => self.move_entity(id, *pos),
                HeroAction::Shoot(target_id) => self.shoot_entity(id, *target_id),
                HeroAction::Wait => self.wait_entity(id),
                HeroAction::Throw(position) => {
                    if t_hero.splash_bombs > 0 {
                        self.throw_entities(id, position.clone())?;
                        return Ok(());
                    }
                    return Err(Box::from(format!("Hero empty bombs")));
                }
            },
            None => Err(Box::from(format!("Hero {} is not defined", id))),
        };
    }

    fn evaluate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.entities.retain(|x| x.wetness <= 200);

        Ok(())
    }

    fn throw_entities(
        &mut self,
        agent_id: i32,
        pos: Position,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for e in self
            .entities
            .iter_mut()
            .filter(|e| e.position.multi_distance(&pos) <= 1)
        {
            if e.agent_id == agent_id {
                e.splash_bombs -= 1;
            }
            e.wetness = 999;
        }

        Ok(())
    }

    fn move_entity(&mut self, idx: i32, pos: Position) -> Result<(), Box<dyn std::error::Error>> {
        let entity = self
            .entities
            .iter_mut()
            .find(|x| x.agent_id == idx)
            .unwrap();

        if let Some(tile) = self.map.get_tile_mut(pos.x as usize, pos.y as usize) {
            if tile.is_walkable() {
                tile.occupant = Occupant::OwnerHero(entity.agent_id);
                entity.position = pos;
                return Ok(());
            } else {
                return Err(Box::from(format!(
                    "Cannot move AgentID={}\tTileStatus={}\tTargetPosition={}",
                    entity.agent_id,
                    tile.is_walkable(),
                    tile.position
                )));
            }
        }
        eprintln!("MAPPING_PRINTING");

        self.map.tiles.iter().for_each(|tile| {
            eprintln!("{:?}", tile);
        });
        eprintln!("END_MAPPING_PRINTING");
        return Err(Box::from(format!(
            "(TileNotFound) Cannot move AgentID={}\tHeroPosition=[{}] to [{}] validator:{:?}",
            entity.agent_id,
            entity.position,
            pos,
            (),
        )));
    }

    fn shoot_entity(
        &mut self,
        shooter_idx: i32,
        target_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(target) = self.entities.iter_mut().find(|e| e.agent_id == target_id) {
            target.wetness += 6;
            eprintln!("Entity {} shot Entity {}", shooter_idx, target.agent_id);
        } else {
            eprintln!("Cannot shoot Entity {}", target_id);
        }
        Ok(())
    }

    fn wait_entity(&self, idx: i32) -> Result<(), Box<dyn std::error::Error>> {
        println!("Entity {} waits", idx);
        Ok(())
    }
}

impl Reader for SimReader {
    fn get_count(&mut self) -> usize {
        self.entities.iter().filter(|x| x.is_owner).count()
    }

    fn step(&mut self, cmd: &HeroCommand) -> Result<(), Box<dyn std::error::Error>> {
        self.apply_hero_commands(cmd)?;
        Ok(())
    }

    fn read_map(&mut self) -> MapState {
        let input_line = self.read_line();
        eprintln!("{} ", input_line);
        let inputs = input_line.trim().split_whitespace().collect::<Vec<_>>();
        let width: usize = inputs[0].parse().unwrap();
        let height: usize = inputs[1].parse().unwrap();

        let mut tiles = Vec::with_capacity(width * height);
        eprintln!("{} {}", width, height);

        for _ in 0..height {
            let input_line = self.read_line();

            let inputs = input_line.trim().split_whitespace().collect::<Vec<_>>();
            eprintln!("{}", input_line);
            for j in 0..width {
                let x: usize = inputs[3 * j].parse().unwrap();
                let y: usize = inputs[3 * j + 1].parse().unwrap();
                let tile_type: i32 = inputs[3 * j + 2].parse().unwrap();
                tiles.push(Tile {
                    position: Position::new(x, y),
                    tile_type: TileType::parse_static(tile_type).unwrap_or(TileType::Empty),
                    occupant: Occupant::None,
                });
            }
        }
        self.map = MapState::new(width, height, tiles.clone());

        MapState::new(width, height, tiles)
    }

    fn read_profiles(&mut self, owner_id: i32) -> Vec<HeroProfile> {
        let mut s = self.read_line();
        let agent_data_count: i32 = s.trim().parse().unwrap();

        eprintln!("{}", agent_data_count);

        let mut profiles = Vec::new();
        for _ in 0..agent_data_count {
            s = self.read_line();

            eprintln!("{}", &s);

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
        self.profiles = self.profiles.clone();
        profiles
    }

    fn read_entities(&mut self, profiles: &HashMap<i32, HeroProfile>) -> Vec<HeroEntity> {
        if self.readed_entity {
            return self.entities.clone();
        }

        let mut s = self.read_line();
        let agent_count: i32 = s.trim().parse().unwrap();
        eprintln!("{}", agent_count);
        let mut entities = Vec::new();
        for _ in 0..agent_count {
            s = self.read_line();
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
        self.readed_entity = true;
        self.entities = entities.clone();
        entities
    }

    fn new(_verbose: bool) -> Self {
        Self {
            profiles: vec![],
            entities: vec![],
            raw_data: fs::read_to_string("./case3.txt")
                .unwrap()
                .lines()
                .map(|x| x.to_string())
                .collect(),
            map: MapState::new(0, 0, vec![]),

            raw_cursor: 0,
            readed_entity: false,
        }
    }

    fn read_i32(&mut self) -> i32 {
        self.read_line().trim().parse::<i32>().ok().unwrap()
    }
}
