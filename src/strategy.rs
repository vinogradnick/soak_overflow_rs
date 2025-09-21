use crate::data::{game_context::GameContext, hero::HeroCommand};

pub struct Strategy;

impl Strategy {
    pub fn do_action(ctx: &GameContext, count: usize) -> Vec<HeroCommand> {
        vec![]
    }
}
