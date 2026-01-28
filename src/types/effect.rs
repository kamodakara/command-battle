use super::character::AbilityType;
use super::status_ailment::StatusAilment;

pub enum Effect {
    HpPercentageDamage(EffectHpPercentageDamage), // HP最大値の割合ダメージ
    SpPercentageDamage(EffectSpPercentageDamage), // SP最大値の割合ダメージ
    AttackDamageModifier(EffectAttackDamageModifier), // 与ダメージ補正
    ReceiveDamageModifier(EffectReceiveDamageModifier), // 被ダメージ補正
    RemoveStatusAilment(EffectRemoveStatusAilment), // 状態異常解除
    AbilityIncrease(EffectAbilityIncrease),       // 能力上昇
    AbilityModifier(EffectAbilityModifier),       // 能力補正
}

pub struct EffectHpPercentageDamage {
    pub percentage: f32, // HP最大値の割合ダメージ
}

pub struct EffectSpPercentageDamage {
    pub percentage: f32, // SP最大値の割合ダメージ
}
pub struct EffectAttackDamageModifier {
    pub modifier: f32, // 与ダメージ補正
}
pub struct EffectReceiveDamageModifier {
    pub modifier: f32, // 被ダメージ補正
}

pub struct EffectRemoveStatusAilment {
    pub status_ailments: Vec<StatusAilment>,
}

pub struct EffectAbilityIncrease {
    pub ability_type: AbilityType,
    pub amount: u32,
}
pub struct EffectAbilityModifier {
    pub ability_type: AbilityType,
    pub modifier: f32,
}
