use std::collections::HashSet;

use super::EnemyCommandId;

// ─── 行動セット ─────────────────────────────────────────────────────────────

/// 1ターンの3回行動セット
#[derive(Clone, Debug, PartialEq)]
pub struct ActionSet {
    pub name: String,
    pub commands: Vec<EnemyCommandId>,
}

impl ActionSet {
    pub fn new(name: impl Into<String>, commands: Vec<EnemyCommandId>) -> Self {
        Self {
            name: name.into(),
            commands,
        }
    }
}

// ─── ビヘイビアツリーノード ─────────────────────────────────────────────────

/// ビヘイビアツリーノード
///
/// 評価結果: `Some(ActionSet)` = 成功, `None` = 失敗
pub enum BehaviorNode {
    /// 子を左から順に評価し、最初に成功したものを返す
    /// 全て失敗した場合は `None` を返す
    Selector(Vec<BehaviorNode>),

    /// 条件が真のときのみ `child` を評価し、偽なら `None` を返す
    Gate {
        condition: BehaviorCondition,
        child: Box<BehaviorNode>,
    },

    /// 重みに応じてランダムに子ノードを1つ選択して評価する
    WeightedRandom(Vec<WeightedChoice>),

    /// 常にこの行動セットを返す（必ず成功）
    Fixed(ActionSet),

    /// `flag_id` が未使用の場合のみ `child` を評価し成功させる
    /// 一度成功するとフラグが立ち、以降は `None` を返す
    OneShot {
        flag_id: String,
        child: Box<BehaviorNode>,
    },
}

/// `WeightedRandom` の選択肢
pub struct WeightedChoice {
    pub weight: u32,
    pub node: BehaviorNode,
}

// ─── 条件 ────────────────────────────────────────────────────────────────────

/// ノードの評価条件
pub enum BehaviorCondition {
    /// HP が `threshold_percent` 以下のとき真 (0.0 〜 1.0)
    HpBelow { threshold_percent: f32 },
    /// HP が `threshold_percent` 以上のとき真
    HpAbove { threshold_percent: f32 },
    /// ターン数が `[min, max]` の範囲内のとき真
    TurnCount { min: Option<u32>, max: Option<u32> },
}

/// フェーズ移行条件（移行後はHPが回復しても元に戻らない・不可逆）
pub enum PhaseCondition {
    HpBelow { threshold_percent: f32 },
}

// ─── フェーズ ────────────────────────────────────────────────────────────────

/// 敵の行動フェーズ
pub struct EnemyPhase {
    /// 移行条件（`None` = 初期フェーズ）
    pub enter_condition: Option<PhaseCondition>,

    /// フェーズ移行時に1度だけ実行される行動セット
    /// `None` の場合は通常のツリー評価に進む
    pub entry_action: Option<ActionSet>,

    /// 毎ターン評価されるビヘイビアツリー
    pub root: BehaviorNode,
}

/// 敵のビヘイビアツリー全体
pub struct EnemyBehaviorTree {
    /// `phases[0]` = 初期フェーズ
    /// 以降は `enter_condition` の HP 閾値降順（50% → 25% → ...）で並べる
    pub phases: Vec<EnemyPhase>,
}

// ─── ランタイム状態 ──────────────────────────────────────────────────────────

/// 敵1体ごとのランタイム状態（ターン跨ぎで保持する）
#[derive(Debug, Default)]
pub struct EnemyAiState {
    /// 現在のフェーズインデックス
    pub current_phase: usize,
    /// フェーズ移行直後フラグ（`entry_action` 実行に使用）
    pub phase_entry_pending: bool,
    /// 使用済みの `OneShot` フラグセット
    pub one_shot_flags: HashSet<String>,
}

/// 評価時のコンテキスト（毎ターン構築）
pub struct AiContext<'a> {
    /// 現在HP / 最大HP（0.0 〜 1.0）
    pub hp_percent: f32,
    /// 現在ターン数
    pub turn: u32,
    /// ランタイム状態への可変参照
    pub ai_state: &'a mut EnemyAiState,
}
