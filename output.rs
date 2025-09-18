use crate::{
    data::{
        context::GameContext,
        map_state::{MapState, Occupant, TileType},
    },
    hero::hero_service::HeroService,
    io::{cg_reader::CGReader, reader::Reader},
    systems::strategy::{SaveStrategy, Strategy},
};
pub mod data {
    pub mod context {
        use crate::{data::map_state::MapState, hero::hero_service::HeroService};
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
    pub mod map_state {
        use crate::{data::position::Position, io::reader::Reader};
        use std::fmt::{self, Display};
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u8)]
        pub enum TileType {
            Empty = 0,
            HighWall = 2,
            LowWall = 1,
        }
        impl TileType {
            pub fn parse_static(value: i32) -> Result<TileType, ()> {
                match value {
                    0 => Ok(TileType::Empty),
                    2 => Ok(TileType::HighWall),
                    1 => Ok(TileType::LowWall),
                    _ => Err(()),
                }
            }
        }
        impl From<TileType> for i32 {
            fn from(value: TileType) -> Self {
                value as i32
            }
        }
        impl fmt::Display for TileType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = match self {
                    TileType::Empty => "0",
                    TileType::HighWall => "2",
                    TileType::LowWall => "1",
                };
                write!(f, "{}", s)
            }
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Occupant {
            None,
            OwnerHero(i32),
            EnemyHero(i32),
        }
        impl Display for Occupant {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Occupant::EnemyHero(v) => write!(f, "EnemyHero({})", v),
                    Occupant::OwnerHero(v) => write!(f, "OwnerHero({})", v),
                    _ => write!(f, ""),
                }
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct Tile {
            pub position: Position,
            pub tile_type: TileType,
            pub occupant: Occupant,
        }
        impl Tile {
            #[inline]
            pub fn is_walkable(&self) -> bool {
                !self.is_cover() && !self.is_occupied()
            }
            pub fn is_occupied(&self) -> bool {
                self.occupant != Occupant::None
            }
            #[inline]
            pub fn is_cover(&self) -> bool {
                return self.tile_type != TileType::Empty;
            }
        }
        impl Display for Tile {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Tile({}, {}) type={} entity={}",
                    self.position.x, self.position.y, self.tile_type, self.occupant
                )
            }
        }
        #[derive(Debug, Default)]
        pub struct MapState {
            pub height: usize,
            pub width: usize,
            pub tiles: Vec<Tile>,
        }
        impl MapState {
            pub fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Self {
                Self {
                    height,
                    width,
                    tiles,
                }
            }
            pub fn neighbors_range(
                &self,
                pos: &Position,
                range: usize,
            ) -> impl Iterator<Item = &Tile> {
                pos.neighbors_range(self.width, self.height, range)
                    .into_iter()
                    .filter_map(|p| self.get_tile(p.x, p.y))
            }
            pub fn neighbors(&self, pos: &Position) -> impl Iterator<Item = &Tile> {
                pos.neighbors(self.width, self.height)
                    .into_iter()
                    .filter_map(move |p| self.get_tile(p.x, p.y))
            }
            pub fn get_sizes(&self) -> (usize, usize) {
                (self.width, self.height)
            }
            #[inline]
            pub fn in_bounds(&self, x: usize, y: usize) -> bool {
                x < self.width && y < self.height
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
            pub fn update_tile(
                &mut self,
                x: usize,
                y: usize,
                tile_type: TileType,
                occupant: Occupant,
            ) {
                if self.in_bounds(x, y) {
                    let index = y * self.width + x;
                    self.tiles[index].tile_type = tile_type;
                    self.tiles[index].occupant = occupant;
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
                        let tile = TileType::parse_static(val).unwrap_or(TileType::Empty);
                        tiles.push(Tile {
                            position: Position { x: x, y: y },
                            tile_type: tile,
                            occupant: Occupant::None,
                        });
                    }
                }
                MapState::new(width, height, tiles)
            }
        }
    }
    pub mod position {
        use std::{fmt::Display, ops::Add};
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        impl From<(i32, i32)> for Position {
            fn from(value: (i32, i32)) -> Self {
                Position {
                    x: value.0 as usize,
                    y: value.1 as usize,
                }
            }
        }
        impl Position {
            pub const WAYPOINTS: [(i32, i32); 8] = Position::generate_directions();
            const fn generate_directions() -> [(i32, i32); 8] {
                let mut dirs = [(0, 0); 8];
                let mut i = 0;
                let mut dx = -1;
                while dx <= 1 {
                    let mut dy = -1;
                    while dy <= 1 {
                        if dx != 0 || dy != 0 {
                            dirs[i] = (dx, dy);
                            i += 1;
                        }
                        dy += 1;
                    }
                    dx += 1;
                }
                dirs
            }
            pub const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            pub fn neighbors_range(
                &self,
                width: usize,
                height: usize,
                range: usize,
            ) -> Vec<Position> {
                let mut result = Vec::new();
                let x = self.x as isize;
                let y = self.y as isize;
                let r = range as isize;
                for dy in -r..=r {
                    for dx in -r..=r {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                            result.push(Position {
                                x: nx as usize,
                                y: ny as usize,
                            });
                        }
                    }
                }
                result
            }
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
            pub fn new_tuple((x, y): (i32, i32)) -> Self {
                Self {
                    x: x as usize,
                    y: y as usize,
                }
            }
            #[doc = " дистанция используемая для бомбы потому что диагонали утываются"]
            pub fn multi_distance(&self, other: &Position) -> usize {
                (self.x as isize - other.x as isize)
                    .abs()
                    .max((self.y as isize - other.y as isize).abs()) as usize
            }
            pub fn new(x: usize, y: usize) -> Self {
                Self { x, y }
            }
            pub fn dist_raw(&self, x: usize, y: usize) -> i32 {
                (self.x as i32 - x as i32).abs() + (self.y as i32 - y as i32).abs()
            }
            pub fn dist(&self, other: &Position) -> i32 {
                (self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()
            }
            #[doc = " `include_diagonal = true` — проверять 8 направлений, иначе только 4"]
            pub fn is_neighbor(&self, other: &Position, include_diagonal: bool) -> bool {
                let dx = (self.x as isize - other.x as isize).abs();
                let dy = (self.y as isize - other.y as isize).abs();
                if dx == 0 && dy == 0 {
                    return false;
                }
                if include_diagonal {
                    dx <= 1 && dy <= 1
                } else {
                    dx + dy == 1
                }
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
}
pub mod hero {
    pub mod hero_cmd {
        use crate::data::position::Position;
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
        use crate::data::position::Position;
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
            data::position::Position,
            hero::{hero_entity::HeroEntity, hero_profile::HeroProfile, hero_view::HeroView},
            io::reader::Reader,
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
            pub fn get_profile(&self, id: i32) -> Option<&HeroProfile> {
                self.profiles.get(&id)
            }
            pub fn get_entity(&self, id: i32) -> Option<&HeroEntity> {
                self.entities.get(&id)
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
                        let dist = hero.position.dist(&enemy.position);
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
            pub fn get_view<'a>(&'a self, agent_id: i32) -> Option<HeroView<'a>> {
                let entity = self.entities.get(&agent_id)?;
                let profile = self.profiles.get(&agent_id)?;
                Some(HeroView::new(profile, entity))
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
pub mod io {
    pub mod cg_reader {
        use crate::{
            data::{
                map_state::{MapState, Occupant, Tile, TileType},
                position::Position,
            },
            hero::{hero_cmd::HeroCommand, hero_entity::HeroEntity, hero_profile::HeroProfile},
            io::reader::{read_value, Reader},
        };
        use std::{collections::HashMap, io};
        pub struct CGReader {
            verbose: bool,
        }
        impl Reader for CGReader {
            fn read_i32(&mut self) -> i32 {
                read_value::<i32>()
            }
            fn new(verbose: bool) -> Self {
                return Self { verbose };
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
                MapState::new(width, height, tiles)
            }
            fn read_profiles(&mut self, owner_id: i32) -> Vec<HeroProfile> {
                let mut s = String::new();
                io::stdin().read_line(&mut s).unwrap();
                let agent_data_count: i32 = s.trim().parse().unwrap();
                eprintln!("{}", agent_data_count);
                let mut profiles = Vec::new();
                for _ in 0..agent_data_count {
                    s.clear();
                    io::stdin().read_line(&mut s).unwrap();
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
                profiles
            }
            fn read_entities(&mut self, profiles: &HashMap<i32, HeroProfile>) -> Vec<HeroEntity> {
                let mut s = String::new();
                io::stdin().read_line(&mut s).unwrap();
                let agent_count: i32 = s.trim().parse().unwrap();
                eprintln!("{}", agent_count);
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
    pub mod reader {
        use crate::{
            data::map_state::MapState,
            hero::{hero_cmd::HeroCommand, hero_entity::HeroEntity, hero_profile::HeroProfile},
        };
        use std::collections::HashMap;
        pub fn read_value<T: std::str::FromStr>() -> T {
            let mut input_line = String::new();
            std::io::stdin().read_line(&mut input_line).unwrap();
            input_line.trim().parse::<T>().ok().unwrap()
        }
        pub trait Reader {
            fn read_i32(&mut self) -> i32;
            fn new(verbose: bool) -> Self;
            fn get_count(&mut self) -> usize;
            fn step(&mut self, cmd: &HeroCommand) -> Result<(), Box<dyn std::error::Error>>;
            fn read_map(&mut self) -> MapState;
            fn read_profiles(&mut self, owner_id: i32) -> Vec<HeroProfile>;
            fn read_entities(&mut self, profiles: &HashMap<i32, HeroProfile>) -> Vec<HeroEntity>;
        }
    }
    pub mod sim_reader {
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
        use std::{
            collections::HashMap,
            fs::{self},
        };
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
                        tile.occupant = Occupant::OwnerHero(entity.agent_id);
                        entity.position = pos;
                    } else {
                        return Err(Box::from("Cannot move entity"));
                    }
                } else {
                    eprintln!("[DEBUG]: Invalid position {}", pos);
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
            fn get_count(&mut self) -> usize {
                self.entities.iter().filter(|x| x.is_owner).count()
            }
            fn step(&mut self, cmd: &HeroCommand) -> Result<(), Box<dyn std::error::Error>> {
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
                    raw_data: fs::read_to_string("./mapper.txt")
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
    }
}
pub mod systems {
    pub mod strategy {
        use crate::{
            data::context::GameContext,
            hero::hero_cmd::{HeroAction, HeroCommand},
        };
        pub trait Strategy {
            fn execute(&mut self, ctx: &GameContext, owns: usize) -> Vec<HeroCommand>;
        }
        pub struct SaveStrategy;
        impl SaveStrategy {
            pub fn new() -> Self {
                return SaveStrategy {};
            }
        }
        impl Strategy for SaveStrategy {
            fn execute(&mut self, ctx: &GameContext, _owns: usize) -> Vec<HeroCommand> {
                let mut commands = vec![];
                for hero in ctx.hero_service.my_list() {
                    let mut cmd: Vec<HeroAction> = vec![];
                    let target = crate::utils::bomb::find_bombing_position(ctx, &hero);
                    if let Some((moving, bomber)) = target {
                        cmd.push(HeroAction::Move(moving));
                        cmd.push(HeroAction::Throw(bomber));
                    }
                    if cmd.len() == 0 {
                        cmd.push(HeroAction::Wait);
                    }
                    commands.push(HeroCommand(hero.agent_id, cmd));
                }
                return commands;
            }
        }
    }
    pub mod scoring {
        use crate::{data::context::GameContext, hero::hero_entity::HeroEntity};
        pub fn is_surrounded(ctx: &GameContext, hero: &HeroEntity) -> bool {
            let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            let pos = hero.position;
            for (dx, dy) in dirs {
                let nx = pos.x as i32 + dx;
                let ny = pos.y as i32 + dy;
                if ctx.map_state.in_bounds(nx as usize, ny as usize) {
                    if let Some(tile) = ctx.map_state.get_tile(nx as usize, ny as usize) {
                        if !tile.is_occupied() && !tile.is_cover() {
                            return false;
                        }
                    }
                }
            }
            true
        }
    }
}
pub mod utils {
    pub mod bomb {
        use crate::{
            data::{context::GameContext, position::Position},
            hero::hero_entity::HeroEntity,
            systems::scoring::is_surrounded,
            utils::{self, pathfinder},
        };
        const RADIUS: i32 = 2;
        fn find_enemy_cluster(ctx: &GameContext) -> Option<Position> {
            let mut best_tile = None;
            let mut max_count = 0;
            for y in 0..ctx.map_state.height as i32 {
                for x in 0..ctx.map_state.width as i32 {
                    let tile_pos = Position::new_tuple((x, y));
                    let count = ctx
                        .hero_service
                        .enemy_list()
                        .filter(|en| {
                            let dist = en.position.dist(&tile_pos);
                            dist > 0 && dist <= RADIUS
                        })
                        .count();
                    if count > max_count {
                        max_count = count;
                        best_tile = Some(tile_pos);
                    }
                }
            }
            best_tile
        }
        pub fn find_bombing_position<'a>(
            ctx: &'a GameContext,
            hero: &'a HeroEntity,
        ) -> Option<(Position, Position)> {
            for tile in ctx.map_state.neighbors(&hero.position) {
                if tile.is_occupied() {
                    return find_save_bombing_position(ctx, hero);
                }
            }
            for tile in &ctx.map_state.tiles {
                if tile.is_cover()
                    || tile.is_occupied()
                    || !utils::pathfinder::can_reach(ctx, &hero.position, &tile.position)
                {
                    continue;
                }
                if !ctx.hero_service.enemy_list().any(|en| {
                    let p = en.position.dist(&tile.position);
                    return p > 0 && p < 3;
                }) {
                    continue;
                }
                let p = find_enemy_cluster(ctx);
                if let Some(t) = p {
                    if t.dist(&tile.position) <= 3 {
                        return Some((tile.position, t));
                    }
                }
            }
            None
        }
        pub fn bomb_evaluate<'a>(
            ctx: &'a GameContext,
            pos: &Position,
            hero_position: &Position,
        ) -> i32 {
            let mut count = 0;
            for enemy in ctx.hero_service.enemy_list() {
                if enemy.position.multi_distance(pos) < 2 {
                    count += 1;
                }
            }
            return count;
        }
        pub fn find_save_bombing_position<'a>(
            ctx: &'a GameContext,
            hero: &'a HeroEntity,
        ) -> Option<(Position, Position)> {
            let prepared = ctx
                .map_state
                .tiles
                .iter()
                .filter(|t| {
                    let pdist = t.position.multi_distance(&hero.position);
                    !t.is_cover() && pdist > 1 && pdist < 3
                })
                .collect::<Vec<_>>();
            let maxed = prepared
                .iter()
                .max_by_key(|t| bomb_evaluate(ctx, &t.position, &hero.position));
            if let Some(v) = maxed {
                return Some((hero.position.clone(), v.position.clone()));
            }
            None
        }
    }
    pub mod cover {
        use crate::{
            data::context::GameContext, data::position::Position, hero::hero_entity::HeroEntity,
        };
        pub fn find_cover_tile<'a>(
            ctx: &'a GameContext<'a>,
            hero: &'a HeroEntity,
        ) -> Option<Position> {
            let mut nearby_covers: Vec<_> = ctx
                .map_state
                .tiles
                .iter()
                .filter_map(|tile| {
                    let dist = tile.position.dist(&hero.position);
                    (tile.is_cover()).then_some((dist, tile))
                })
                .collect();
            nearby_covers.sort_by_key(|(dist, tile)| *dist / tile.tile_type as i32);
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
    }
    pub mod pathfinder {
        use crate::{data::context::GameContext, data::position::Position};
        pub fn can_reach(ctx: &GameContext, start: &Position, goal: &Position) -> bool {
            use std::collections::VecDeque;
            let mut visited = std::collections::HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(*start);
            while let Some(pos) = queue.pop_front() {
                if pos == *goal {
                    return true;
                }
                if !visited.insert(pos) {
                    continue;
                }
                for next in ctx.map_state.neighbors(&pos) {
                    if let Some(tile) = ctx.map_state.get_tile(next.position.x, next.position.y) {
                        if tile.is_walkable() {
                            queue.push_back(next.position);
                        }
                    }
                }
            }
            false
        }
    }
}
#[doc = "\n * Win the water fight by controlling the most territory, or out-soak your opponent!\n *"]
fn main() {
    let mut strat = SaveStrategy::new();
    let mut reader = CGReader::new(true);
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
                TileType::Empty,
                if x.is_owner {
                    Occupant::OwnerHero(x.agent_id)
                } else {
                    Occupant::EnemyHero(x.agent_id)
                },
            )
        });
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

