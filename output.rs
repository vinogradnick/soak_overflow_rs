pub mod data {
    pub mod game_context {
        use crate::data::{hero::HeroStore, tilemap::TileMap};
        #[derive(Debug, Clone)]
        pub struct GameContext {
            pub player_id: i32,
            pub tilemap: TileMap,
            pub hero_store: HeroStore,
        }
        impl GameContext {
            pub fn new() -> Self {
                Self {
                    player_id: 0,
                    tilemap: TileMap::new(0, 0),
                    hero_store: HeroStore::new(),
                }
            }
        }
    }
    pub mod hero {
        use crate::data::position::Position;
        use std::{collections::HashMap, fmt};
        #[derive(Debug, Clone, Default)]
        pub struct HeroStore {
            pub owner_id: usize,
            pub heroes: HashMap<usize, Hero>,
        }
        #[derive(Debug, Clone, Copy)]
        pub struct Hero {
            pub agent_id: usize,
            pub is_owner: bool,
            pub shoot_cooldown: i32,
            pub optimal_range: i32,
            pub soaking_power: i32,
            pub splash_bombs: i32,
            pub position: Position,
            pub cooldown: i32,
            pub wetness: i32,
            pub initialized: bool,
        }
        impl Hero {
            pub fn new(
                agent_id: usize,
                player: bool,
                shoot_cooldown: i32,
                optimal_range: i32,
                soaking_power: i32,
                splash_bombs: i32,
                position: Position,
            ) -> Self {
                Hero {
                    agent_id,
                    is_owner: player,
                    shoot_cooldown,
                    optimal_range,
                    soaking_power,
                    splash_bombs,
                    position: position,
                    cooldown: 0,
                    wetness: 0,
                    initialized: false,
                }
            }
        }
        impl HeroStore {
            pub fn new() -> Self {
                Self::default()
            }
            pub fn update_hero(&mut self, idx: usize, hero: &Hero) {
                self.heroes.insert(idx, hero.clone());
            }
            pub fn owns<'a>(&'a self) -> impl Iterator<Item = &'a Hero> {
                self.heroes.values().filter(move |x| x.is_owner)
            }
            pub fn enemies<'a>(&'a self) -> impl Iterator<Item = &'a Hero> {
                self.heroes.values().filter(move |x| !x.is_owner)
            }
        }
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
        pub struct HeroCommand(pub usize, pub Vec<HeroAction>);
        impl fmt::Display for HeroCommand {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let actions_str: Vec<String> = self.1.iter().map(|a| a.to_string()).collect();
                write!(f, "{}; {}", self.0, actions_str.join("; "))
            }
        }
    }
    pub mod position {
        use std::fmt::{Debug, Display};
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Position {
            pub x: usize,
            pub y: usize,
        }
        impl Position {
            pub const WAYPOINTS: [(i32, i32); 8] = generate_directions();
            pub const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        }
        impl Display for Position {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", self.x, self.y)
            }
        }
        impl Default for Position {
            fn default() -> Self {
                Self {
                    x: Default::default(),
                    y: Default::default(),
                }
            }
        }
        pub const fn generate_directions() -> [(i32, i32); 8] {
            [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
        }
        impl Position {
            pub fn distance_8x(&self, other: &Position) -> i32 {
                let dx = (self.x as i32 - other.x as i32).abs();
                let dy = (self.y as i32 - other.y as i32).abs();
                dx.max(dy)
            }
            pub fn distance(&self, other: &Position) -> i32 {
                let x1 = self.x as i32;
                let y1 = self.y as i32;
                let x2 = other.x as i32;
                let y2 = other.y as i32;
                (x2 - x1).abs() + (y2 - y1).abs()
            }
        }
        pub fn is_between<T>(value: T, min: T, max: T) -> bool
        where
            T: PartialEq + PartialOrd,
        {
            value >= min && value <= max
        }
    }
    pub mod query {
        pub struct Query<T> {
            pub len: usize,
            pub idx: usize,
            pub(crate) components: T,
        }
        pub struct PlayerQueryView<'a> {
            pub players: &'a [i32],
            pub positions_x: &'a [usize],
            pub positions_y: &'a [usize],
        }
        impl<'a> Iterator for Query<PlayerQueryView<'a>> {
            type Item = (i32, (usize, usize));
            fn next(&mut self) -> Option<Self::Item> {
                if self.idx >= self.len {
                    return None;
                }
                let result = (
                    self.components.players[self.idx],
                    (
                        self.components.positions_x[self.idx],
                        self.components.positions_y[self.idx],
                    ),
                );
                self.idx += 1;
                Some(result)
            }
        }
    }
    pub mod tile {
        use crate::data::position::Position;
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum TileType {
            HighWall = 0,
            LowWall = 1,
            Empty = 2,
        }
        impl Default for TileType {
            fn default() -> Self {
                TileType::Empty
            }
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Occupant {
            Enemy(usize),
            Owner(usize),
            Nil,
        }
        impl Default for Occupant {
            fn default() -> Self {
                Self::Nil
            }
        }
        pub struct TileMeta {}
        #[derive(Debug, Clone, Copy, Default)]
        pub struct TileView {
            pub position: Position,
            pub occupant: Occupant,
            pub tile_type: TileType,
        }
        impl TileView {
            #[inline]
            pub fn is_cover(&self) -> bool {
                self.tile_type != TileType::Empty
            }
            #[inline]
            pub fn is_free(&self) -> bool {
                self.occupant == Occupant::Nil && self.tile_type == TileType::Empty
            }
            #[inline]
            pub fn is_ocuped(&self) -> bool {
                self.occupant != Occupant::Nil
            }
            pub fn is_enemy_occuped(&self) -> bool {
                match self.occupant {
                    Occupant::Enemy(_) => true,
                    _ => false,
                }
            }
            pub fn is_own_occuped(&self) -> bool {
                match self.occupant {
                    Occupant::Owner(_) => true,
                    _ => false,
                }
            }
        }
    }
    pub mod tilemap {
        use crate::data::{position::Position, tile::TileView};
        #[derive(Debug, Clone)]
        pub struct TileMap {
            height: usize,
            width: usize,
            pub tiles: Vec<TileView>,
        }
        impl TileMap {
            pub fn new(w: usize, h: usize) -> Self {
                TileMap {
                    height: h,
                    width: w,
                    tiles: vec![TileView::default(); w * h],
                }
            }
            pub fn tiles_by_dist<'a>(
                &'a self,
                other: &'a Position,
                dist: i32,
            ) -> impl Iterator<Item = &'a TileView> {
                self.tiles
                    .iter()
                    .filter(move |tile| tile.position.distance(other) >= dist)
            }
            pub fn find_by_id(&self, id: usize) -> Option<&TileView> {
                self.tiles.get(id)
            }
            pub fn get_tile<'a>(&'a self, position: &Position) -> Option<&'a TileView> {
                self.tiles.get(position.y * self.width + position.x)
            }
            pub fn get_tile_mut(&mut self, position: &Position) -> Option<&mut TileView> {
                self.tiles.get_mut(position.y * self.width + position.x)
            }
            pub fn out_of_bounds(&self, x: impl Into<i32>, y: impl Into<i32>) -> bool {
                let x = x.into();
                let y = y.into();
                x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32
            }
            pub fn get_height(&self) -> usize {
                self.height
            }
            pub fn get_width(&self) -> usize {
                self.width
            }
            pub fn neighbors(&self, from: &Position) -> Vec<Position> {
                let mut v = vec![];
                for (x, y) in Position::DIRECTIONS {
                    let dx = from.x as i32 + x;
                    let dy = from.y as i32 + y;
                    if self.out_of_bounds(dx, dy) {
                        continue;
                    }
                    v.push(Position {
                        x: dx as usize,
                        y: dy as usize,
                    });
                }
                v
            }
            pub fn neighbors_8x(&self, from: &Position) -> Vec<Position> {
                let mut v = vec![];
                for (x, y) in Position::WAYPOINTS {
                    let dx = from.x as i32 + x;
                    let dy = from.y as i32 + y;
                    if self.out_of_bounds(dx, dy) {
                        continue;
                    }
                    v.push(Position {
                        x: dx as usize,
                        y: dy as usize,
                    });
                }
                v
            }
        }
    }
    pub mod tilemap_iter {
        use crate::data::{position::Position, tile::TileView, tilemap::TileMap};
        pub struct Neighbors<'a> {
            map: &'a TileMap,
            pos: Position,
            diag: bool,
            idx: usize,
        }
        impl<'a> Neighbors<'a> {
            pub fn new(map: &'a TileMap, pos: Position, diag: bool) -> Self {
                Self {
                    map,
                    pos,
                    diag,
                    idx: 0,
                }
            }
        }
        impl<'a> Iterator for Neighbors<'a> {
            type Item = &'a TileView;
            fn next(&mut self) -> Option<Self::Item> {
                const DIRS_4: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                const DIRS_8: [(i32, i32); 8] = [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ];
                let dirs: &[(i32, i32)] = if self.diag { &DIRS_8 } else { &DIRS_4 };
                while self.idx < dirs.len() {
                    let (dx, dy) = dirs[self.idx];
                    self.idx += 1;
                    let x = self.pos.x as i32 + dx;
                    let y = self.pos.y as i32 + dy;
                    if x < 0
                        || y < 0
                        || x >= self.map.get_width() as i32
                        || y >= self.map.get_height() as i32
                    {
                        continue;
                    }
                    let id = (y as usize) * self.map.get_width() + (x as usize);
                    if let Some(tile) = self.map.find_by_id(id) {
                        return Some(tile);
                    }
                }
                None
            }
        }
        pub struct NeighborsRange<'a> {
            map: &'a TileMap,
            center: Position,
            diag: bool,
            range: usize,
            curr_y: isize,
            curr_x: isize,
        }
        impl<'a> NeighborsRange<'a> {
            pub fn new(map: &'a TileMap, pos: Position, diag: bool, range: usize) -> Self {
                let r = range as isize;
                Self {
                    map,
                    center: pos,
                    diag,
                    range,
                    curr_y: -r,
                    curr_x: -r,
                }
            }
        }
        impl<'a> Iterator for NeighborsRange<'a> {
            type Item = &'a TileView;
            fn next(&mut self) -> Option<Self::Item> {
                let r = self.range as isize;
                while self.curr_y <= r {
                    while self.curr_x <= r {
                        let dx = self.curr_x;
                        let dy = self.curr_y;
                        self.curr_x += 1;
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if !self.diag && dx.abs() + dy.abs() > 1 {
                            continue;
                        }
                        let nx = self.center.x as isize + dx;
                        let ny = self.center.y as isize + dy;
                        if nx >= 0
                            && ny >= 0
                            && nx < self.map.get_width() as isize
                            && ny < self.map.get_height() as isize
                        {
                            let index = (ny as usize) * self.map.get_width() + (nx as usize);
                            return Some(&self.map.tiles[index]);
                        }
                    }
                    self.curr_x = -r;
                    self.curr_y += 1;
                }
                None
            }
        }
    }
}
pub mod logger {
    use std::time::{SystemTime, UNIX_EPOCH};
    pub const VERBOSE_LOGGS: bool = true;
    pub fn log<T>(value: &T, func: &str)
    where
        T: std::fmt::Debug,
    {
        if !VERBOSE_LOGGS {
            return;
        }
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!(
            "[{}.{:03}] [{}] [{}] {:?}",
            now.as_secs(),
            now.subsec_millis(),
            "DEBUG",
            func,
            value
        );
    }
}
pub mod reader {
    use crate::{
        data::{
            game_context::GameContext,
            hero::{Hero, HeroAction, HeroCommand, HeroStore},
            position::Position,
            tile::{Occupant, TileType, TileView},
            tilemap::TileMap,
            tilemap_iter::Neighbors,
        },
        logger,
        systems::pathfinder,
    };
    use std::{fmt::Debug, io, str::FromStr};
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
                    let agent_count = read_number::<usize>();
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
                        let agent_count = read_number::<usize>();
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
            let width: usize = read_number_str(inputs[0]);
            let height: usize = read_number_str(inputs[1]);
            ctx.tilemap = TileMap::new(width, height);
            for i in 0..height as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                eprintln!("{}", &input_line);
                let inputs = input_line.split_whitespace().collect::<Vec<_>>();
                for j in 0..width as usize {
                    let x: usize = read_number_str(inputs[3 * j]);
                    let y: usize = read_number_str(inputs[3 * j + 1]);
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
            let agent_data_count = read_number::<usize>();
            ctx.hero_store = HeroStore::new();
            for i in 0..agent_data_count as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                eprintln!("{}", &input_line);
                let inputs = input_line
                    .split(" ")
                    .map(read_number_str::<i32>)
                    .collect::<Vec<_>>();
                let hero = Hero {
                    agent_id: inputs[0] as usize,
                    is_owner: ctx.player_id == inputs[1],
                    shoot_cooldown: inputs[2],
                    optimal_range: inputs[3],
                    soaking_power: inputs[4],
                    splash_bombs: inputs[5],
                    position: Position::default(),
                    cooldown: 0,
                    wetness: 0,
                    initialized: false,
                };
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
    fn simulator_action(ctx: &mut GameContext, actions: Vec<HeroCommand>) -> Result<(), String> {
        for act in actions {
            eprintln!("ACTION: {}", act);
            let id = act.0;
            let loc_actions = act.1;
            let hero_entity = ctx.hero_store.heroes.get(&id).cloned();
            if let Some(hero) = hero_entity {
                for action in loc_actions {
                    match action {
                        HeroAction::Move(position) => apply_move_action(ctx, &position, &hero)?,
                        HeroAction::Throw(position) => {
                            let hero_mut = ctx.hero_store.heroes.get_mut(&hero.agent_id).unwrap();
                            if hero_mut.splash_bombs < 1
                                || hero_mut.position.distance(&position) > 3
                            {
                                eprintln!(
                                    "OUT ACTION ->{}->{:?}",
                                    hero_mut.position.distance(&position),
                                    hero
                                );
                                return Ok(());
                            }
                            hero_mut.splash_bombs -= 1;
                            for tile in ctx
                                .tilemap
                                .tiles
                                .iter_mut()
                                .filter(|x| x.position.distance_8x(&position) <= 1)
                            {
                                match tile.occupant {
                                    Occupant::Enemy(id) => {
                                        match ctx.hero_store.heroes.remove(&id) {
                                            Some(res) => {
                                                eprintln!("Removed {:?}", res);
                                                tile.occupant = Occupant::Nil;
                                            }
                                            None => {}
                                        }
                                    }
                                    Occupant::Owner(id) => {
                                        match ctx.hero_store.heroes.remove(&id) {
                                            Some(res) => {
                                                eprintln!("Removed {:?}", res);
                                                tile.occupant = Occupant::Nil;
                                            }
                                            None => {}
                                        }
                                    }
                                    Occupant::Nil => continue,
                                };
                            }
                        }
                        HeroAction::Shoot(_) => todo!(),
                        HeroAction::Wait => {}
                    }
                }
            }
        }
        Ok(())
    }
    fn apply_move_action(
        ctx: &mut GameContext,
        position: &Position,
        hero: &Hero,
    ) -> Result<(), String> {
        if !ctx
            .tilemap
            .get_tile(&position)
            .is_some_and(|tile| tile.is_free())
        {
            return Ok(());
        }
        if position.distance(&hero.position) > 1 {
            let path = pathfinder::find_path(ctx, &hero.position, &position);
            if let Some(t) = path {
                for p in t {
                    if hero.position != p {
                        let mut hero_clone = hero.clone();
                        hero_clone.position = p;
                        ctx.hero_store.update_hero(hero.agent_id, &hero_clone);
                        return Ok(());
                    }
                }
            }
        } else {
            let mut hero_clone = hero.clone();
            hero_clone.position = position.clone();
            ctx.hero_store.update_hero(hero.agent_id, &hero_clone);
        };
        Ok(())
    }
}
pub mod strategy {
    use crate::{
        data::{
            game_context::GameContext,
            hero::{HeroAction, HeroCommand},
        },
        systems::bomber::find_bomb_all,
    };
    pub struct Strategy;
    impl Strategy {
        pub fn do_action(ctx: &GameContext) -> Vec<HeroCommand> {
            let mut commands = vec![];
            for hero in ctx.hero_store.owns() {
                let mut cmd = HeroCommand(hero.agent_id, vec![]);
                match find_bomb_all(ctx, hero) {
                    Some([movement, bomb]) => {
                        cmd.1 = vec![HeroAction::Move(movement), HeroAction::Throw(bomb)]
                    }
                    None => {}
                }
                if cmd.1.len() == 0 {
                    cmd.1.push(HeroAction::Wait);
                }
                commands.push(cmd);
            }
            commands
        }
    }
}
pub mod systems {
    pub mod history {
        use crate::data::game_context::GameContext;
        pub struct HistorySystem {
            data: Vec<GameContext>,
            cursor: usize,
            capacity: usize,
        }
        impl HistorySystem {
            pub fn new(capacity: usize) -> Self {
                Self {
                    data: Vec::with_capacity(capacity),
                    cursor: 0,
                    capacity,
                }
            }
            pub fn current(&self) -> Option<&GameContext> {
                self.data.get(self.cursor)
            }
            pub fn next(&mut self) -> Option<&GameContext> {
                if self.cursor + 1 < self.data.len() {
                    self.cursor += 1;
                }
                self.data.get(self.cursor)
            }
            pub fn prev(&mut self) -> Option<&GameContext> {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                self.data.get(self.cursor)
            }
            pub fn apply(&mut self, ctx: GameContext) {
                if self.cursor + 1 < self.data.len() {
                    self.data.truncate(self.cursor + 1);
                }
                self.data.push(ctx);
                if self.data.len() > self.capacity {
                    self.data.remove(0);
                }
                self.cursor = self.data.len() - 1;
            }
        }
    }
    pub mod pathfinder {
        use crate::data::{game_context::GameContext, position::Position, tilemap_iter::Neighbors};
        use std::collections::{HashMap, HashSet, VecDeque};
        pub fn can_reach(ctx: &GameContext, start: &Position, goal: &Position) -> bool {
            use std::collections::VecDeque;
            let mut visited = std::collections::HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(start.clone());
            while let Some(pos) = queue.pop_front() {
                if pos == goal.clone() {
                    return true;
                }
                if !visited.insert(pos) {
                    continue;
                }
                for next in Neighbors::new(&ctx.tilemap, pos, false) {
                    if ctx
                        .tilemap
                        .get_tile(&next.position)
                        .is_some_and(|x| x.is_free())
                    {
                        queue.push_back(next.position);
                    }
                }
            }
            false
        }
        pub fn find_path(
            ctx: &GameContext,
            start: &Position,
            goal: &Position,
        ) -> Option<Vec<Position>> {
            let mut visited = HashSet::new();
            let mut parents: HashMap<Position, Position> = HashMap::new();
            let mut queue = VecDeque::new();
            queue.push_back(*start);
            while let Some(pos) = queue.pop_front() {
                if pos == *goal {
                    let mut path = vec![pos];
                    let mut current = pos;
                    while let Some(parent) = parents.get(&current) {
                        path.push(*parent);
                        current = *parent;
                    }
                    path.reverse();
                    return Some(path);
                }
                if !visited.insert(pos) {
                    continue;
                }
                for next in Neighbors::new(&ctx.tilemap, pos, false) {
                    if ctx
                        .tilemap
                        .get_tile(&next.position)
                        .is_some_and(|x| x.is_free())
                        && !visited.contains(&next.position)
                    {
                        parents.insert(next.position, pos);
                        queue.push_back(next.position);
                    }
                }
            }
            None
        }
    }
    pub mod bomber {
        use crate::{
            data::{game_context::GameContext, hero::Hero, position::Position},
            systems::pathfinder,
        };
        pub fn find_bomb_target(ctx: &GameContext, hero: &Hero) -> Option<Position> {
            let mut candidates = vec![];
            for tile in &ctx.tilemap.tiles {
                if tile.is_cover() {
                    continue;
                }
                let mut score = 0;
                for near in ctx.tilemap.neighbors_8x(&tile.position) {
                    if let Some(t) = ctx.tilemap.get_tile(&near) {
                        match t.occupant {
                            crate::data::tile::Occupant::Enemy(_) => {
                                score += 1;
                            }
                            crate::data::tile::Occupant::Owner(_) => {
                                score -= 1000;
                            }
                            crate::data::tile::Occupant::Nil => {}
                        }
                    }
                }
                candidates.push((score, tile));
            }
            match candidates.iter().max_by_key(|(score, _)| score) {
                Some((score, tile)) => Some(tile.position),
                None => None,
            }
        }
        pub fn find_bomb_source(
            ctx: &GameContext,
            hero: &Hero,
            target: &Position,
        ) -> Option<Position> {
            for tile in &ctx.tilemap.tiles {
                if tile.is_cover() || tile.is_ocuped() {
                    continue;
                }
                let distance = tile.position.distance(target);
                if !pathfinder::can_reach(ctx, &hero.position, &tile.position) {
                    continue;
                }
                if distance < 1 || distance >= 4 {
                    continue;
                }
                return Some(tile.position);
            }
            None
        }
        pub fn count_adjacent_units(ctx: &GameContext, hero: &Hero) -> usize {
            let mut counter = 0;
            for tile in &ctx.tilemap.tiles {
                let distance = tile.position.distance_8x(&hero.position);
                if distance != 1 || !tile.is_enemy_occuped() {
                    continue;
                }
                counter += 1;
            }
            return counter;
        }
        pub fn occupantion_bombing(ctx: &GameContext, hero: &Hero) -> Option<[Position; 2]> {
            let mut candidates = vec![];
            for tile in &ctx.tilemap.tiles {
                if tile.is_cover()
                    || tile.position.distance(&hero.position) > 2
                    || tile.position == hero.position
                {
                    continue;
                }
                let mut score = 0;
                for near in ctx.tilemap.neighbors_8x(&tile.position) {
                    if let Some(t) = ctx.tilemap.get_tile(&near) {
                        match t.occupant {
                            crate::data::tile::Occupant::Enemy(_) => {
                                score += 1;
                            }
                            crate::data::tile::Occupant::Owner(_) => {
                                score -= 1000;
                            }
                            crate::data::tile::Occupant::Nil => {}
                        }
                    }
                }
                candidates.push((score, tile));
            }
            let item = candidates.iter().max_by_key(|(score, _)| score);
            match item {
                Some((score, tile_view)) => {
                    return Some([hero.position, tile_view.position]);
                }
                None => None,
            }
        }
        pub fn find_bomb_all(ctx: &GameContext, hero: &Hero) -> Option<[Position; 2]> {
            let closet_value = count_adjacent_units(ctx, hero);
            if closet_value > 1 {
                eprintln!("Hero[{0}]->{closet_value}", hero.agent_id);
                return occupantion_bombing(ctx, hero);
            }
            let target = find_bomb_target(ctx, hero);
            if let Some(target_value) = target {
                match find_bomb_source(ctx, hero, &target_value) {
                    Some(source_pos) => {
                        return Some([source_pos, target_value]);
                    }
                    None => return None,
                }
            }
            None
        }
    }
}
use crate::{data::game_context::GameContext, reader::Reader, strategy::Strategy};
fn main() {
    let mut ctx = GameContext::new();
    let io_reader = Reader::CodeingameReader;
    io_reader.read_id(&mut ctx);
    io_reader.read_profiles(&mut ctx);
    io_reader.read_tilemap(&mut ctx);
    loop {
        io_reader.read_entities(&mut ctx);
        let my_agent_count = io_reader.read_number(&mut ctx);
        let commands = Strategy::do_action(&ctx);
        match io_reader.receive_action(&mut ctx, commands) {
            Ok(_) => {}
            Err(data) => {
                eprintln!("{}", data);
            }
        }
    }
}

