use bevy::prelude::*;
use crate::fundamental::*;
use std::sync::Arc;

/// バトルフェーズ（ロジック書き込み、UIも読み取り可）
#[derive(Resource, PartialEq, Eq)]
pub enum BattlePhase {
    DecideEnemyConduct,
    AwaitCommand,
    ConfirmQueued,
    ConfirmAllCommands,
    InBattle,
    TurnEnd,
    Finished,
}

/// ターンカウント
#[derive(Resource)]
pub struct Turn(pub u32);

/// 次ターンの事前決定済み敵行動（ロジック専有）
#[derive(Resource)]
pub struct EnemyPlannedAction(pub Option<BattleConduct>);

/// 連続コマンドキュー（UIも表示用に参照可）
#[derive(Resource, Default)]
pub struct ConsecutiveCommands {
    pub commands: Vec<ConsecutiveCommandEntry>,
}

#[derive(Clone)]
pub struct ConsecutiveCommandEntry {
    pub art: Arc<Art>,
    pub weapon_index: Option<usize>,
    pub battle_weapon_id: Option<BattleWeaponId>,
}

/// バトルデータ（UIも読み取り可）
#[derive(Resource)]
pub struct BattleResource(pub Battle);

/// プレイヤーの基本アーツ（UIも読み取り可）
#[derive(Resource)]
pub struct PlayerBasicArts(pub Vec<Arc<Art>>);

/// プレイヤーの装備武器（UIも読み取り可）
#[derive(Resource)]
pub struct PlayerEquippedWeapons {
    pub weapons: Vec<EquippedWeaponWithArts>,
}

#[derive(Clone)]
pub struct EquippedWeaponWithArts {
    pub weapon: Weapon,
    pub skills: Vec<Arc<Art>>,
    pub sorceries: Vec<Arc<Art>>,
    pub battle_weapon_id: BattleWeaponId,
}
