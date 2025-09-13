use std::collections::HashMap;

use crate::{position::Position, reader::Reader};

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
    pub fn my_list(&self) -> impl Iterator<Item = &HeroEntity> {
        self.entities.values().filter(|&x| x.is_owner)
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
