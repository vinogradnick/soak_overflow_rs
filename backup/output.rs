pub mod context {
    use crate::{hero_profile::HeroService, map_state::MapState};
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
pub mod hero_cmd {
    use crate::position::Position;
    #[derive(Debug, Clone)]
    pub enum HeroAction {
        Move(Position),
        Shoot(i32),
        Wait,
    }
    #[derive(Debug, Clone)]
    pub struct HeroCmd {
        pub hero_id: i32,
        pub actions: Vec<HeroAction>,
    }
    impl HeroCmd {
        pub fn new(id: i32) -> Self {
            HeroCmd {
                hero_id: id,
                actions: vec![],
            }
        }
        pub fn to_string(&self) -> String {
            let items = self
                .actions
                .iter()
                .map(|cmd| match &cmd {
                    HeroAction::Move(pos) => format!("MOVE {}", pos),
                    HeroAction::Shoot(target_id) => format!("SHOOT {}", target_id),
                    HeroAction::Wait => format!("WAIT",),
                })
                .collect::<Vec<_>>()
                .join("; ");
            return format!("{}; {}", self.hero_id, items);
        }
        pub fn with(&mut self, action: HeroAction) -> &mut Self {
            self.actions.push(action);
            return self;
        }
    }
}
pub mod hero_profile {
    use crate::{position::Position, reader::Reader};
    use std::collections::HashMap;
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
    #[derive(Debug)]
    pub struct HeroView<'a> {
        metadata: &'a HeroProfile,
        entity: &'a HeroEntity,
    }
    impl<'a> HeroView<'a> {
        pub fn new(metadata: &'a HeroProfile, entity: &'a HeroEntity) -> Self {
            Self { metadata, entity }
        }
    }
    pub struct HeroService {
        owner_id: i32,
        entities: HashMap<i32, HeroEntity>,
        profiles: HashMap<i32, HeroProfile>,
    }
    impl HeroService {
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
pub mod map_state {
    use crate::{position::Position, reader::Reader};
    #[derive(Debug, Clone, Copy)]
    pub struct Tile {
        pub position: Position,
        pub tile_type: i32,
        pub entity_id: i32,
    }
    impl Tile {
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
    }
    #[derive(Debug)]
    pub struct MapState {
        pub height: usize,
        pub width: usize,
        pub tiles: Vec<Tile>,
    }
    impl MapState {
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
        pub fn to_index(width: usize, x: usize, y: usize) -> usize {
            y * width + x
        }
        #[inline]
        pub fn in_bounds(&self, x: usize, y: usize) -> bool {
            x < self.width && y < self.height
        }
        pub fn is_in_map(pos: &Position, sizes: (usize, usize)) -> bool {
            pos.x < 0 && pos.x < sizes.0 as i32 && pos.y < 0 && pos.y < sizes.1 as i32
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
                        position: Position {
                            x: x as i32,
                            y: y as i32,
                        },
                        tile_type: val,
                        entity_id: -1,
                    });
                }
            }
            Self {
                height,
                width,
                tiles,
            }
        }
    }
}
pub mod pathfinder {
    use crate::{
        context::GameContext,
        hero_profile::HeroEntity,
        map_state::{MapState, Tile},
        position::Position,
    };
    use std::collections::BinaryHeap;
    pub struct Pathfinder;
    impl Pathfinder {
        #[inline]
        pub fn to_index(width: usize, pos: &Position) -> i32 {
            pos.x * width as i32 + pos.y
        }
        #[inline]
        pub fn to_index_raw(width: usize, x: i32, y: i32) -> i32 {
            x * width as i32 + y
        }
        const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        pub fn nearest_enemy<'a>(
            pos: &Position,
            all_heroes: impl Iterator<Item = &'a HeroEntity>,
        ) -> Option<&'a HeroEntity> {
            all_heroes
                .filter(|h| !h.is_owner)
                .min_by_key(|h| h.position.m_dist(pos))
        }
        #[doc = "  ищет таргет до цели перпендикулярно"]
        pub fn find_shoot_target<'a>(
            pos: &Position,
            all_heroes: impl Iterator<Item = &'a HeroEntity>,
        ) -> Option<&'a HeroEntity> {
            all_heroes
                .filter(|h| !h.is_owner)
                .filter(|hero| hero.position.x.eq(&pos.x) || hero.position.y.eq(&pos.y))
                .min_by_key(|hero| hero.position.m_dist(pos))
        }
        pub fn find_shoot_target_by<'a>(
            pos: &Position,
            all_heroes: impl Iterator<Item = &'a HeroEntity>,
        ) -> Option<&'a HeroEntity> {
            all_heroes
                .filter(|h| !h.is_owner)
                .filter(|hero| hero.position.x.eq(&pos.x) || hero.position.y.eq(&pos.y))
                .min_by(|l, r| {
                    let dl = l.position.m_dist(pos);
                    let dr = r.position.m_dist(pos);
                    dl.cmp(&dr).then_with(|| l.wetness.cmp(&r.wetness))
                })
        }
        pub fn get_cover_against<'a>(
            hero: &HeroEntity,
            enemy: &HeroEntity,
            tiles: &[Tile],
            size: (usize, usize),
        ) -> Option<&'a Tile> {
            for (dx, dy) in Pathfinder::DIRECTIONS {
                let target_x = hero.position.x + dx;
                let target_y = hero.position.y + dy;
                let pos: Position = Position::new(target_x, target_y);
                if !MapState::is_in_map(&pos, size) {
                    continue;
                }
                let index = MapState::to_index(size.0, target_x as usize, target_y as usize);
                if tiles[index].get_cover_value() > 0.0 {}
                let direction = hero.position.dir(&enemy.position);
            }
            None
        }
        pub fn k_closest_enemies<'a>(
            hero: &HeroEntity,
            enemies: impl Iterator<Item = &'a HeroEntity>,
            k: usize,
            distance: usize,
        ) -> Vec<&'a HeroEntity> {
            let mut vec: Vec<&HeroEntity> = enemies.collect();
            vec.sort_by_key(|e| e.position.m_dist(&hero.position));
            vec.truncate(k);
            vec
        }
        pub fn check_enemy_cover<'a>(
            enemy: &HeroEntity,
            tiles: &[Tile],
            size: (usize, usize),
        ) -> Option<&'a Tile> {
            for (x_pos, y_pos) in Pathfinder::DIRECTIONS {
                let dx = enemy.position.x + x_pos;
                let dy = enemy.position.y + y_pos;
                if dx >= 0 && dx < size.0 as i32 && dy >= 0 && dy < size.1 as i32 {
                    continue;
                }
                let index = Pathfinder::to_index_raw(size.0, dx, dy);
                let tile = tiles[index as usize];
            }
            None
        }
        pub fn closet_cover_point<'a>(ctx: &GameContext, hero: &HeroEntity) -> (Position, i32) {
            let nearest_tile = ctx
                .map_state
                .tiles
                .iter()
                .filter(|&x| x.tile_type == 2)
                .min_by_key(|x| x.position.m_dist(&hero.position));
            let near_enemies = ctx
                .hero_service
                .enemy_list()
                .filter(|x| x.position.m_dist(&hero.position) <= 5)
                .collect::<Vec<_>>();
            let mut target_tile = nearest_tile.unwrap().position.clone();
            let mut t_dir = 1;
            for en in near_enemies {
                let ta = en.position.dir(&hero.position);
                if ta.x > 0 {
                    t_dir = 1;
                } else {
                    t_dir = -1;
                }
            }
            target_tile.x += t_dir;
            return (target_tile, 0);
        }
    }
}
mod position {
    use std::fmt::Display;
    #[derive(Debug, Clone, Copy)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }
    impl Position {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
        pub fn m_dist(&self, other: &Position) -> i32 {
            (self.x - other.x).abs() + (self.y - other.y).abs()
        }
        pub fn dir(&self, other: &Position) -> Position {
            Position::new(other.x - self.x, other.y - self.y)
        }
        pub fn direction(lhs: &Position, rhs: &Position) -> Position {
            Position::new(rhs.x - lhs.x, rhs.y - lhs.y)
        }
        pub fn is_valid(&self, size: (usize, usize)) -> bool {
            self.x <= 0 && self.x < size.0 as i32 && self.y <= 0 && self.y < size.1 as i32
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
        hero_cmd::{HeroAction, HeroCmd},
        hero_profile::{HeroEntity, HeroProfile},
        map_state::{MapState, Tile},
        position::Position,
    };
    use std::{collections::HashMap, io};
    fn read_value<T: std::str::FromStr>() -> T {
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).unwrap();
        input_line.trim().parse::<T>().ok().unwrap()
    }
    pub trait Reader {
        fn read_i32(&mut self) -> i32;
        fn new() -> Self;
        fn get_count(&mut self) -> usize;
        fn step(&mut self, cmd: &str) -> Result<(), Box<dyn std::error::Error>>;
        fn read_map(&mut self) -> MapState;
        fn read_profiles(&mut self, owner_id: i32) -> Vec<HeroProfile>;
        fn read_entities(&mut self, profiles: &HashMap<i32, HeroProfile>) -> Vec<HeroEntity>;
    }
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
        fn step(&mut self, cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
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
            for _ in 0..height {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let inputs = input_line.trim().split_whitespace().collect::<Vec<_>>();
                for j in 0..width {
                    let x: i32 = inputs[3 * j].parse().unwrap();
                    let y: i32 = inputs[3 * j + 1].parse().unwrap();
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
                let x: i32 = inputs[1].parse().unwrap();
                let y: i32 = inputs[2].parse().unwrap();
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
            let agent_id: i32 = commands[0].parse().unwrap();
            let mut hero_cmd = HeroCmd::new(agent_id);
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
            self.apply_hero_commands(hero_cmd)?;
            Ok(())
        }
        fn read_map(&mut self) -> MapState {
            let map_container = r#"0000200010000
0200002002010
0000200010000
0102002000020
0000100020000"#;
            self.map = MapState::from_str(map_container);
            MapState::from_str(map_container)
        }
        fn read_profiles(&mut self, _owner_id: i32) -> Vec<HeroProfile> {
            if self.read_profiles_once {
                vec![]
            } else {
                self.read_profiles_once = true;
                self.profiles.clone()
            }
        }
        fn read_entities(&mut self, _profiles: &HashMap<i32, HeroProfile>) -> Vec<HeroEntity> {
            if self.entities.len() > 0 {
                return self.entities.clone();
            }
            self.entities.push(HeroEntity {
                position: Position { x: 0, y: 2 },
                is_owner: true,
                agent_id: 1,
                cooldown: 1,
                splash_bombs: 2,
                wetness: 3,
            });
            self.entities.push(HeroEntity {
                position: Position { x: 12, y: 2 },
                is_owner: true,
                agent_id: 2,
                cooldown: 1,
                splash_bombs: 2,
                wetness: 3,
            });
            self.entities.push(HeroEntity {
                position: Position { x: 8, y: 1 },
                is_owner: false,
                agent_id: 3,
                cooldown: 1,
                splash_bombs: 2,
                wetness: 3,
            });
            self.entities.push(HeroEntity {
                position: Position { x: 8, y: 3 },
                is_owner: false,
                agent_id: 4,
                cooldown: 1,
                splash_bombs: 2,
                wetness: 3,
            });
            self.entities.push(HeroEntity {
                position: Position { x: 4, y: 1 },
                is_owner: false,
                agent_id: 5,
                cooldown: 1,
                splash_bombs: 2,
                wetness: 3,
            });
            self.entities.push(HeroEntity {
                position: Position { x: 4, y: 3 },
                is_owner: false,
                agent_id: 6,
                cooldown: 1,
                splash_bombs: 2,
                wetness: 3,
            });
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
}
pub mod strategy {
    use crate::{
        context::GameContext,
        hero_cmd::{HeroAction, HeroCmd},
        hero_profile::HeroEntity,
        pathfinder::Pathfinder,
        position::Position,
    };
    pub trait Strategy {
        fn execute(&self, ctx: &GameContext, owns: usize) -> Vec<String>;
    }
    pub struct SaveStrategy;
    impl SaveStrategy {
        pub fn get_direction<'a>(
            enemies: impl Iterator<Item = &'a HeroEntity>,
            wall: &Position,
        ) -> Position {
            for &e in enemies {
                if e.position.m_dist(wall) <= 5 {
                    let p = &e.position;
                    return p.dir(wall);
                }
            }
            Position::new(0, 0)
        }
    }
    impl Strategy for SaveStrategy {
        fn execute(&self, ctx: &GameContext, owns: usize) -> Vec<String> {
            let mut commands = vec![];
            for hero in ctx.hero_service.entities_list().filter(|x| x.is_owner) {
                let agg_path = Pathfinder::closet_cover_point(ctx, hero);
                commands.push(
                    HeroCmd::new(hero.agent_id)
                        .with(HeroAction::Move(agg_path.0))
                        .with(HeroAction::Shoot(agg_path.1))
                        .to_string(),
                );
            }
            return commands;
        }
    }
}
use crate::context::GameContext;
use crate::hero_profile::HeroService;
use crate::map_state::MapState;
use crate::reader::{CGReader, Reader, SimReader};
use crate::strategy::{SaveStrategy, Strategy};
#[doc = "\n * Win the water fight by controlling the most territory, or out-soak your opponent!\n *"]
fn main() {
    let strat = SaveStrategy;
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
