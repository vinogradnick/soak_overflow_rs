use crate::data::position::Position;
#[derive(Debug, Clone, Copy)]
pub struct Hero {
    pub is_owner: bool,
    pub agent_id: i32,
    pub player: i32,
    pub soaking_power: i32,
    pub shoot_cooldown: i32,
    pub optimal_range: i32,
    pub splash_bombs: i32,
    pub position: Position,
    pub cooldown: i32,
    pub wetness: i32,
    pub alive: bool,
}

impl Hero {}

#[derive(Debug, Clone)]
pub struct HeroStore {
    pub heroes: Vec<Hero>,
    pub my_heroes: Vec<Hero>,
    pub enemies: Vec<Hero>,
}

impl HeroStore {
    pub fn new() -> HeroStore {
        Self {
            heroes: vec![],
            my_heroes: vec![],
            enemies: vec![],
        }
    }
}

/// Возможные действия героя за один ход.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeroActionVariant {
    /// Попытка перемещения к позиции (x, y)
    Move(Position),
    /// Попытка выстрела по агенту с id
    Shoot { id: i32 },
    /// Попытка броска взрывной бомбы в позицию (x, y)
    Throw(Position),
    /// Занять укрытие, снижая урон на 25% этим ходом
    HunkerDown,
    /// Отправка сообщения в viewer (для отладки)
    Message { text: String },
}

/// Действие героя в виде пары (id агента, список действий)
pub struct HeroAction(pub i32, pub Vec<HeroActionVariant>);

impl HeroAction {
    pub fn new(agent_id: i32) -> Self {
        Self(agent_id, vec![])
    }
    pub fn to_string(&self) -> String {
        let agent_id = self.0;
        let actions_str: Vec<String> = self
            .1
            .iter()
            .map(|action| match action {
                HeroActionVariant::Move(position) => format!("MOVE {} ", position),
                HeroActionVariant::Shoot { id } => format!("SHOOT {}", id),
                HeroActionVariant::Throw(position) => format!("THROW {} ", position),
                HeroActionVariant::HunkerDown => "HUNKER_DOWN".to_string(),
                HeroActionVariant::Message { text } => format!("MESSAGE {}", text),
            })
            .collect();

        std::iter::once(agent_id.to_string())
            .chain(actions_str.into_iter())
            .collect::<Vec<_>>()
            .join(";")
    }
}
