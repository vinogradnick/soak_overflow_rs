use crate::{context::GameContext, position::Position};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq)]
struct Node {
    position: Position,
    cost: usize, // f = g + h
    g: usize,    // стоимость пути от старта
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // max-heap -> инвертируем
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: &Position, b: &Position) -> usize {
    ((a.x as isize - b.x as isize).abs() + (a.y as isize - b.y as isize).abs()) as usize
}

pub fn find_path_with_cost(
    ctx: &GameContext,
    start: &Position,
    goal: &Position,
) -> Option<Vec<(Position, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut g_score = vec![vec![usize::MAX; ctx.map_state.width]; ctx.map_state.height];

    g_score[start.y][start.x] = 0;
    open_set.push(Node {
        position: *start,
        cost: heuristic(start, goal),
        g: 0,
    });

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(current) = open_set.pop() {
        if current.position == *goal {
            // восстановление пути с стоимостью
            let mut path = vec![(current.position, current.g)];
            let mut cur = current.position;
            while let Some(prev) = came_from.get(&cur) {
                path.push((*prev, g_score[prev.y][prev.x]));
                cur = *prev;
            }
            path.reverse();
            return Some(path);
        }

        for (dx, dy) in directions {
            let nx = current.position.x as isize + dx;
            let ny = current.position.y as isize + dy;

            if nx < 0
                || ny < 0
                || nx >= ctx.map_state.width as isize
                || ny >= ctx.map_state.height as isize
            {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            // проверка проходимости тайла
            if let Some(tile) = ctx.map_state.get_tile(ny, nx) {
                if tile.tile_type != 0 {
                    continue;
                }
            } else {
                continue;
            }

            let tentative_g = current.g + 1; // стоимость перехода = 1
            if tentative_g < g_score[ny][nx] {
                g_score[ny][nx] = tentative_g;
                let f = tentative_g + heuristic(&Position { x: nx, y: ny }, goal);
                came_from.insert(Position { x: nx, y: ny }, current.position);
                open_set.push(Node {
                    position: Position { x: nx, y: ny },
                    cost: f,
                    g: tentative_g,
                });
            }
        }
    }

    None // путь не найден
}
