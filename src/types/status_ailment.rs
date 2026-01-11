use super::equipment::GuardCutRate;

// 状態異常
pub struct StatusAilment {
    // TODO
}

// 状態変化
pub struct StatusCondition {
    // TODO
}

pub struct StatusEffect {
    pub potency: StatusEffectPotency,   // 効果量
    pub duration: StatusEffectDuration, // 継続時間
}

// 状態変化持続時間
pub enum StatusEffectDuration {
    Permanent,
    Turn(StatusEffectDurationTurn),   // ターン数
    Count(StatusEffectDurationCount), // 回数
    UntilNextAction,                  // 次の行動まで
}
pub struct StatusEffectDurationTurn {
    // 効果ターン数
    pub turns: u32,
}

pub struct StatusEffectDurationCount {
    // 効果回数
    pub count: u32,
}

#[derive(Clone)]
pub enum StatusEffectPotency {
    Resistance(StatusEffectResistance), // 防御
    Break(StatusEffectBreak),           // ブレイク状態
    Evasion,                            // 回避
    Airborne,                           // 空中
    Floating,                           // 浮遊
    Melee,                              // 近距離
    Ranged,                             // 遠距離
}

// 戦闘中の状態変化 防御
#[derive(Clone)]
pub struct StatusEffectResistance {
    pub cut_rate: GuardCutRate,
}
// 戦闘中の状態変化 ブレイク状態
#[derive(Clone)]
pub struct StatusEffectBreak {
    // TODO: 詳細な効果
    // ダメージ計算に使う値
    // ブレイク回復については別途処理で行うのでここには不要かも
}
