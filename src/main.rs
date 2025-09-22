pub mod data;
pub mod logger;
pub mod reader;
pub mod strategy;
pub mod systems;
pub mod viz;
pub mod simulator;
use std::{cell::RefCell, rc::Rc};

use crate::{
    data::game_context::GameContext,
    reader::Reader,
    strategy::Strategy,
    systems::{history::HistorySystem, score_system::ScoringSystem},
    viz::render::{draw_heroes, draw_map, render_context},
};
use macroquad::prelude::*;

/**
 * Win the water fight by controlling the most territory, or out-soak your opponent!
 **/

#[macroquad::main("MyGame")]
async fn main() {
    let ctx = Rc::new(RefCell::new(GameContext::new()));
    let mut scoring = ScoringSystem::new(Rc::clone(&ctx));
    let io_reader = Reader::SimulatorReader(false);
    {
        let mut ctx_mut = ctx.borrow_mut();
        io_reader.read_id(&mut ctx_mut);
        io_reader.read_profiles(&mut ctx_mut);
        io_reader.read_tilemap(&mut ctx_mut);
    }

    let mut ticker = 0.0;

    // game loop
    loop {
        let dt = get_frame_time();

        ticker += dt;

        clear_background(BLACK);
        {
            let ctx_immut = ctx.borrow_mut();
            draw_map(&ctx_immut);
            draw_heroes(&ctx_immut);

            if is_key_down(KeyCode::R) {
                Strategy::do_action(&ctx_immut);
            }
        }

        if ticker >= 1.0 {
            {
                let mut ctx_immut = ctx.borrow_mut();
                io_reader.read_entities(&mut ctx_immut);
                io_reader.read_number(&mut ctx_immut); // Number of alive agents controlled by you
            }
            scoring.update();
            {
                let mut ctx_immut = ctx.borrow_mut();
                let commands = Strategy::do_action(&ctx_immut);
                match io_reader.receive_action(&mut ctx_immut, commands) {
                    Ok(_) => {}
                    Err(data) => {
                        eprintln!("{}", data);
                    }
                }
            }

            ticker -= 1.0; // сбрасываем таймер, можно вычитать 1.0 чтобы не накапливался баг
        }
        {
            let ctx_immut = ctx.borrow_mut();
            render_context(&ctx_immut);
        }

        next_frame().await
    }
}
