use super::*;

// コンビネーション用技判定用行動カテゴリ
pub enum CombinationConductCategory {
    Attack,               // 攻撃
    Support,              // 支援
    ArtBase,              // アーツ、基礎
    ArtSkill,             // アーツ、技
    ArtSorcery,           // アーツ、術
    Attribute(Attribute), // 属性
    Guard,                // ガード
}

pub struct Heart {
    level1_effect: HeartEffect,
    level2_effect: HeartEffect,
    level3_effect: HeartEffect,

    combination: HeartCombinationSkill,
}

pub enum HeartEffect {
    // TODO: 実装
}

// コンビネーション技
pub struct HeartCombinationSkill {
    name: String,
    effect: HeartCombinationEffect,
    condition: HeartCombinationSkillCondition, // 発動条件
}
pub struct HeartCombinationSkillCondition {
    // TODO: 実装
}

pub enum HeartCombinationEffect {
    // TODO: 実装
}
