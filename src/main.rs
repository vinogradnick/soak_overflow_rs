pub mod cg_reader;
pub mod context;
pub mod hero_cmd;
pub mod hero_profile;
pub mod map_state;
pub mod pathfinder;
mod position;
pub mod reader;
pub mod sim_reader;
pub mod strategy;

use crate::cg_reader::CGReader;
use crate::context::GameContext;
use crate::hero_profile::HeroService;
use crate::map_state::MapState;

use crate::reader::Reader;
use crate::sim_reader::SimReader;
use crate::strategy::{SaveStrategy, Strategy};

use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let strat = SaveStrategy;
    let mut reader = SimReader::new();
    let id = reader.read_i32();
    let mut hero_service = HeroService::new(id);
    hero_service.read_profile(&mut reader);
    let mut map_state = MapState::from_input(&mut reader);

    let mut steps = 0;
    loop {
        let mouse_pos = mouse_position();
        let mouse_point = vec2(mouse_pos.0, mouse_pos.1);
        let tile_w = screen_width() / map_state.width as f32;
        let tile_h = screen_height() / map_state.height as f32;
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
            for ent in hero_service.my_list() {
                eprintln!("{:#?}", ent);
            }
        }
        steps += 1;

        clear_background(BLACK);

        for tile in &map_state.tiles {
            let rec = Rect::new(
                tile.position.x as f32 * tile_w,
                tile.position.y as f32 * tile_h,
                tile_w,
                tile_h,
            );

            draw_rectangle(
                tile.position.x as f32 * tile_w,
                tile.position.y as f32 * tile_h,
                tile_w,
                tile_h,
                match tile.tile_type {
                    1 => Color::from_rgba(80, 80, 80, 255), // тёмно-серый (вместо LIGHTGRAY)
                    2 => Color::from_rgba(50, 50, 50, 255), // ещё темнее серый (вместо GRAY)
                    _ => Color::from_rgba(60, 100, 60, 255), // тёмно-пастельный зелёный
                },
            );

            draw_text(
                format!("{},{}", tile.position.x, tile.position.y).as_str(),
                tile.position.x as f32 * tile_w + tile_w / 2.0 - 10.0,
                tile.position.y as f32 * tile_h + tile_h / 2.0,
                20.0,
                GREEN,
            );

            if rec.contains(mouse_point) {
                draw_text(
                    format!("TileType:{}", tile.tile_type).as_str(),
                    tile.position.x as f32 * tile_w,
                    tile.position.y as f32 * tile_h + 20.0,
                    20.0,
                    GREEN,
                );

                draw_rectangle_lines(
                    tile.position.x as f32 * tile_w,
                    tile.position.y as f32 * tile_h,
                    tile_w,
                    tile_h,
                    5.0,
                    Color::from_hex(0x3CA7D5),
                );
            }
        }
        for hero in hero_service.entities_list() {
            draw_circle(
                hero.position.x as f32 * tile_w + tile_w / 2.0,
                hero.position.y as f32 * tile_h + tile_h / 2.0,
                tile_w.min(tile_h) * 0.4, // круг займёт ~80% клетки
                if hero.is_owner { BLUE } else { RED },
            );

            let rec = Rect::new(
                hero.position.x as f32 * tile_w,
                hero.position.y as f32 * tile_h,
                tile_w,
                tile_h,
            );

            if rec.contains(mouse_point) {
                let items = hero.fields_vec();

                for v in 0..items.len() {
                    draw_text(
                        items[v].as_str(),
                        hero.position.x as f32 * tile_w,
                        hero.position.y as f32 * tile_h + 40.0 + v as f32 * 18.0,
                        20.0,
                        WHITE,
                    );
                }
            }
        }

        for v in 0..actions.len() {
            draw_text(
                actions[v].as_str(),
                screen_width() - 250.0,
                screen_height() - (20.0 * (v as f32 + 1.0)),
                20.0,
                WHITE,
            );
        }

        // draw_line(
        //     0.0 * tile_w + tile_w / 2.0,
        //     2.0 * tile_h + tile_h / 2.0,
        //     4.0 * tile_w + tile_w / 2.0,
        //     3.0 * tile_h + tile_h / 2.0,
        //     10.0,
        //     GOLD,
        // );

        next_frame().await
    }
}
