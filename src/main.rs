pub mod cg_reader;
pub mod context;
pub mod hero;
pub mod map_state;
mod position;
pub mod reader;
pub mod sim_reader;
pub mod strategy;
pub mod utils;
pub mod viz;

use crate::context::GameContext;
use crate::hero::hero_service::HeroService;
use crate::map_state::MapState;

use crate::reader::Reader;
use crate::sim_reader::SimReader;
use crate::strategy::{SaveStrategy, Strategy};

use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let mut strat = SaveStrategy::new();
    let mut reader = SimReader::new();
    let id = reader.read_i32();
    let mut hero_service = HeroService::new(id);
    hero_service.read_profile(&mut reader);
    let mut map_state = MapState::from_input(&mut reader);

    let mut steps = 0;

    loop {
        hero_service.read_entity(&mut reader);
        hero_service.entities_list().for_each(|&x| {
            map_state.update_tile(
                x.position.x as usize,
                x.position.y as usize,
                if x.is_owner { 4 } else { 3 },
                x.agent_id,
            )
        });

        let my_agent_count = reader.get_count(); // Number of alive agents controlled by you

        let ctx: GameContext<'_> = GameContext::new(&hero_service, &map_state);

        clear_background(BLACK);

        viz::render::draw_map(&ctx);
        viz::render::draw_heroes(&ctx);

        let actions = strat.execute(&ctx, my_agent_count);

        if steps < 1 {
            for i in &actions {
                eprintln!("RAW ACTION:[{}]", &i);
                match reader.step(i) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }

            steps += 1;
        }

        viz::render::draw_actions(&actions);

        next_frame().await
    }
}
