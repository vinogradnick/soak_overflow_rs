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
