use crate::{data::context::GameContext, data::position::Position};

pub fn can_reach(ctx: &GameContext, start: &Position, goal: &Position) -> bool {
    use std::collections::VecDeque;
    let mut visited = std::collections::HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(*start);

    while let Some(pos) = queue.pop_front() {
        if pos == *goal {
            return true;
        }

        if !visited.insert(pos) {
            continue;
        }

        for next in ctx.map_state.neighbors(&pos) {
            if let Some(tile) = ctx.map_state.get_tile(next.position.x, next.position.y) {
                if tile.is_walkable() {
                    queue.push_back(next.position);
                }
            }
        }
    }

    false
}
