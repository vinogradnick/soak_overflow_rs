pub mod cg_reader {
    use crate::{
        hero::{hero_cmd::HeroCommand, hero_entity::HeroEntity, hero_profile::HeroProfile},
        map_state::{MapState, Tile},
        position::Position,
        reader::{read_value, Reader},
    };
    use std::{collections::HashMap, io};
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
            MapState {
                height,
                width,
                tiles,
                scoring: vec![],
            }
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
}
pub mod context {
    use crate::{hero::hero_service::HeroService, map_state::MapState};
    pub struct GameContext<'a> {
        pub hero_service: &'a HeroService,
        pub map_state: &'a MapState,
    }
    impl<'a> GameContext<'a> {
        pub fn new(hero_service: &'a HeroService, map_state: &'a MapState) -> Self {
            Self {
                hero_service,
                map_state,
            }
        }
    }
    pub struct GameContextMut<'a> {
        pub hero_service: &'a mut HeroService,
        pub map_state: &'a mut MapState,
    }
    impl<'a> GameContextMut<'a> {
        pub fn new(hero_service: &'a mut HeroService, map_state: &'a mut MapState) -> Self {
            Self {
                hero_service,
                map_state,
            }
        }
    }
}
pub mod hero {
    pub mod hero_cmd {
        use crate::position::Position;
        use std::fmt;
        #[derive(Debug, Clone)]
        pub enum HeroAction {
            Move(Position),
            Throw(Position),
            Shoot(i32),
            Wait,
        }
        impl fmt::Display for HeroAction {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    HeroAction::Move(pos) => write!(f, "MOVE {} {}", pos.x, pos.y),
                    HeroAction::Shoot(id) => write!(f, "SHOOT {}", id),
                    HeroAction::Wait => write!(f, "WAIT"),
                    HeroAction::Throw(position) => write!(f, "THROW {} {}", position.x, position.y),
                }
            }
        }
        #[derive(Debug)]
        pub struct HeroCommand(pub i32, pub Vec<HeroAction>);
        impl fmt::Display for HeroCommand {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let actions_str: Vec<String> = self.1.iter().map(|a| a.to_string()).collect();
                write!(f, "{}; {}", self.0, actions_str.join("; "))
            }
        }
    }
    pub mod hero_entity {
        use crate::position::Position;
        #[derive(Debug, Clone, Copy)]
        pub struct HeroEntity {
            pub position: Position,
            pub is_owner: bool,
            pub agent_id: i32,
            pub cooldown: i32,
            pub splash_bombs: i32,
            pub wetness: i32,
        }
        impl HeroEntity {
            pub fn fields_vec(&self) -> Vec<String> {
                vec![
                    format!("id: {}", self.agent_id),
                    format!("owner: {}", self.is_owner),
                    format!("cd: {}", self.cooldown),
                    format!("bombs: {}", self.splash_bombs),
                    format!("wet: {}", self.wetness),
                    format!("pos: ({},{})", self.position.x, self.position.y),
                ]
            }
        }
    }
    pub mod hero_profile {
        #[derive(Debug, Clone, Copy)]
        pub struct HeroProfile {
            pub is_owner: bool,
            pub agent_id: i32,
            pub player: i32,
            pub shoot_cooldown: i32,
            pub optimal_range: i32,
            pub soaking_power: i32,
            pub splash_bombs: i32,
        }
    }
    pub mod hero_service {
        use crate::{
            hero::{hero_entity::HeroEntity, hero_profile::HeroProfile, hero_view::HeroView},
            reader::Reader,
        };
        use std::collections::HashMap;
        pub struct HeroService {
            owner_id: i32,
            entities: HashMap<i32, HeroEntity>,
            profiles: HashMap<i32, HeroProfile>,
        }
        impl HeroService {
            pub fn my_list(&self) -> impl Iterator<Item = &HeroEntity> {
                self.entities.values().filter(|&x| x.is_owner)
            }
            #[doc = " Возвращает врагов, которые находятся в пределах `range` от конкретного героя."]
            pub fn nearby_enemies<'a>(
                &'a self,
                hero: &'a HeroEntity,
                range: i32,
            ) -> impl Iterator<Item = (&'a HeroEntity, i32)> + 'a {
                self.entities
                    .values()
                    .filter(|e| !e.is_owner)
                    .filter_map(move |enemy| {
                        let dist = hero.position.m_dist(&enemy.position);
                        if dist <= range {
                            Some((enemy, dist))
                        } else {
                            None
                        }
                    })
            }
            pub fn enemy_list(&self) -> impl Iterator<Item = &HeroEntity> {
                self.entities.values().filter(|&x| !x.is_owner)
            }
            pub fn entities_list(&self) -> impl Iterator<Item = &HeroEntity> {
                self.entities.values()
            }
            pub fn profile_list(&self) -> Vec<&HeroProfile> {
                self.profiles.values().collect()
            }
            pub fn update_profile(&mut self, entity: HeroProfile) {
                self.profiles.insert(entity.agent_id, entity);
            }
            pub fn update(&mut self, entity: HeroEntity) {
                self.entities.insert(entity.agent_id, entity);
            }
            pub fn get_view(&self, agent_id: i32) -> Option<HeroView> {
                let entity = self.entities.get(&agent_id)?;
                let profile = self.profiles.get(&agent_id)?;
                Some(HeroView {
                    metadata: profile,
                    entity,
                })
            }
            pub fn read_profile<R: Reader>(&mut self, reader: &mut R) {
                let profiles = reader.read_profiles(self.owner_id);
                for p in profiles {
                    self.profiles.insert(p.agent_id, p);
                }
            }
            pub fn read_entity<R: Reader>(&mut self, reader: &mut R) {
                let entities = reader.read_entities(&self.profiles);
                self.entities.clear();
                for e in entities {
                    self.entities.insert(e.agent_id, e);
                }
            }
            pub fn new(id: i32) -> Self {
                Self {
                    owner_id: id,
                    entities: HashMap::new(),
                    profiles: HashMap::new(),
                }
            }
        }
    }
    pub mod hero_view {
        use crate::hero::{hero_entity::HeroEntity, hero_profile::HeroProfile};
        #[derive(Debug)]
        pub struct HeroView<'a> {
            pub metadata: &'a HeroProfile,
            pub entity: &'a HeroEntity,
        }
        impl<'a> HeroView<'a> {
            pub fn new(metadata: &'a HeroProfile, entity: &'a HeroEntity) -> Self {
                Self { metadata, entity }
            }
        }
    }
}
pub mod map_state {
    use crate::{position::Position, reader::Reader};
    use std::fmt::{self, Display};
    #[derive(Debug, Clone, Copy)]
    pub struct Tile {
        pub position: Position,
        pub tile_type: i32,
        pub entity_id: i32,
    }
    impl Tile {
        pub fn get_cover_int(&self) -> i32 {
            if self.tile_type != 0 {
                self.tile_type + 1
            } else {
                1
            }
        }
        pub fn get_cover_value(&self) -> f32 {
            match self.tile_type {
                1 => 0.5,
                2 => 0.7,
                _ => 0.0,
            }
        }
        pub fn is_occupied(&self) -> bool {
            self.entity_id != -1
        }
        #[inline]
        pub fn is_cover(&self) -> bool {
            return self.tile_type == 1 || self.tile_type == 2;
        }
    }
    impl Display for Tile {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Tile({}, {}) type={} entity={}",
                self.position.x, self.position.y, self.tile_type, self.entity_id
            )
        }
    }
    #[derive(Debug, Default, Clone, Copy)]
    pub struct TileScore {
        pub danger: i32,
        pub safety: i32,
        pub position: i32,
    }
    #[derive(Debug)]
    pub struct MapState {
        pub height: usize,
        pub width: usize,
        pub tiles: Vec<Tile>,
        pub scoring: Vec<TileScore>,
    }
    impl MapState {
        pub fn neighbors(&self, pos: &Position) -> impl Iterator<Item = &Tile> {
            pos.neighbors(self.width, self.height)
                .into_iter()
                .filter_map(move |p| self.get_tile(p.x, p.y))
        }
        pub fn get_sizes(&self) -> (usize, usize) {
            (self.width, self.height)
        }
        fn find_nearest_tile(&self, position: &Position, t_type: i32) -> Option<&Tile> {
            self.tiles
                .iter()
                .filter(|h| h.tile_type == t_type)
                .min_by_key(|h| h.position.m_dist(position))
        }
        #[inline]
        pub fn in_bounds(&self, x: usize, y: usize) -> bool {
            x >= 0 && y >= 0 && x < self.width && y < self.height
        }
        pub fn is_in_map(&self, pos: &Position) -> bool {
            self.in_bounds(pos.x, pos.y)
        }
        pub fn from_input<R: Reader>(reader: &mut R) -> Self {
            reader.read_map()
        }
        pub fn print(&self) {
            for y in 0..self.height {
                for x in 0..self.width {
                    let idx = y * self.width + x;
                    let tile = &self.tiles[idx];
                    eprint!("{}", tile.tile_type);
                }
                eprintln!();
            }
        }
        pub fn update_tile(&mut self, x: usize, y: usize, tile_type: i32, entity_id: i32) {
            if self.in_bounds(x, y) {
                let index = y * self.width + x;
                self.tiles[index].tile_type = tile_type;
                self.tiles[index].entity_id = entity_id;
            }
        }
        pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
            if self.in_bounds(x, y) {
                let index = y * self.width + x;
                self.tiles.get(index)
            } else {
                None
            }
        }
        pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
            if self.in_bounds(x, y) {
                let index = y * self.width + x;
                self.tiles.get_mut(index)
            } else {
                None
            }
        }
        pub fn from_str(s: &str) -> Self {
            let lines: Vec<&str> = s.lines().collect();
            let height = lines.len();
            let width = lines[0].len();
            let mut tiles = Vec::with_capacity(height * width);
            for (y, line) in lines.iter().enumerate() {
                for (x, ch) in line.chars().enumerate() {
                    let val = ch.to_digit(10).unwrap() as i32;
                    tiles.push(Tile {
                        position: Position { x: x, y: y },
                        tile_type: val,
                        entity_id: -1,
                    });
                }
            }
            Self {
                height,
                width,
                tiles,
                scoring: vec![],
            }
        }
    }
}
mod position {
    use std::{fmt::Display, ops::Add};
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Position {
        pub x: usize,
        pub y: usize,
    }
    impl Add for Position {
        type Output = Position;
        fn add(self, other: Position) -> Position {
            Position {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    impl Add<(usize, usize)> for Position {
        type Output = Position;
        fn add(self, other: (usize, usize)) -> Position {
            Position {
                x: self.x + other.0,
                y: self.y + other.1,
            }
        }
    }
    impl Position {
        pub const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        pub fn neighbors(&self, width: usize, height: usize) -> Vec<Position> {
            let x = self.x;
            let y = self.y;
            Self::DIRECTIONS
                .iter()
                .filter_map(|(dx, dy)| {
                    let nx = if *dx >= 0 {
                        x.checked_add(*dx as usize)
                    } else {
                        x.checked_sub((-dx) as usize)
                    };
                    let ny = if *dy >= 0 {
                        y.checked_add(*dy as usize)
                    } else {
                        y.checked_sub((-dy) as usize)
                    };
                    match (nx, ny) {
                        (Some(nx), Some(ny)) if nx < width && ny < height => {
                            Some(Position { x: nx, y: ny })
                        }
                        _ => None,
                    }
                })
                .collect()
        }
        pub fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }
        pub fn m_dist(&self, other: &Position) -> i32 {
            (self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()
        }
        pub fn dir(&self, other: &Position) -> (i32, i32) {
            (
                other.x as i32 - self.x as i32,
                other.y as i32 - self.y as i32,
            )
        }
        pub fn in_radius(&self, other: &Position, radius: usize) -> bool {
            let dx = (self.x as isize - other.x as isize).abs();
            let dy = (self.y as isize - other.y as isize).abs();
            dx as usize <= radius && dy as usize <= radius
        }
        pub fn direction(lhs: &Position, rhs: &Position) -> Position {
            Position::new(rhs.x - lhs.x, rhs.y - lhs.y)
        }
        pub fn is_linear(lhs: &Position, rhs: &Position) -> bool {
            lhs.x == rhs.x || lhs.y == rhs.y
        }
    }
    impl Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.x, self.y)
        }
    }
}
pub mod reader {
    use crate::{
        hero::{hero_cmd::HeroCommand, hero_entity::HeroEntity, hero_profile::HeroProfile},
        map_state::MapState,
    };
    use std::collections::HashMap;
    pub fn read_value<T: std::str::FromStr>() -> T {
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).unwrap();
        input_line.trim().parse::<T>().ok().unwrap()
    }
    pub trait Reader {
        fn read_i32(&mut self) -> i32;
        fn new() -> Self;
        fn get_count(&mut self) -> usize;
        fn step(&mut self, cmd: &HeroCommand) -> Result<(), Box<dyn std::error::Error>>;
        fn read_map(&mut self) -> MapState;
        fn read_profiles(&mut self, owner_id: i32) -> Vec<HeroProfile>;
        fn read_entities(&mut self, profiles: &HashMap<i32, HeroProfile>) -> Vec<HeroEntity>;
    }
}
pub mod sim_reader {
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
    use std::{collections::HashMap, fs, io};
    pub struct SimReader {
        values: Vec<i32>,
        profiles: Vec<HeroProfile>,
        entities: Vec<HeroEntity>,
        read_profiles_once: bool,
        map: MapState,
        cursor: usize,
    }
    impl SimReader {
        fn apply_hero_commands(
            &mut self,
            cmd: &HeroCommand,
        ) -> Result<(), Box<dyn std::error::Error>> {
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
        fn move_entity(
            &mut self,
            idx: usize,
            pos: Position,
        ) -> Result<(), Box<dyn std::error::Error>> {
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
                read_profiles_once: false,
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
}
pub mod strategy {
    use crate::{
        context::GameContext,
        hero::hero_cmd::{HeroAction, HeroCommand},
        position::Position,
        utils::{cover::is_hero_icopued, targeting::find_bomb_target},
    };
    pub trait Strategy {
        fn execute(&mut self, ctx: &GameContext, owns: usize) -> Vec<HeroCommand>;
    }
    pub struct SaveStrategy {
        pub cursor: usize,
    }
    impl SaveStrategy {
        pub const WAYPOINTS: [Position; 4] = [
            Position { x: 5, y: 2 },
            Position { x: 5, y: 9 },
            Position { x: 11, y: 9 },
            Position { x: 11, y: 2 },
        ];
        pub fn new() -> Self {
            return SaveStrategy { cursor: 0 };
        }
    }
    impl Strategy for SaveStrategy {
        fn execute(&mut self, ctx: &GameContext, owns: usize) -> Vec<HeroCommand> {
            let mut commands = vec![];
            for hero in ctx.hero_service.my_list() {
                let mut cmd: Vec<HeroAction> = vec![];
                if is_hero_icopued(ctx, &hero.position) {
                    let target = find_bomb_target(ctx, &hero.position);
                    if let Some(t) = target {
                        cmd.push(HeroAction::Throw(t));
                    }
                } else {
                    let mut hero_clone = hero.clone();
                    if !hero.position.eq(&SaveStrategy::WAYPOINTS[self.cursor]) {
                        hero_clone.position = SaveStrategy::WAYPOINTS[self.cursor];
                        cmd.push(HeroAction::Move(hero_clone.position));
                    } else {
                        self.cursor += 1;
                        let target = find_bomb_target(ctx, &hero_clone.position);
                        if let Some(t) = target {
                            cmd.push(HeroAction::Throw(t));
                        } else {
                            cmd.push(HeroAction::Wait);
                        }
                    }
                }
                commands.push(HeroCommand(hero.agent_id, cmd));
            }
            return commands;
        }
    }
    pub struct GuideStrategy;
    impl Strategy for GuideStrategy {
        fn execute(&mut self, ctx: &GameContext, owns: usize) -> Vec<HeroCommand> {
            todo!()
        }
    }
}
pub mod utils {
    pub mod cover {
        use crate::{context::GameContext, hero::hero_entity::HeroEntity, position::Position};
        pub fn find_cover_tile<'a>(
            ctx: &'a GameContext<'a>,
            hero: &'a HeroEntity,
        ) -> Option<Position> {
            let mut nearby_covers: Vec<_> = ctx
                .map_state
                .tiles
                .iter()
                .filter_map(|tile| {
                    let dist = tile.position.m_dist(&hero.position);
                    (tile.is_cover()).then_some((dist, tile))
                })
                .collect();
            nearby_covers.sort_by_key(|(dist, tile)| *dist / tile.get_cover_int());
            for (_, tile) in &nearby_covers {
                let mut dx = 0;
                let mut dy = 0;
                for (enemy, _) in ctx.hero_service.nearby_enemies(hero, 5) {
                    let (_dx, _dy) = tile.position.dir(&enemy.position);
                    dx += _dx;
                    dy += _dy;
                }
                let mut position_clone = tile.position.clone();
                if dx > 0 {
                    position_clone.x -= 1;
                } else {
                    position_clone.x += 1;
                }
                return Some(position_clone);
            }
            None
        }
        pub fn is_covered_hero(ctx: &GameContext, hero_position: &Position) -> bool {
            ctx.map_state
                .neighbors(hero_position)
                .any(|tile| tile.is_cover())
        }
        pub fn get_hero_cover_quality<'a>(
            ctx: &'a GameContext<'a>,
            hero_position: &'a Position,
        ) -> i32 {
            let mut value = 0;
            for tile in ctx.map_state.neighbors(hero_position) {
                if !tile.is_cover() && !Position::is_linear(&tile.position, hero_position) {
                    continue;
                }
                value += tile.get_cover_int();
            }
            return value;
        }
        pub fn is_hero_icopued(ctx: &GameContext, hero_position: &Position) -> bool {
            let mut value = 0;
            for tile in ctx.map_state.neighbors(hero_position) {
                if tile.tile_type != 3 {
                    continue;
                }
                value += tile.get_cover_int();
            }
            value > 0
        }
    }
    pub mod scoring {
        use crate::context::GameContext;
        pub fn score_defense(ctx: &GameContext) {}
        pub fn score_dangerous(ctx: &GameContext) {}
        pub fn score_shooter(ctx: &GameContext) {}
        pub fn score_bomber(ctx: &GameContext) {}
    }
    pub mod targeting {
        use crate::{
            context::GameContext, hero::hero_entity::HeroEntity, position::Position,
            utils::cover::get_hero_cover_quality,
        };
        pub fn nearest_enemy<'a>(ctx: &GameContext) -> i32 {
            0
        }
        pub fn k_closest_enemies<'a>(ctx: &GameContext) -> Vec<i32> {
            vec![]
        }
        pub fn find_save_bomb_position(
            ctx: &GameContext,
            position: &Position,
        ) -> Option<(Position, Position)> {
            for nbh in ctx.map_state.neighbors(position) {
                if let Some(t) = find_bomb_target(ctx, &nbh.position) {
                    return Some((nbh.position.clone(), t.clone()));
                }
            }
            None
        }
        pub fn find_bomb_target<'a>(ctx: &GameContext, position: &Position) -> Option<Position> {
            let enemies: Vec<_> = ctx
                .hero_service
                .enemy_list()
                .filter(|x| x.position.m_dist(&position) <= 4)
                .collect();
            let width = ctx.map_state.width;
            let height = ctx.map_state.height;
            let mut score_map = vec![vec![0u8; width]; height];
            let radius = 2;
            for e in &enemies {
                let min_x = e.position.x.saturating_sub(radius as usize);
                let max_x = (e.position.x + radius as usize).min(width - 1);
                let min_y = e.position.y.saturating_sub(radius as usize);
                let max_y = (e.position.y + radius as usize).min(height - 1);
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        score_map[y][x] += 1;
                    }
                }
            }
            let own_heroes: Vec<_> = ctx.hero_service.my_list().collect();
            let mut best_pos = None;
            let mut max_score = 0;
            for y in 0..height {
                for x in 0..width {
                    if score_map[y][x] > max_score {
                        let p: Position = Position { x, y };
                        let mut can_bombed = true;
                        for o_hero in &own_heroes {
                            if o_hero.position.in_radius(&p, 2) {
                                can_bombed = false;
                            }
                        }
                        if !can_bombed {
                            continue;
                        }
                        max_score = score_map[y][x];
                        best_pos = Some(p);
                    }
                }
            }
            if let Some(p) = best_pos {
                eprintln!("BEST_BOMB_POSITION:{}", p);
            }
            best_pos
        }
        pub fn find_shoot_target<'a>(ctx: &GameContext, hero: &HeroEntity) -> Option<i32> {
            let mut enemies = ctx
                .hero_service
                .nearby_enemies(hero, 6)
                .map(|(enemy, _)| enemy)
                .collect::<Vec<_>>();
            enemies.sort_by_key(|enemy| get_hero_cover_quality(ctx, &enemy.position));
            for en in enemies {
                return Some(en.agent_id);
            }
            return None;
        }
    }
}
use crate::cg_reader::CGReader;
use crate::context::GameContext;
use crate::hero::hero_service::HeroService;
use crate::map_state::MapState;
use crate::reader::Reader;
use crate::strategy::{SaveStrategy, Strategy};
#[doc = "\n * Win the water fight by controlling the most territory, or out-soak your opponent!\n *"]
fn main() {
    let mut strat = SaveStrategy::new();
    let mut reader = CGReader::new();
    let id = reader.read_i32();
    let mut hero_service = HeroService::new(id);
    hero_service.read_profile(&mut reader);
    let mut map_state = MapState::from_input(&mut reader);
    loop {
        hero_service.read_entity(&mut reader);
        hero_service.entities_list().for_each(|&x| {
            map_state.update_tile(
                x.position.x as usize,
                x.position.y as usize,
                if x.is_owner { 4 } else { 3 },
                x.agent_id,
            )
        });
        map_state.print();
        let context = GameContext::new(&hero_service, &map_state);
        let my_agent_count = reader.get_count();
        let actions = strat.execute(&context, my_agent_count);
        for i in &actions {
            match reader.step(i) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }
        }
    }
}

