use std::collections::{HashMap, HashSet, VecDeque};

use crate::data::{game_context::GameContext, position::Position, tilemap_iter::Neighbors};

pub fn can_reach(ctx: &GameContext, start: &Position, goal: &Position) -> bool {
    use std::collections::VecDeque;
    let mut visited = std::collections::HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start.clone());

    while let Some(pos) = queue.pop_front() {
        if pos == goal.clone() {
            return true;
        }

        if !visited.insert(pos) {
            continue;
        }

        for next in Neighbors::new(&ctx.tilemap, pos, false) {
            if ctx
                .tilemap
                .get_tile(&next.position)
                .is_some_and(|x| x.is_free())
            {
                queue.push_back(next.position);
            }
        }
    }

    false
}

pub fn find_path(ctx: &GameContext, start: &Position, goal: &Position) -> Option<Vec<Position>> {
    let mut visited = HashSet::new();
    let mut parents: HashMap<Position, Position> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(*start);

    while let Some(pos) = queue.pop_front() {
        if pos == *goal {
            // восстановим путь
            let mut path = vec![pos];
            let mut current = pos;
            while let Some(parent) = parents.get(&current) {
                path.push(*parent);
                current = *parent;
            }
            path.reverse();
            return Some(path);
        }

        if !visited.insert(pos) {
            continue;
        }

        for next in Neighbors::new(&ctx.tilemap, pos, false) {
            if ctx
                .tilemap
                .get_tile(&next.position)
                .is_some_and(|x| x.is_free())
                && !visited.contains(&next.position)
            {
                parents.insert(next.position, pos);
                queue.push_back(next.position);
            }
        }
    }

    None
}
