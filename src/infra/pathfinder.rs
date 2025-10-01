use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    data::{game_context::GameContext, position::Position},
    infra::logger,
};

pub fn can_reach(ctx: &GameContext, start: &Position, goal: &Position) -> bool {
    logger::log_str(format!("Start:{:?} Goal:{:?}", start, goal), "can_reach");

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
        for next in ctx.tilemap.neighbors(&pos) {
            if ctx.tilemap.get_tile(&next).is_some_and(|x| x.is_free()) {
                queue.push_back(next);
            }
        }
    }
    false
}
pub fn find_path(ctx: &GameContext, start: &Position, goal: &Position) -> Option<Vec<Position>> {
    logger::log_str(format!("Start:{:?} Goal:{:?}", start, goal), "find_path");

    let mut visited = HashSet::new();
    let mut parents: HashMap<Position, Position> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(*start);
    while let Some(pos) = queue.pop_front() {
        if pos == *goal {
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
        for next in ctx.tilemap.neighbors(&pos) {
            if ctx.tilemap.get_tile(&next).is_some_and(|x| x.is_free()) && !visited.contains(&next)
            {
                parents.insert(next, pos);
                queue.push_back(next);
            }
        }
    }
    None
}
