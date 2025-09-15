use std::collections::HashMap;

use crate::{
    hero::{hero_entity::HeroEntity, hero_profile::HeroProfile, hero_view::HeroView},
    position::Position,
    reader::Reader,
};

pub struct HeroService {
    owner_id: i32,
    entities: HashMap<i32, HeroEntity>,
    profiles: HashMap<i32, HeroProfile>,
}

impl HeroService {
    pub fn my_list(&self) -> impl Iterator<Item = &HeroEntity> {
        self.entities.values().filter(|&x| x.is_owner)
    }

    /// Возвращает врагов, которые находятся в пределах `range` от конкретного героя.
    pub fn nearby_enemies<'a>(
        &'a self,
        hero: &'a HeroEntity,
        range: i32,
    ) -> impl Iterator<Item = (&'a HeroEntity, i32)> + 'a {
        self.entities
            .values()
            .filter(|e| !e.is_owner) // только враги
            .filter_map(move |enemy| {
                let dist = hero.position.dist(&enemy.position);
                if dist <= range {
                    Some((enemy, dist))
                } else {
                    None
                }
            })
    }

    pub fn enemy_range(
        &self,
        position: &Position,
        range: &i32,
    ) -> impl Iterator<Item = &HeroEntity> {
        self.entities
            .values()
            .filter(|x| !x.is_owner && x.position.dist(position) <= *range)
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
