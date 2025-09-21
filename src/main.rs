pub mod data;
pub mod reader;
pub mod strategy;
pub mod systems;
pub mod viz;
use crate::{
    data::game_context::GameContext, reader::Reader, strategy::Strategy,
    systems::history::HistorySystem, viz::render::render_context,
};
use macroquad::prelude::*;

/**
 * Win the water fight by controlling the most territory, or out-soak your opponent!
 **/

#[macroquad::main("MyGame")]
async fn main() {
    let mut history = HistorySystem::new();
    let mut ctx = GameContext::new();
    let io_reader = Reader::SimulatorReader(false);
    io_reader.read_id(&mut ctx);
    io_reader.read_profiles(&mut ctx);
    io_reader.read_tilemap(&mut ctx);

    let mut ticker = 0.0;

    // game loop
    loop {
        let dt = get_frame_time();

        ticker += dt;

        if ticker >= 1.0 {
            io_reader.read_entities(&mut ctx);
            let my_agent_count = io_reader.read_number(&mut ctx); // Number of alive agents controlled by you
            let commands = Strategy::do_action(&ctx, my_agent_count);
            io_reader.receive_action(&mut ctx, commands);
            history.apply(&ctx);
            ticker -= 1.0; // сбрасываем таймер, можно вычитать 1.0 чтобы не накапливался баг
        }

        render_context(&ctx).await;
    }
}
