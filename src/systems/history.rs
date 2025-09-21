use crate::data::game_context::GameContext;

pub struct HistorySystem {
    data: Vec<GameContext>,
    cursor: usize,
}

impl HistorySystem {
    pub fn new() -> Self {
        Self {
            data: vec![],
            cursor: 0,
        }
    }
    pub fn next(&mut self) -> Option<&GameContext> {
        let ctx = self.data.get(self.cursor);
        self.cursor += 1;
        return ctx;
    }
    pub fn prev(&mut self) -> Option<&GameContext> {
        let ctx = self.data.get(self.cursor);
        self.cursor -= 1;
        return ctx;
    }

    pub fn apply(&mut self, ctx: &GameContext) {
        if self.data.len() > 5 {
            self.cursor = 0;
            self.data.clear();
        }
        self.data.push(ctx.clone());
    }
}
