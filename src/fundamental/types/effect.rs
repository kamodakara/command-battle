use super::character::AbilityType;
use super::status_ailment::StatusAilment;

pub enum Effect {
    HpPercentageDamage(EffectHpPercentageDamage), // HP最大値の割合ダメージ
    SpPercentageDamage(EffectSpPercentageDamage), // SP最大値の割合ダメージ
    AttackDamageModifier(EffectAttackDamageModifier), // 与ダメージ補正
    ReceiveDamageModifier(EffectReceiveDamageModifier), // 被ダメージ補正
    AttackBreakDamageModifier(EffectAttackBreakDamageModifier), // 与ブレイクダメージ補正
    RemoveStatusAilment(EffectRemoveStatusAilment), // 状態異常解除
    AbilityIncrease(EffectAbilityIncrease),       // 能力上昇
    AbilityModifier(EffectAbilityModifier),       // 能力補正
    PhysicalDefenseModifier(EffectPhysicalDefenseModifier), // 物理防御力補正
    MagicalDefenseModifier(EffectMagicalDefenseModifier), // 魔法防御力補正
    PhysicalAttackModifier(EffectPhysicalAttackModifier), // 物理攻撃力補正
    MagicalAttackModifier(EffectMagicalAttackModifier), // 魔法攻撃力補正
    StaminaRecoveryModifier(EffectStaminaRecoveryModifier), // スタミナ回復量補正
    UnableToAct, // 行動不能効果 TODO: 何で行動不能なのか情報を持たせるべきか？
}

#[derive(Clone, Debug)]
pub struct EffectHpPercentageDamage {
    pub percentage: f32, // HP最大値の割合ダメージ
}

#[derive(Clone, Debug)]
pub struct EffectSpPercentageDamage {
    pub percentage: f32, // SP最大値の割合ダメージ
}
#[derive(Clone, Debug)]
pub struct EffectAttackDamageModifier {
    pub modifier: f32, // 与ダメージ補正
}
#[derive(Clone, Debug)]
pub struct EffectReceiveDamageModifier {
    pub modifier: f32, // 被ダメージ補正
}
#[derive(Clone, Debug)]
pub struct EffectAttackBreakDamageModifier {
    pub modifier: f32, // 与ブレイクダメージ補正
}
#[derive(Clone, Debug)]
pub struct EffectRemoveStatusAilment {
    pub status_ailments: Vec<StatusAilment>,
}
#[derive(Clone, Debug)]
pub struct EffectAbilityIncrease {
    pub ability_type: AbilityType,
    pub amount: u32,
}
#[derive(Clone, Debug)]
pub struct EffectAbilityModifier {
    pub ability_type: AbilityType,
    pub modifier: f32,
}
#[derive(Clone, Debug)]
pub struct EffectPhysicalDefenseModifier {
    pub modifier: f32,
}
#[derive(Clone, Debug)]
pub struct EffectMagicalDefenseModifier {
    pub modifier: f32,
}
#[derive(Clone, Debug)]
pub struct EffectPhysicalAttackModifier {
    pub modifier: f32,
}
#[derive(Clone, Debug)]
pub struct EffectMagicalAttackModifier {
    pub modifier: f32,
}
#[derive(Clone, Debug)]
pub struct EffectStaminaRecoveryModifier {
    pub modifier: f32,
}
