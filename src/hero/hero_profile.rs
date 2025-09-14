#[derive(Debug, Clone, Copy)]
pub struct HeroProfile {
    pub is_owner: bool,
    pub agent_id: i32,
    pub player: i32,
    pub shoot_cooldown: i32,
    pub optimal_range: i32,
    pub soaking_power: i32,
    pub splash_bombs: i32,
}
