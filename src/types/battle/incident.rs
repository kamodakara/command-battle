use super::*;

// 戦闘出来事
pub enum BattleIncident {
    Conduct(BattleIncidentConduct),
}
pub struct BattleIncidentConduct {
    pub attacker_id: u32,
    pub defender_id: u32,
    pub conduct: BattleConduct,
    // 成否
    pub outcome: BattleIncidentConductOutcome,
    // TODO: その他必要な情報
}
// 攻撃の成否
pub enum BattleIncidentConductOutcome {
    Success(BattleIncidentConductOutcomeSuccess), // 発動
    Failure(BattleIncidentConductOutcomeFailure), // 不発
}
// 行動成功
pub struct BattleIncidentConductOutcomeSuccess {
    // 行動者
    pub attacker: BattleIncidentConductOutcomeSuccessAttacker,
    // 被行動者
    pub defenders: Vec<BattleIncidentConductOutcomeSuccessDefender>,
}

pub struct BattleIncidentConductOutcomeSuccessAttacker {
    pub character_id: u32,
    pub stats_changes: Vec<BattleIncidentStats>,
}
pub struct BattleIncidentConductOutcomeSuccessDefender {
    pub character_id: u32,
    pub stats_changes: Vec<BattleIncidentStats>,
    pub status_effects: Vec<BattleIncidentStatusEffect>, // 状態変化
    pub is_evaded: bool,                                 // 回避したか
    // TODO: 回避した理由
    pub is_defended: bool, // 防御したか
}

pub enum BattleIncidentStats {
    DamageHp(BattleIncidentDamageHp),
    DamageSp(BattleIncidentDamageSp),
    DamageStamina(BattleIncidentDamageStamina),
    DamageBreak(BattleIncidentDamageBreak),
}

// HPダメージ
pub struct BattleIncidentDamageHp {
    pub damage: u32,
    pub before: u32,
    pub after: u32,
}
// SP減少
pub struct BattleIncidentDamageSp {
    pub damage: u32,
    pub before: u32,
    pub after: u32,
}
// スタミナ減少
pub struct BattleIncidentDamageStamina {
    pub damage: u32,
    pub before: u32,
    pub after: u32,
}
// ブレイク減少
pub struct BattleIncidentDamageBreak {
    pub damage: u32,
    pub before: u32,
    pub after: u32,
}

// 状態変化
pub struct BattleIncidentStatusEffect {
    // 状態変化内容
    pub status_effect: BattleStatusEffect,
    // 発生内容
    pub status_effect_handling: BattleIncidentStatusEffectHandling,
}
pub enum BattleIncidentStatusEffectHandling {
    Applied(BattleIncidentStatusEffectApplied), // 付与
    Removed(BattleIncidentStatusEffectRemoved), // 解除
}
pub struct BattleIncidentStatusEffectApplied {
    // TODO: 付与理由
}
pub struct BattleIncidentStatusEffectRemoved {
    // TODO: 解除理由
}

// 行動失敗
pub struct BattleIncidentConductOutcomeFailure {
    pub reason: BattleIncidentConductOutcomeFailureReason,
}
pub struct BattleIncidentConductOutcomeFailureReason {
    // TODO: 詳細な理由分解
    pub insufficient_stamina: bool, // スタミナ不足
    pub insufficient_ability: bool, // 能力不足
    pub insufficient_sp: bool,      // SP不足
}
