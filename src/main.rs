use crate::{
    data::{
        context::GameContext,
        map_state::{MapState, Occupant, TileType},
    },
    hero::hero_service::HeroService,
    io::{reader::Reader, sim_reader::SimReader},
    systems::strategy::{SaveStrategy, Strategy},
};
use macroquad::prelude::*;

pub mod data;
pub mod hero;
pub mod io;
pub mod systems;
pub mod utils;
pub mod viz;
#[macroquad::main("MyGame")]
async fn main() {
    let mut strat = SaveStrategy::new();
    let mut reader = SimReader::new(true);
    let id = reader.read_i32();
    let mut hero_service = HeroService::new(id);
    hero_service.read_profile(&mut reader);
    let mut map_state = MapState::from_input(&mut reader);

    loop {
        hero_service.read_entity(&mut reader);
        hero_service.entities_list().for_each(|&x| {
            map_state.update_tile(
                x.position.x as usize,
                x.position.y as usize,
                TileType::Empty,
                if x.is_owner {
                    Occupant::OwnerHero(x.agent_id)
                } else {
                    Occupant::EnemyHero(x.agent_id)
                },
            )
        });

        let my_agent_count = reader.get_count(); // Number of alive agents controlled by you

        let ctx: GameContext<'_> = GameContext::new(&hero_service, &map_state);

        clear_background(BLACK);

        viz::render::draw_map(&ctx);
        viz::render::draw_heroes(&ctx);

        if is_key_down(KeyCode::R) {
            let actions = strat.execute(&ctx, my_agent_count);

            for i in &actions {
                match reader.step(i) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }

            viz::render::draw_actions(&actions);
        }

        next_frame().await
    }
}
