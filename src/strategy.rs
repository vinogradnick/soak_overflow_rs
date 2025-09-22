use crate::data::{
    game_context::GameContext,
    hero::{HeroAction, HeroCommand},
};

pub struct Strategy;

impl Strategy {
    pub fn do_action(ctx: &GameContext) -> Vec<HeroCommand> {
        let mut commands = vec![];

        for hero in ctx.hero_store.owns() {
            let mut cmd = HeroCommand(hero.agent_id, vec![]);

            if cmd.1.len() == 0 {
                cmd.1.push(HeroAction::Wait);
            }
            commands.push(cmd);
        }

        commands
    }
}
