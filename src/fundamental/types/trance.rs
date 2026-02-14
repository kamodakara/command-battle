use super::*;

#[derive(Debug)]
pub struct Heart {
    pub name: String,

    pub level1_effects: Vec<HeartEffect>,
    pub level2_effects: Vec<HeartEffect>,
    pub level3_effects: Vec<HeartEffect>,

    pub combination: Option<CombinationSkill>,
}

#[derive(Clone, Debug)]
pub enum HeartEffect {
    PhysicalDefenseModifier(EffectPhysicalDefenseModifier), // 物理防御力補正
    MagicalDefenseModifier(EffectMagicalDefenseModifier),   // 魔法防御力補正
    PhysicalAttackModifier(EffectPhysicalAttackModifier),   // 物理攻撃力補正
    MagicalAttackModifier(EffectMagicalAttackModifier),     // 魔法攻撃力補正
    StaminaRecoveryModifier(EffectStaminaRecoveryModifier), // スタミナ回復量補正
}

// コンビネーション技
#[derive(Debug)]
pub struct CombinationSkill {
    pub name: String,
    pub effect: HeartCombinationEffect,
    pub condition: CombinationSkillCondition, // 発動条件
}
#[derive(PartialEq, Debug)]
pub struct CombinationSkillCondition {
    // 現在の行動の必要条件
    pub current_requirements: CombinationSkillConditionRequirements,
    // 一つ前の行動の必要条件
    pub previous_requirements: Option<CombinationSkillConditionRequirements>,
    // 二つ前の行動の必要条件
    pub two_steps_before_requirements: Option<CombinationSkillConditionRequirements>,
}
#[derive(PartialEq, Debug)]
pub struct CombinationSkillConditionRequirements {
    pub categories: Vec<CombinationConductCategory>,
    pub results: Vec<CombinationConductResult>,
}

// コンビネーション用技判定用行動カテゴリ
#[derive(PartialEq, Debug)]
pub enum CombinationConductCategory {
    Attack,                     // 攻撃
    Support,                    // 支援
    ArtBasic,                   // アーツ、基礎
    ArtSkill,                   // アーツ、技
    ArtSorcery,                 // アーツ、術
    AttackAttribute(Attribute), // 攻撃属性
    Guard,                      // ガード
}
#[derive(PartialEq, Debug)]
pub enum CombinationConductResult {
    Success,      // 行動成功
    Failed,       // 行動失敗
    GuardSuccess, // ガード成功
}

#[derive(Debug)]
pub enum HeartCombinationEffect {
    AttackDamageModifier(EffectAttackDamageModifier), // 与ダメージ補正
    AttackBreakDamageModifier(EffectAttackBreakDamageModifier), // 与ブレイクダメージ補正
                                                      // TODO: ガード貫通
}
