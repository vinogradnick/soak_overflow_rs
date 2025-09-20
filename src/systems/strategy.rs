use crate::{
    data::context::GameContext,
    hero::hero_cmd::{HeroAction, HeroCommand},
};

pub trait Strategy {
    fn execute(&mut self, ctx: &GameContext, owns: usize) -> Vec<HeroCommand>;
}

pub struct SaveStrategy;

impl SaveStrategy {
    pub fn new() -> Self {
        return SaveStrategy {};
    }
}

impl Strategy for SaveStrategy {
    fn execute(&mut self, ctx: &GameContext, _owns: usize) -> Vec<HeroCommand> {
        let mut commands = vec![];

        for hero in ctx.hero_service.my_list() {
            let mut cmd: Vec<HeroAction> = vec![];

            let target = crate::utils::bomb::find_bombing_position(ctx, &hero);
            if let Some((moving, bomber)) = target {
                if hero.position != moving {
                    cmd.push(HeroAction::Move(moving));
                }

                cmd.push(HeroAction::Throw(bomber));
            }

            if cmd.len() == 0 {
                cmd.push(HeroAction::Wait);
            }

            commands.push(HeroCommand(hero.agent_id, cmd));
        }

        return commands;
    }
}
