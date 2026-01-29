use super::equipment::GuardCutRate;

// 状態異常
#[derive(Clone)]
pub enum StatusAilment {
    Poison, // 毒
    Sleep,  // 眠気
    Chill,  // 寒気

    Bleed,     // 出血
    Burn,      // 火傷
    Paralysis, // 麻痺

    Fear, // 恐怖
    Rage, // 激昂
}

// =================== 状態変化 ================= //

// 状態変化
#[derive(Clone)]
pub struct StatusCondition {
    pub potency: StatusConditionPotency,   // 効果量
    pub duration: StatusConditionDuration, // 継続時間
}

// 状態変化持続時間
#[derive(Clone)]
pub enum StatusConditionDuration {
    Permanent,
    Turn(StatusConditionDurationTurn),   // ターン数
    Count(StatusConditionDurationCount), // 回数
    UntilNextAction,                     // 次の行動まで
}
#[derive(Clone)]
pub struct StatusConditionDurationTurn {
    // 効果ターン数
    pub turns: u32,
}

#[derive(Clone)]
pub struct StatusConditionDurationCount {
    // 効果回数
    pub count: u32,
}

#[derive(Clone)]
pub enum StatusConditionPotency {
    Resistance(StatusConditionResistance), // 防御
    Break(StatusConditionBreak),           // ブレイク状態
    Evasion,                               // 回避
    Airborne,                              // 空中
    Floating,                              // 浮遊
    Melee,                                 // 近距離
    Ranged,                                // 遠距離
}

// 戦闘中の状態変化 防御状態
#[derive(Clone)]
pub struct StatusConditionResistance {
    pub cut_rate: GuardCutRate, // カット率
                                // pub guard_strength: u32,    // ガード強度
}
// 戦闘中の状態変化 ブレイク状態
#[derive(Clone)]
pub struct StatusConditionBreak {
    // TODO: 詳細な効果
    // ダメージ計算に使う値
    // ブレイク回復については別途処理で行うのでここには不要かも
}
