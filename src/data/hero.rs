use std::{collections::HashMap, fmt};

use crate::data::position::Position;

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
    Shoot(usize), // agent_id цели
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
pub struct HeroCommand(pub usize, pub Vec<HeroAction>);

// Реализация Display для HeroCommand
impl fmt::Display for HeroCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let actions_str: Vec<String> = self.1.iter().map(|a| a.to_string()).collect();
        write!(f, "{}; {}", self.0, actions_str.join("; "))
    }
}
