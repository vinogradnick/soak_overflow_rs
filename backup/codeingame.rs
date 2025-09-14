pub mod cg_reader;
pub mod context;
pub mod hero;
pub mod map_state;
mod position;
pub mod reader;
pub mod sim_reader;
pub mod strategy;
pub mod utils;

use crate::cg_reader::CGReader;
use crate::context::GameContext;
use crate::hero::hero_service::HeroService;
use crate::map_state::MapState;

use crate::reader::Reader;
use crate::strategy::{SaveStrategy, Strategy};

/**
 * Win the water fight by controlling the most territory, or out-soak your opponent!
 **/
fn main() {
    let strat = SaveStrategy;
    let mut reader = CGReader::new();
    let id = reader.read_i32();
    let mut hero_service = HeroService::new(id);
    hero_service.read_profile(&mut reader);
    let mut map_state = MapState::from_input(&mut reader);

    // game loop
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
        map_state.print();

        let context = GameContext::new(&hero_service, &map_state);
        let my_agent_count = reader.get_count(); // Number of alive agents controlled by you
        let actions = strat.execute(&context, my_agent_count);
        for i in &actions {
            match reader.step(i) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }
        }
    }
}
