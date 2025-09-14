use crate::position::Position;

#[derive(Debug, Clone, Copy)]
pub struct HeroEntity {
    pub position: Position,
    pub is_owner: bool,
    pub agent_id: i32,
    pub cooldown: i32,
    pub splash_bombs: i32,
    pub wetness: i32,
}

impl HeroEntity {
    pub fn fields_vec(&self) -> Vec<String> {
        vec![
            format!("id: {}", self.agent_id),
            format!("owner: {}", self.is_owner),
            format!("cd: {}", self.cooldown),
            format!("bombs: {}", self.splash_bombs),
            format!("wet: {}", self.wetness),
            format!("pos: ({},{})", self.position.x, self.position.y),
        ]
    }
}
