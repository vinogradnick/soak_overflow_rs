use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::data::{game_context::GameContext, tile::TileMeta};

pub struct ScoringSystem {
    ctx: Rc<RefCell<GameContext>>,
    scoring: Vec<i32>,
    save_score: Vec<i32>,
    width: usize,
}

impl ScoringSystem {
    pub fn new(ctx: Rc<RefCell<GameContext>>) -> Self {
        let (full, width): (usize, usize) = {
            let tl = ctx.borrow();
            (
                tl.tilemap.get_height() * tl.tilemap.get_width(),
                tl.tilemap.get_width(),
            )
        };

        Self {
            ctx,
            scoring: vec![0; full],
            save_score: vec![0; full],
            width,
        }
    }

    pub fn update(&mut self) {
        let mut ctx = self.ctx.borrow();
        // здесь можно менять GameContext
    }
}
