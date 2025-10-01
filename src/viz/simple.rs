use std::fmt;

use crate::data::position::Position;

#[derive(Debug)]
pub enum DrawingCommand {
    Clear {
        color: String,
    },
    Rect {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: String,
    },
    Circle {
        x: i32,
        y: i32,
        r: i32,
        color: String,
    },
    Line {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: String,
        width: u32,
    },
    Text {
        x: i32,
        y: i32,
        text: String,
        color: String,
    },
}

impl fmt::Display for DrawingCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DrawingCommand::Clear { color } => write!(f, "CLEAR {}", color),
            DrawingCommand::Rect { x, y, w, h, color } => {
                write!(f, "DRAW RECT {} {} {} {} {}", x, y, w, h, color)
            }
            DrawingCommand::Circle { x, y, r, color } => {
                write!(f, "DRAW CIRCLE {} {} {} {}", x, y, r, color)
            }
            DrawingCommand::Line {
                x1,
                y1,
                x2,
                y2,
                color,
                width,
            } => write!(
                f,
                "DRAW LINE {} {} {} {} {} {}",
                x1, y1, x2, y2, color, width
            ),
            DrawingCommand::Text { x, y, text, color } => {
                write!(f, "TEXT {} {} \"{}\" {}", x, y, text, color)
            }
        }
    }
}

pub fn viz_simple_debug<S: AsRef<str>, R: AsRef<str>>(
    command_type: S,
    position: &Position,
    color: R,
    text: Option<String>,
) {
    let cmd_type = command_type.as_ref().to_uppercase();
    let color = color.as_ref();

    let output = match cmd_type.as_str() {
        "RECT" | "CIRCLE" => format!("DRAW {} {} {} {}", cmd_type, position.x, position.y, color),
        "TEXT" => {
            let t = text.unwrap_or_default();
            format!("TEXT {} {} \"{}\" {}", position.x, position.y, t, color)
        }
        _ => format!("UNKNOWN {} {} {}", cmd_type, position.x, position.y),
    };

    eprintln!("{}", output);
}
