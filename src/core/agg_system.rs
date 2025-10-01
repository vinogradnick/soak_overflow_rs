use crate::{
    core::{ai_system::AiSystem, cover_system::CoverSystem},
    data::{game_context::GameContext, hero::HeroAction},
};

pub struct AggSystem {
    ai: AiSystem,
    cover: CoverSystem,
}

impl AggSystem {
    pub fn new() -> AggSystem {
        AggSystem {
            ai: AiSystem::new(),
            cover: CoverSystem::new(),
        }
    }

    pub fn process(&mut self, ctx: &GameContext) -> Vec<HeroAction> {
        return self.ai.process(ctx);
    }
}

impl Default for AggSystem {
    fn default() -> Self {
        Self::new()
    }
}
