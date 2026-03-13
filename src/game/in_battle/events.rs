use bevy::prelude::*;
use crate::fundamental::{Art, BattleWeaponId};
use std::sync::Arc;

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

/// プレイヤーがアーツを選択した
#[derive(Message)]
pub struct PlayerArtSelectedEvent {
    pub art: Arc<Art>,
    pub weapon_index: Option<usize>,
    pub battle_weapon_id: Option<BattleWeaponId>,
}

/// キューの先頭コマンドを実行する
#[derive(Message)]
pub struct ExecuteQueuedEvent {
    pub use_combination: bool,
}

/// キューのコマンドを全てキャンセル（入力しなおし）
#[derive(Message)]
pub struct CancelQueuedEvent;

/// キューの末尾のコマンドを取り消し
#[derive(Message)]
pub struct RemoveLastQueuedEvent;
