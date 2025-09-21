use crate::data::{game_context::GameContext, hero::HeroCommand, position::Position};

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

fn color_convert<S: AsRef<str>>(s: S) -> Color {
    let s = s.as_ref().trim_start_matches('#');
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
    let tile_w = screen_width() / ctx.tilemap.get_width() as f32;
    let tile_h = screen_height() / ctx.tilemap.get_height() as f32;
    let mouse_point = vec2(mouse_position().0, mouse_position().1);

    for (tile_type, enemy_id, position) in ctx.tilemap.iter() {
        let rec = Rect::new(
            position.x as f32 * tile_w,
            position.y as f32 * tile_h,
            tile_w,
            tile_h,
        );

        draw_rectangle(
            rec.x,
            rec.y,
            tile_w,
            tile_h,
            match tile_type {
                1 => color_convert("#669BBC"),
                2 => color_convert("#003049"),
                _ => color_convert("#000000"),
            },
        );
        draw_rectangle_lines(rec.x, rec.y, tile_w, tile_h, 1.0, color_convert("#14213D"));

        if rec.contains(mouse_point) {
            draw_text(
                format!("TileType:{}", tile_type).as_str(),
                rec.x,
                rec.y + 20.0,
                20.0,
                GREEN,
            );
            draw_tile_text(
                format!("{}", position).as_str(),
                &position,
                tile_w,
                tile_h,
                20.0,
                WHITE,
            );
            draw_rectangle_lines(rec.x, rec.y, tile_w, tile_h, 5.0, Color::from_hex(0x3CA7D5));
        }
    }
}

pub fn draw_heroes(ctx: &GameContext) {
    let tile_w = screen_width() / ctx.tilemap.get_width() as f32;
    let tile_h = screen_height() / ctx.tilemap.get_height() as f32;
    let mouse_point = vec2(mouse_position().0, mouse_position().1);

    for hero in ctx.hero_store.iter() {
        if !hero.initialized {
            continue;
        }

        draw_circle(
            hero.position.x as f32 * tile_w + tile_w / 2.0,
            hero.position.y as f32 * tile_h + tile_h / 2.0,
            tile_w.min(tile_h) * 0.4,
            if hero.player == ctx.player_id {
                color_convert("#FCA311")
            } else {
                color_convert("#780000")
            },
        );
        draw_tile_text(
            &hero.agent_id.to_string(),
            &hero.position,
            tile_w,
            tile_h,
            20.0,
            WHITE,
        );

        let rec = Rect::new(
            hero.position.x as f32 * tile_w,
            hero.position.y as f32 * tile_h,
            tile_w,
            tile_h,
        );
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

pub fn debug_position<S: AsRef<str>, U: AsRef<str>>(
    ctx: &GameContext,
    position: &Position,
    color: S,
    meta: U,
) {
    let tile_w = screen_width() / ctx.tilemap.get_width() as f32;
    let tile_h = screen_height() / ctx.tilemap.get_height() as f32;

    let rec = Rect::new(
        position.x as f32 * tile_w,
        position.y as f32 * tile_h,
        tile_w,
        tile_h,
    );

    draw_rectangle(rec.x, rec.y, tile_w, tile_h, color_convert(color));

    draw_tile_text(meta.as_ref(), position, tile_w, tile_h, 20.0, BLACK);
}

pub async fn render_context(game_context: &GameContext) {
    clear_background(BLACK);
    draw_map(game_context);
    draw_heroes(game_context);
    next_frame().await
}
