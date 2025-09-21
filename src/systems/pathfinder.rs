use crate::data::{game_context::GameContext, position::Position};

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

        for next in ctx.tilemap.neighbors(&pos) {
            if ctx.tilemap.get_view(next).0 == 0 {
                queue.push_back(next);
            }
        }
    }

    false
}
