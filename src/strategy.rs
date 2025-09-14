use crate::{
    context::GameContext,
    hero::hero_cmd::{HeroAction, HeroCommand},
    position::Position,
    utils::{
        cover::is_hero_icopued,
        targeting::{find_all_map_bomb_position, find_bomb_target, find_safe_bomb_position},
    },
};

pub trait Strategy {
    fn execute(&mut self, ctx: &GameContext, owns: usize) -> Vec<HeroCommand>;
}

pub struct SaveStrategy {
    pub cursor: usize,
}

impl SaveStrategy {
    pub const WAYPOINTS: [Position; 4] = [
        Position { x: 5, y: 2 },
        Position { x: 5, y: 9 },
        Position { x: 11, y: 9 },
        Position { x: 11, y: 2 },
    ];

    pub fn new() -> Self {
        return SaveStrategy { cursor: 0 };
    }
}

impl Strategy for SaveStrategy {
    fn execute(&mut self, ctx: &GameContext, owns: usize) -> Vec<HeroCommand> {
        let mut commands = vec![];

        for hero in ctx.hero_service.my_list() {
            let mut cmd: Vec<HeroAction> = vec![];

            if is_hero_icopued(ctx, &hero.position) {
                let target = find_safe_bomb_position(ctx, &hero.position);
                if let Some((moving, bomber)) = target {
                    cmd.push(HeroAction::Move(moving));
                    cmd.push(HeroAction::Throw(bomber));
                }

                if cmd.len() == 0 {
                    cmd.push(HeroAction::Wait);
                }
            } else {
                find_all_map_bomb_position(ctx, &hero.position);

                // if let Some((moving, bomber)) = target {
                //     cmd.push(HeroAction::Move(moving));
                //     cmd.push(HeroAction::Throw(bomber));
                // }
            }

            // if !cover::is_covered_hero(ctx, &hero.position) {
            //     let cov = cover::find_cover_tile(ctx, &hero);

            //     match cov {
            //         Some(t) => {
            //             // cmd.push(HeroAction::Move(Position { x: 8, y: 6 }));
            //         }
            //         None => {
            //             eprintln!("cannot cover HeroID:{}", hero.agent_id);
            //         }
            //     }
            //     let target = targeting::find_shoot_target(ctx, &hero);

            //     // match target {
            //     //     Some(v) => {
            //     //         cmd.push(HeroAction::Shoot(v));
            //     //     }
            //     //     None => {
            //     //         eprintln!("cannot shoot HeroID:{}", hero.agent_id);
            //     //     }
            //     // }

            //     // let target = targeting::find_shoot_target(ctx, &hero);
            // }

            commands.push(HeroCommand(hero.agent_id, cmd));
        }

        return commands;
    }
}

pub struct GuideStrategy;

impl Strategy for GuideStrategy {
    fn execute(&mut self, ctx: &GameContext, owns: usize) -> Vec<HeroCommand> {
        todo!()
    }
}
