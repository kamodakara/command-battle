use super::character::AbilityType;
use super::effect::{
    EffectAbilityIncrease, EffectAttackDamageModifier, EffectReceiveDamageModifier,
};

pub struct KarmaDeck {
    pub name: String,
    pub cards: Vec<KarmaCard>,
}

#[derive(Debug)]
pub struct KarmaCard {
    pub name: String,
    pub cost: u32,
    pub max_turn: u32,             // 場に出ている最大ターン数
    pub effects: Vec<KarmaEffect>, // 空の時は効果なし
}

#[derive(PartialEq)]
pub enum KarmaEffectType {
    AttackDamageModifier,
    ReceiveDamageModifier,
    AbilityIncrease,
}
#[derive(Clone, Debug)]
pub enum KarmaEffect {
    // 攻撃ダメージ補正
    AttackDamageModifier(EffectAttackDamageModifier),
    // 受けるダメージ補正
    ReceiveDamageModifier(EffectReceiveDamageModifier),
    // 能力上昇
    AbilityIncrease(EffectAbilityIncrease),
}
