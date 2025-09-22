pub mod data;
pub mod logger;
pub mod reader;
pub mod strategy;
pub mod systems;
use crate::{data::game_context::GameContext, reader::Reader, strategy::Strategy};

fn main() {
    let mut ctx = GameContext::new();
    let io_reader = Reader::CodeingameReader;
    io_reader.read_id(&mut ctx);
    io_reader.read_profiles(&mut ctx);
    io_reader.read_tilemap(&mut ctx);

    loop {
        io_reader.read_entities(&mut ctx);
        let my_agent_count = io_reader.read_number(&mut ctx); // Number of alive agents controlled by you
        let commands = Strategy::do_action(&ctx);
        match io_reader.receive_action(&mut ctx, commands) {
            Ok(_) => {}
            Err(data) => {
                eprintln!("{}", data);
            }
        }
    }
}
