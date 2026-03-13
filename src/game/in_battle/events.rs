use bevy::prelude::*;
use crate::fundamental::{Art, BattleWeaponId};
use std::sync::Arc;

// ─── 共有データ型 ─────────────────────────────────────────────────────────────

/// コマンドキューの1エントリ（UIが管理し、Logicに渡す）
#[derive(Clone)]
pub struct ConsecutiveCommandEntry {
    pub art: Arc<Art>,
    pub weapon_index: Option<usize>,
    pub battle_weapon_id: Option<BattleWeaponId>,
}

// ─── Logic → UI ──────────────────────────────────────────────────────────────

/// 戦闘ログメッセージ（全ての戦闘テキストをまとめてUIへ通知）
#[derive(Message)]
pub struct BattleLogEvent(pub String);

/// 敵がダメージを受けた（ダメージポップアップ専用）
#[derive(Message)]
pub struct EnemyDamagedEvent {
    pub amount: u32,
}

/// 敵の行動が決定した（次の行動表示用）
#[derive(Message)]
pub struct EnemyActionPlannedEvent {
    pub action_name: String,
}

/// バトル結果
#[derive(Message)]
pub enum BattleResultEvent {
    Victory,
    Defeat,
}

// ─── UI → Logic ──────────────────────────────────────────────────────────────

/// コマンドを1つ実行する（UIがキューを管理し、1ターン分を送る）
#[derive(Message)]
pub struct ExecuteBattleCommandsEvent {
    pub command: ConsecutiveCommandEntry,
    pub use_combination: bool,
}
