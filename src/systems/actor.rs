use crate::{
    data::{
        game_context::GameContext,
        hero::{Hero, HeroAction, HeroCommand},
        position::Position,
    },
    systems::{cover::CoverQuery, judge::JudgeService, shooter::ShooterQuery},
};

pub struct Transitor {}

pub fn select_action(
    ctx: &GameContext,
    agent: &Hero,
    cover: &Vec<(Position, i32)>,
    prev_score: (i32, i32),
) -> Vec<HeroAction> {
    let mut actions = vec![];

    if !CoverQuery::is_covered_hero_position(ctx, agent) {
        let cover_position = cover.iter().find(|x| x.0.distance(&agent.position) < 4);
        dbg!("CoverQuery::is_covered_hero_position", cover_position);
        match cover_position {
            Some((position, _)) => {
                actions.push(HeroAction::Move(position.clone()));
            }
            None => {}
        }
    }

    if agent.optimal_range > 2 {
        let target = ShooterQuery::get_target(ctx, agent);

        match target {
            Some(enemy) => actions.push(HeroAction::Shoot(enemy.agent_id)),
            None => {}
        }
    }

    let changed = JudgeService::apply_action_sim(ctx, HeroCommand(agent.agent_id, actions.clone()));

    let diff = (prev_score.0 - changed.0, prev_score.1 - changed.1);

    eprintln!(
        "[{}]: Difference: OwnScore:{} EnemyScore:{} Actions:{:#?}",
        agent.agent_id, diff.0, diff.1, actions,
    );

    return actions;
}
