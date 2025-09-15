use crate::{context::GameContext, hero::hero_cmd::HeroCommand, position::Position};
use macroquad::prelude::*;

pub const BLACK_BG: Color = BLACK;

pub const STATIC_COLORS: [&str; 16] = [
    "#FF0000", // красный
    "#00FF00", // зеленый
    "#0000FF", // синий
    "#FFFF00", // желтый
    "#FF00FF", // пурпурный
    "#00FFFF", // циан
    "#800000", // темно-красный
    "#008000", // темно-зеленый
    "#000080", // темно-синий
    "#808000", // оливковый
    "#800080", // фиолетовый
    "#008080", // бирюзовый
    "#C0C0C0", // серебристый
    "#808080", // серый
    "#FFA500", // оранжевый
    "#A52A2A", // коричневый
];

fn color_convert(s: &str) -> Color {
    let s = s.trim_start_matches('#');
    let r = u8::from_str_radix(&s[0..2], 16).unwrap();
    let g = u8::from_str_radix(&s[2..4], 16).unwrap();
    let b = u8::from_str_radix(&s[4..6], 16).unwrap();
    let a = if s.len() == 8 {
        u8::from_str_radix(&s[6..8], 16).unwrap()
    } else {
        255
    };

    Color::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    )
}

pub fn draw_map(ctx: &GameContext) {
    let tile_w = screen_width() / ctx.map_state.width as f32;
    let tile_h = screen_height() / ctx.map_state.height as f32;
    let mouse_point = vec2(mouse_position().0, mouse_position().1);

    for tile in &ctx.map_state.tiles {
        let rec = Rect::new(
            tile.position.x as f32 * tile_w,
            tile.position.y as f32 * tile_h,
            tile_w,
            tile_h,
        );

        draw_rectangle(
            rec.x,
            rec.y,
            tile_w,
            tile_h,
            match tile.tile_type {
                1 => Color::from_rgba(80, 80, 80, 255),
                2 => Color::from_rgba(50, 50, 50, 255),
                _ => Color::from_rgba(60, 100, 60, 255),
            },
        );

        draw_tile_text(
            format!("{}", tile.position).as_str(),
            &tile.position,
            tile_w,
            tile_h,
            20.0,
            YELLOW,
        );

        if rec.contains(mouse_point) {
            draw_text(
                format!("TileType:{}", tile.tile_type).as_str(),
                rec.x,
                rec.y + 20.0,
                20.0,
                GREEN,
            );
            draw_rectangle_lines(rec.x, rec.y, tile_w, tile_h, 5.0, Color::from_hex(0x3CA7D5));
        }
    }
}

pub fn draw_heroes(ctx: &GameContext) {
    let tile_w = screen_width() / ctx.map_state.width as f32;
    let tile_h = screen_height() / ctx.map_state.height as f32;
    let mouse_point = vec2(mouse_position().0, mouse_position().1);

    for hero in ctx.hero_service.entities_list() {
        draw_circle(
            hero.position.x as f32 * tile_w + tile_w / 2.0,
            hero.position.y as f32 * tile_h + tile_h / 2.0,
            tile_w.min(tile_h) * 0.4,
            if hero.is_owner { BLUE } else { RED },
        );

        let rec = Rect::new(
            hero.position.x as f32 * tile_w,
            hero.position.y as f32 * tile_h,
            tile_w,
            tile_h,
        );

        if rec.contains(mouse_point) {
            for (i, field) in hero.fields_vec().iter().enumerate() {
                draw_text(field, rec.x, rec.y + 40.0 + i as f32 * 18.0, 20.0, WHITE);
            }
        }
    }
}

pub fn draw_actions(actions: &[HeroCommand]) {
    for (i, act) in actions.iter().enumerate() {
        draw_text(
            format!("{:?}", act).as_str(),
            screen_width() - 250.0,
            screen_height() - (20.0 * (i as f32 + 1.0)),
            20.0,
            WHITE,
        );
    }
}

// Вспомогательная функция для рисования текста в центре тайла
fn draw_tile_text(
    text: &str,
    position: &Position,
    tile_w: f32,
    tile_h: f32,
    font_size: f32,
    color: Color,
) {
    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x = position.x as f32 * tile_w + tile_w / 2.0 - text_dimensions.width / 2.0;
    let y = position.y as f32 * tile_h + tile_h / 2.0 + text_dimensions.height / 2.0;
    draw_text(text, x, y, font_size, color);
}

pub fn debug_position(ctx: &GameContext, position: &Position, color: &str, meta: String) {
    let tile_w: f32 = screen_width() / ctx.map_state.width as f32;
    let tile_h = screen_height() / ctx.map_state.height as f32;

    let rec = Rect::new(
        position.x as f32 * tile_w,
        position.y as f32 * tile_h,
        tile_w,
        tile_h,
    );

    draw_rectangle(rec.x, rec.y, tile_w, tile_h, color_convert(color));

    draw_tile_text(meta.as_str(), position, tile_w, tile_h, 20.0, BLACK);
}
