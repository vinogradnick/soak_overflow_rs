use crate::{
    data::{
        game_context::GameContext,
        hero::{HeroAction, HeroCommand},
    },
    systems::{actor::select_action, cover::CoverQuery, judge::JudgeService},
};

pub struct Strategy;

impl Strategy {
    pub fn do_action(ctx: &GameContext) -> Vec<HeroCommand> {
        let mut commands = vec![];
        let cover = CoverQuery::evaluate(ctx);
        let score = JudgeService::evaluate_context(ctx);

        eprintln!("OwnScore:{} EnemyScore:{}", score.0, score.1);

        for hero in ctx.hero_store.owns() {
            let mut cmd = HeroCommand(hero.agent_id, vec![]);
            cmd.1 = select_action(ctx, hero, &cover, score);

            if cmd.1.len() == 0 {
                cmd.1.push(HeroAction::Wait);
            }
            commands.push(cmd);
        }

        commands
    }
}
