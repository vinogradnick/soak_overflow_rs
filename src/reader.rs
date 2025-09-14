use std::collections::HashMap;

use crate::{
    hero::{hero_cmd::HeroCommand, hero_entity::HeroEntity, hero_profile::HeroProfile},
    map_state::MapState,
};

pub fn read_value<T: std::str::FromStr>() -> T {
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().parse::<T>().ok().unwrap()
}

// ---------- Reader Trait ----------
pub trait Reader {
    fn read_i32(&mut self) -> i32;
    fn new() -> Self;
    fn get_count(&mut self) -> usize;
    fn step(&mut self, cmd: &HeroCommand) -> Result<(), Box<dyn std::error::Error>>;
    fn read_map(&mut self) -> MapState;
    fn read_profiles(&mut self, owner_id: i32) -> Vec<HeroProfile>;
    fn read_entities(&mut self, profiles: &HashMap<i32, HeroProfile>) -> Vec<HeroEntity>;
}
