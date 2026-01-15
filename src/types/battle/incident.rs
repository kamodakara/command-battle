use super::*;

// 戦闘出来事
// TODO: これいらないかも
pub enum BattleIncident {
    Conduct(BattleIncidentConduct),         // 行動
    AutoTrigger(BattleIncidentAutoTrigger), // 自動発動
}
pub struct BattleIncidentConduct {
    pub attacker_id: BattleCharacterId,
    pub defender_id: BattleCharacterId,
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
    pub character_id: BattleCharacterId,
    pub stats_changes: Vec<BattleIncidentStats>,
}
pub struct BattleIncidentConductOutcomeSuccessDefender {
    pub character_id: BattleCharacterId,
    pub stats_changes: Vec<BattleIncidentStats>,
    pub status_effects: Vec<BattleIncidentStatusEffect>, // 状態変化
    pub is_evaded: bool,                                 // 回避したか
    // TODO: 回避した理由
    pub is_defended: bool, // 防御したか
                           // pub is_dead: bool,     // 戦闘不能になったか
}

pub enum BattleIncidentStats {
    DamageHp(BattleIncidentDamageHp),
    DamageSp(BattleIncidentDamageSp),
    DamageStamina(BattleIncidentDamageStamina),
    DamageBreak(BattleIncidentDamageBreak),
    RecoverHp(BattleIncidentRecoverHp),
    RecoverSp(BattleIncidentRecoverSp),
    RecoverStamina(BattleIncidentRecoverStamina),
    RecoverBreak(BattleIncidentRecoverBreak),
}

// HPダメージ
pub struct BattleIncidentDamageHp {
    pub damage: u32,
    pub before: u32, // ダメージ前HP
    pub after: u32,  // ダメージ後HP
}
// SPダメージ
pub struct BattleIncidentDamageSp {
    pub damage: u32,
    pub before: u32, // ダメージ前SP
    pub after: u32,  // ダメージ後SP
}
// スタミナダメージ
pub struct BattleIncidentDamageStamina {
    pub damage: u32,
    pub before: u32, // ダメージ前スタミナ
    pub after: u32,  // ダメージ後スタミナ
}
// ブレイクダメージ
pub struct BattleIncidentDamageBreak {
    pub damage: u32,
    pub before: u32, // ダメージ前ブレイク
    pub after: u32,  // ダメージ後ブレイク
}

// HP回復
pub struct BattleIncidentRecoverHp {
    pub recover: u32,
    pub before: u32, // 回復前HP
    pub after: u32,  // 回復後HP
}
// SP回復
pub struct BattleIncidentRecoverSp {
    pub recover: u32,
    pub before: u32, // 回復前SP
    pub after: u32,  // 回復後SP
}
// スタミナ回復
pub struct BattleIncidentRecoverStamina {
    pub recover: u32,
    pub before: u32, // 回復前スタミナ
    pub after: u32,  // 回復後スタミナ
}
// ブレイク回復
pub struct BattleIncidentRecoverBreak {
    pub recover: u32,
    pub before: u32, // 回復前ブレイク
    pub after: u32,  // 回復後ブレイク
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
    pub is_break: bool,             // ブレイク中
}

pub struct BattleIncidentAutoTrigger {
    pub character_id: BattleCharacterId,
    pub stats_changes: Vec<BattleIncidentStats>,
    pub status_effects: Vec<BattleIncidentStatusEffect>, // 状態変化
}
