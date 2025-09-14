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
