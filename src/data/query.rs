pub struct Query<T> {
    pub len: usize,
    pub idx: usize,
    pub(crate) components: T,
}

// Пример для двух компонентов: player и position
pub struct PlayerQueryView<'a> {
    pub players: &'a [i32],
    pub positions_x: &'a [usize],
    pub positions_y: &'a [usize],
}

impl<'a> Iterator for Query<PlayerQueryView<'a>> {
    type Item = (i32, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            return None;
        }
        let result = (
            self.components.players[self.idx],
            (
                self.components.positions_x[self.idx],
                self.components.positions_y[self.idx],
            ),
        );
        self.idx += 1;
        Some(result)
    }
}
