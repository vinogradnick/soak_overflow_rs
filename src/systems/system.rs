use crate::data::{
    game_context::GameContext,
    position::{Position, PositionUtil},
};

pub fn score_position(ctx: &GameContext, id_hero: usize) {
    let mut count = 0;
    let hero_x = ctx.hero_store.positions_x[id_hero];
    let hero_y = ctx.hero_store.positions_y[id_hero];

    for enemy in 0..ctx.hero_store.length() {
        let x = ctx.hero_store.positions_x[enemy];
        let y = ctx.hero_store.positions_y[enemy];

        if PositionUtil::multi_distance((hero_x, hero_y), (x, y)) == 1 {
            count += 1;
        }
    }
    
    

}
