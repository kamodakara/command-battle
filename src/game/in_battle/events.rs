use bevy::prelude::*;
use crate::fundamental::{Art, BattleWeaponId, BattleIncidentConduct, BattleIncidentCharacter};
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

/// 汎用ログメッセージ（ターン番号・アーツ選択などのメタ情報）
#[derive(Message)]
pub struct BattleLogEvent(pub String);

/// コンビネーション発動結果
#[derive(Message)]
pub struct BattleCombinationEvent {
    pub actor_character_id: u32,
    pub incident: Arc<BattleIncidentCharacter>,
}

/// 1行動の実行結果（プレイヤー・敵それぞれ1回ずつ発火）
#[derive(Message)]
pub struct BattleConductResolvedEvent {
    pub incident: Arc<BattleIncidentConduct>,
    pub player_character_id: u32,
    pub enemy_character_id: u32,
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
