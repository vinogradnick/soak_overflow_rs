use std::{collections::HashMap, io};

use crate::{
    hero_cmd::{HeroAction, HeroCmd},
    hero_profile::{HeroEntity, HeroProfile},
    map_state::MapState,
    position::Position,
    reader::{Reader, read_value},
};

// ---------- SimReader ----------
pub struct SimReader {
    values: Vec<i32>,
    profiles: Vec<HeroProfile>,
    entities: Vec<HeroEntity>,
    read_profiles_once: bool,
    map: MapState,
    cursor: usize,
}
impl SimReader {
    fn apply_hero_commands(&mut self, cmd: HeroCmd) -> Result<(), Box<dyn std::error::Error>> {
        for action in cmd.actions {
            self.apply_action(cmd.hero_id, action)?;
        }
        Ok(())
    }

    fn apply_action(
        &mut self,
        id: i32,
        action: HeroAction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(entity) = self.entities.get_mut(id as usize) {
            match action {
                HeroAction::Move(position) => {
                    let tile = self
                        .map
                        .get_tile_mut(position.x as usize, position.y as usize);

                    match tile {
                        Some(t) => {
                            if !t.is_occupied() {
                                t.entity_id = entity.agent_id;
                                entity.position = position;
                            }
                        }
                        None => {
                            eprintln!("Error: invalid position {}", &position)
                        }
                    }

                    eprintln!(" Entity{} Move  to {}", id, position);
                }
                HeroAction::Shoot(target_id) => {
                    let target = self.entities.get_mut(target_id as usize);
                    if let Some(hit) = target {
                        hit.wetness += 6;

                        eprintln!(" Entity {} Shoot  to Entity {}", id, hit.agent_id);
                    } else {
                        eprintln!("Cannot Shoot Entity {}", target_id);
                    }
                }
                HeroAction::Wait => {
                    println!("Entity {} waits", id);
                    // entity.wait();
                }
            }
        } else {
            return Err(format!("Entity with id {} not found", id).into());
        }
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

    fn step(&mut self, cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
        let commands = cmd
            .split(";")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        // Первый элемент - ID агента
        let agent_id: i32 = commands[0].parse().unwrap();

        let mut hero_cmd = HeroCmd::new(agent_id);

        // Обрабатываем остальные команды
        for command in &commands[1..] {
            let parts = command.split_whitespace().collect::<Vec<_>>();

            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "MOVE" => {
                    if parts.len() == 3 {
                        let x: i32 = parts[1].parse()?;
                        let y: i32 = parts[2].parse()?;
                        hero_cmd.actions.push(HeroAction::Move(Position { x, y }));
                    } else {
                        return Err(format!("Invalid MOVE command: {}", command).into());
                    }
                }
                "SHOOT" => {
                    if parts.len() == 2 {
                        let target_id: i32 = parts[1].parse()?;
                        hero_cmd.actions.push(HeroAction::Shoot(target_id));
                    } else {
                        return Err(format!("Invalid SHOOT command: {}", command).into());
                    }
                }
                "WAIT" => {
                    hero_cmd.actions.push(HeroAction::Wait);
                }
                _ => {
                    return Err(format!("Unknown command: {}", parts[0]).into());
                }
            }
        }

        // Применяем все действия героя
        self.apply_hero_commands(hero_cmd)?;

        Ok(())
    }

    fn read_map(&mut self) -> MapState {
        let map_container = r#"0000000000000
0101300032020
4000000000004
0202300031010
0000000000000"#;

        self.map = MapState::from_str(map_container);

        MapState::from_str(map_container)
    }

    fn read_profiles(&mut self, _owner_id: i32) -> Vec<HeroProfile> {
        let input = r#"Profile:1 0 0 6 50 0
Profile:2 0 0 6 50 0
Profile:3 1 0 6 100 0
Profile:4 1 0 6 100 0
Profile:5 1 0 6 100 0
Profile:6 1 0 6 100 0
"#;
        for line in input.lines() {
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

        let input = r#"1 0 2 0 0 0
2 12 2 0 0 0
3 4 1 0 0 0
4 4 3 0 0 0
5 8 1 0 0 0
6 8 3 0 0 0
        "#;

        for line in input.lines() {
            let inputs: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            if inputs.len() < 6 {
                continue;
            }

            let agent_id = inputs[0];
            let x = inputs[1];
            let y = inputs[2];
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
            read_profiles_once: false,
            map: MapState {
                height: 1,
                width: 0,
                tiles: vec![],
            },
            values: vec![0],
            cursor: 0,
        }
    }
}
