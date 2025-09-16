use std::fmt;

use crate::data::position::Position;

#[derive(Debug, Clone)]
pub enum HeroAction {
    Move(Position),
    Throw(Position),
    Shoot(i32), // agent_id цели
    Wait,
}

// Реализация Display для HeroAction
impl fmt::Display for HeroAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeroAction::Move(pos) => write!(f, "MOVE {} {}", pos.x, pos.y),
            HeroAction::Shoot(id) => write!(f, "SHOOT {}", id),
            HeroAction::Wait => write!(f, "WAIT"),
            HeroAction::Throw(position) => write!(f, "THROW {} {}", position.x, position.y),
        }
    }
}

#[derive(Debug)]
pub struct HeroCommand(pub i32, pub Vec<HeroAction>);

// Реализация Display для HeroCommand
impl fmt::Display for HeroCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let actions_str: Vec<String> = self.1.iter().map(|a| a.to_string()).collect();
        write!(f, "{}; {}", self.0, actions_str.join("; "))
    }
}
