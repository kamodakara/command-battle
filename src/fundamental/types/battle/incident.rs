use super::*;

// 戦闘出来事

// 行動起点での出来事
pub struct BattleIncidentConduct {
    pub actor_character_id: BattleCharacterId,
    pub target: BattleConductTargetType,
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
    pub attacker: BattleIncidentCharacter,
    // 被行動者
    pub defenders: Vec<BattleIncidentConductOutcomeSuccessDefender>,
}

pub struct BattleIncidentConductOutcomeSuccessAttacker {
    pub character_id: BattleCharacterId,
    pub character_incidents: Vec<BattleCharacterIncident>,
}
pub struct BattleIncidentConductOutcomeSuccessDefender {
    pub character: BattleIncidentCharacter,
    pub is_evaded: bool,   // 回避したか TODO: 回避した理由
    pub is_defended: bool, // 防御したか
    pub is_dead: bool,     // 戦闘不能になったか
}

// 戦闘者
pub struct BattleIncidentCharacter {
    pub character_id: BattleCharacterId,
    pub incidents: Vec<BattleCharacterIncident>,
}

// 戦闘者の出来事
pub struct BattleCharacterIncident {
    pub reason: BattleCharacterIncidentReason,
    pub concretes: Vec<BattleCharacterIncidentConcrete>,
}
pub enum BattleCharacterIncidentReason {
    ConductConsumption, // 行動時の消費(SP、スタミナなど)
    ConductEffect,      // 行動の効果を受けた(攻撃を受けた、回復を受けたなど)
    TurnEndRecovery,    // ターン終了時の回復
}

// 戦闘者発生した具体的な出来事
pub enum BattleCharacterIncidentConcrete {
    DamageHp(BattleIncidentDamageHp),             // HPダメージ
    DamageSp(BattleIncidentDamageSp),             // SPダメージ
    DamageStamina(BattleIncidentDamageStamina),   // スタミナダメージ
    DamageBreak(BattleIncidentDamageBreak),       // ブレイクダメージ
    RecoverHp(BattleIncidentRecoverHp),           // HP回復
    RecoverSp(BattleIncidentRecoverSp),           // SP回復
    RecoverStamina(BattleIncidentRecoverStamina), // スタミナ回復
    RecoverBreak(BattleIncidentRecoverBreak),     // ブレイク回復
    StatusConditionApplied(BattleIncidentStatusConditionApplied), // 状態変化付与
    StatusConditionRemoved(BattleIncidentStatusConditionRemoved), // 状態変化解除
    StatusAilmentAccumulation(BattleIncidentStatusAilmentAccumulation), // 状態異常値蓄積
    StatusAilmentRecovery(BattleIncidentStatusAilmentRecovery), // 状態異常値回復
    StatusAilmentApplied(BattleIncidentStatusAilmentApplied), // 状態異常付与
    StatusAilmentRemoved(BattleIncidentStatusAilmentRemoved), // 状態異常解除
    Death(BattleIncidentDeath),                   // 死亡
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
// 状態異常値蓄積
pub struct BattleIncidentStatusAilmentAccumulation {
    pub status_ailment: StatusAilment,
    pub accumulation: u32,
    pub before_accumulation: u32,
    pub after_accumulation: u32,
}
// 状態異常値回復
pub struct BattleIncidentStatusAilmentRecovery {
    pub status_ailment: StatusAilment,
    pub recover: u32,
    pub before_accumulation: u32,
    pub after_accumulation: u32,
}
// 状態変化付与
pub struct BattleIncidentStatusConditionApplied {
    pub status_condition: BattleStatusCondition,
}
// 状態変化解除
pub struct BattleIncidentStatusConditionRemoved {
    pub status_condition: BattleStatusCondition,
}

// 状態異常付与
pub struct BattleIncidentStatusAilmentApplied {
    pub status_ailment: StatusAilment,
}
// 状態異常解除
pub struct BattleIncidentStatusAilmentRemoved {
    pub status_ailment: StatusAilment,
}

// 行動失敗
pub struct BattleIncidentConductOutcomeFailure {
    pub reason: BattleIncidentConductOutcomeFailureReason,
}
pub enum BattleIncidentConductOutcomeFailureReason {
    InsufficientStamina, // スタミナ不足
    InsufficientAbility, // 能力不足
    InsufficientSp,      // SP不足
    IsBreak,             // ブレイク状態
}
pub struct BattleIncidentDeath {
    // TODO: 必要な情報
}
