use crate::data::game_context::GameContext;

pub struct HistorySystem {
    data: Vec<GameContext>,
    cursor: usize, // указывает на текущую позицию
    capacity: usize,
}

impl HistorySystem {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            cursor: 0,
            capacity,
        }
    }

    pub fn current(&self) -> Option<&GameContext> {
        self.data.get(self.cursor)
    }

    pub fn next(&mut self) -> Option<&GameContext> {
        if self.cursor + 1 < self.data.len() {
            self.cursor += 1;
        }
        self.data.get(self.cursor)
    }

    pub fn prev(&mut self) -> Option<&GameContext> {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
        self.data.get(self.cursor)
    }

    pub fn apply(&mut self, ctx: GameContext) {
        // если мы находимся не в конце истории — отрезаем "ветку"
        if self.cursor + 1 < self.data.len() {
            self.data.truncate(self.cursor + 1);
        }

        self.data.push(ctx);

        // ограничение по capacity
        if self.data.len() > self.capacity {
            self.data.remove(0);
        }

        self.cursor = self.data.len() - 1;
    }
}
