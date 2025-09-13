use crate::position::Position;

#[derive(Debug, Clone)]
pub enum HeroAction {
    Move(Position),
    Shoot(i32), // agent_id цели
    Wait,
}

#[derive(Debug, Clone)]
pub struct HeroCmd {
    pub hero_id: i32,
    pub actions: Vec<HeroAction>,
}

impl HeroCmd {
    pub fn new(id: i32) -> Self {
        HeroCmd {
            hero_id: id,
            actions: vec![],
        }
    }
    pub fn to_string(&self) -> String {
        let items = self
            .actions
            .iter()
            .map(|cmd| match &cmd {
                HeroAction::Move(pos) => format!("MOVE {}", pos),
                HeroAction::Shoot(target_id) => format!("SHOOT {}", target_id),
                HeroAction::Wait => format!("WAIT",),
            })
            .collect::<Vec<_>>()
            .join("; "); // или "\n", если нужно

        return format!("{}; {}", self.hero_id, items);
    }
    pub fn with(&mut self, action: HeroAction) -> &mut Self {
        self.actions.push(action);
        return self;
    }
}
