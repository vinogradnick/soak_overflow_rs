use std::fmt;

use crate::data::{
    position::Position,
    query::{PlayerQueryView, Query},
};

#[derive(Debug, Clone, Default)]
pub struct HeroStore {
    pub player: Vec<i32>,
    pub shoot_cooldown: Vec<i32>,
    pub optimal_range: Vec<i32>,
    pub soaking_power: Vec<i32>,
    pub splash_bombs: Vec<i32>,
    pub positions_x: Vec<usize>,
    pub positions_y: Vec<usize>,
    pub cooldown: Vec<i32>,
    pub wetness: Vec<i32>,
    pub initialized: Vec<bool>,
}

#[derive(Debug, Clone, Copy)]
pub struct Hero {
    pub agent_id: i32,
    pub player: i32,
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
        agent_id: i32,
        player: i32,
        shoot_cooldown: i32,
        optimal_range: i32,
        soaking_power: i32,
        splash_bombs: i32,
        position: (usize, usize),
    ) -> Self {
        Hero {
            agent_id,
            player,
            shoot_cooldown,
            optimal_range,
            soaking_power,
            splash_bombs,
            position: position.into(),
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
    pub fn new_count(count: usize) -> Self {
        Self {
            player: vec![-1; count],
            shoot_cooldown: vec![-1; count],
            optimal_range: vec![-1; count],
            soaking_power: vec![-1; count],
            splash_bombs: vec![-1; count],
            positions_x: vec![0; count],
            positions_y: vec![0; count],
            cooldown: vec![-1; count],
            wetness: vec![-1; count],
            initialized: vec![false; count],
        }
    }

    pub fn length(&self) -> usize {
        self.initialized.len()
    }

    pub fn update_hero(&mut self, idx: usize, hero: &Hero) {
        if hero.player != -1 {
            self.player[idx] = hero.player;
        }
        if hero.shoot_cooldown != -1 {
            self.shoot_cooldown[idx] = hero.shoot_cooldown;
        }
        if hero.optimal_range != -1 {
            self.optimal_range[idx] = hero.optimal_range;
        }
        if hero.soaking_power != -1 {
            self.soaking_power[idx] = hero.soaking_power;
        }
        if hero.splash_bombs != -1 {
            self.splash_bombs[idx] = hero.splash_bombs;
        }
        if hero.cooldown != -1 {
            self.cooldown[idx] = hero.cooldown;
        }
        if hero.wetness != -1 {
            self.wetness[idx] = hero.wetness;
        }
        self.initialized[idx] = hero.initialized;
        // Позиции всегда обновляем, так как они usize
        self.positions_x[idx] = hero.position.x;
        self.positions_y[idx] = hero.position.y;
    }
}

// Итератор по героям
pub struct HeroStoreIter<'a> {
    store: &'a HeroStore,
    idx: usize,
}

impl<'a> Iterator for HeroStoreIter<'a> {
    type Item = Hero;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.store.player.len() {
            return None;
        }
        let hero = Hero {
            agent_id: self.idx as i32,
            player: self.store.player[self.idx],
            shoot_cooldown: self.store.shoot_cooldown[self.idx],
            optimal_range: self.store.optimal_range[self.idx],
            soaking_power: self.store.soaking_power[self.idx],
            splash_bombs: self.store.splash_bombs[self.idx],
            position: (
                self.store.positions_x[self.idx],
                self.store.positions_y[self.idx],
            )
                .into(),
            cooldown: self.store.cooldown[self.idx],
            wetness: self.store.wetness[self.idx],
            initialized: self.store.initialized[self.idx],
        };

        self.idx += 1;
        Some(hero)
    }
}

impl HeroStore {
    pub fn iter(&self) -> HeroStoreIter<'_> {
        HeroStoreIter {
            store: self,
            idx: 0,
        }
    }

    pub fn query_player_positions(&self) -> Query<PlayerQueryView<'_>> {
        Query {
            len: self.player.len(),
            idx: 0,
            components: PlayerQueryView {
                players: &self.player,
                positions_x: &self.positions_x,
                positions_y: &self.positions_y,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum HeroAction {
    Move(Position),
    Throw(Position),
    Shoot(i32), // agent_id цели
    Wait,
}

// Реализация Display для HeroAction
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

// Реализация Display для HeroCommand
impl fmt::Display for HeroCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let actions_str: Vec<String> = self.1.iter().map(|a| a.to_string()).collect();
        write!(f, "{}; {}", self.0, actions_str.join("; "))
    }
}
