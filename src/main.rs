pub mod data;
pub mod logger;
pub mod reader;
pub mod simulator;
pub mod strategy;
pub mod systems;
pub mod viz;

use crate::{
    data::game_context::GameContext,
    reader::Reader,
    strategy::Strategy,
    viz::render::{draw_heroes, draw_map, render_context},
};
use macroquad::prelude::*;

/**
 * Win the water fight by controlling the most territory, or out-soak your opponent!
 **/

#[macroquad::main("MyGame")]
async fn main() {
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

        clear_background(BLACK);

        draw_map(&ctx);
        draw_heroes(&ctx);

        if is_key_down(KeyCode::R) {
            Strategy::do_action(&ctx);
        }

        if ticker >= 1.0 {
            io_reader.read_entities(&mut ctx);
            io_reader.read_number(&mut ctx); // Number of alive agents controlled by you

            let commands = Strategy::do_action(&ctx);
            match io_reader.receive_action(&mut ctx, commands) {
                Ok(_) => {}
                Err(data) => {
                    eprintln!("{}", data);
                }
            }

            ticker -= 1.0; // сбрасываем таймер, можно вычитать 1.0 чтобы не накапливался баг
        }

        render_context(&ctx);

        next_frame().await
    }
}
