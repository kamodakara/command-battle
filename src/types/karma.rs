use super::character::AbilityType;

pub struct KarmaDeck {
    pub name: String,
    pub cards: Vec<KarmaCard>,
}

pub struct KarmaCard {
    pub deck_cost: u32,
    pub max_turn: u32,             // 場に出ている最大ターン数
    pub effects: Vec<KarmaEffect>, // 空の時は効果なし
}

pub enum KarmaEffect {
    // 攻撃ダメージ補正
    AttackDamageModifier { modifier: f32 },
    // 受けるダメージ補正
    ReceiveDamageModifier { modifier: f32 },
    // 能力上昇
    AbilityIncrease { ability: AbilityType, amount: u32 },
}
