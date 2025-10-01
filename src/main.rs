pub mod core;
pub mod data;
pub mod infra;
pub mod viz;

use crate::{
    core::agg_system::AggSystem,
    infra::{
        input_reader::{read_for_loop, read_for_loop_update, read_input},
        logger,
        position_utils::find_cover_position,
        simulator::simulator_action,
    },
    viz::render::{draw_heroes, draw_map, render_context},
};
use macroquad::prelude::*;

/**
 * Win the water fight by controlling the most territory, or out-soak your opponent!
 **/

#[macroquad::main("MyGame")]
async fn main() {
    let mut ctx = read_input();
    let mut agg_system = AggSystem::new();

    let mut ticker = 0.0;
    let mut iteration = 0;

    // game loop
    loop {
        let dt = get_frame_time();

        ticker += dt;

        clear_background(BLACK);

        draw_map(&ctx);
        draw_heroes(&ctx);

        if is_key_down(KeyCode::R) {
            find_cover_position(&ctx, 1);
        }

        if ticker >= 1.0 {
            logger::log(&iteration, "main::ticker");
            iteration += 1;
            match read_for_loop(&mut ctx) {
                Ok(_) => {}
                Err(err) => {
                    if let Err(inner) = read_for_loop_update(&mut ctx) {
                        eprintln!("ACTION:{}", inner);
                    }
                    eprintln!("{:?}", err);
                }
            }

            let res = agg_system.process(&ctx);
            if res.len() > 0 {
                match simulator_action(&mut ctx, res) {
                    Result::Ok(_) => {}
                    Err(e_string) => panic!("{}", e_string),
                }
            }

            ticker -= 1.0;
        }

        render_context(&ctx);

        next_frame().await
    }
}
