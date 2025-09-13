use crate::{
    context::GameContext,
    hero_cmd::{HeroAction, HeroCmd},
    hero_profile::HeroEntity,
    pathfinder::Pathfinder,
    position::Position,
};

pub trait Strategy {
    fn execute(&self, ctx: &GameContext, owns: usize) -> Vec<String>;
}

pub struct SaveStrategy;

impl SaveStrategy {
    pub fn get_direction<'a>(
        enemies: impl Iterator<Item = &'a HeroEntity>,
        wall: &Position,
    ) -> Position {
        for &e in enemies {
            if e.position.m_dist(wall) <= 5 {
                let p = &e.position;
                return p.dir(wall);
            }
        }

        Position::new(0, 0)
    }
}

impl Strategy for SaveStrategy {
    fn execute(&self, ctx: &GameContext, owns: usize) -> Vec<String> {
        let mut commands = vec![];

        for hero in ctx.hero_service.entities_list().filter(|x| x.is_owner) {
            let agg_path = Pathfinder::closet_cover_point(ctx, hero);

            commands.push(
                HeroCmd::new(hero.agent_id)
                    .with(HeroAction::Move(agg_path.0))
                    .with(HeroAction::Shoot(agg_path.1))
                    .to_string(),
            );
        }

        return commands;
    }
}
