use bevy::prelude::*;
use crate::fundamental::*;

/// バトルフェーズ（ロジック書き込み、UIも読み取り可）
#[derive(Resource, PartialEq, Eq)]
pub enum BattlePhase {
    DecideEnemyConduct,
    AwaitCommand,
    TurnEnd,
    Finished,
}

/// ターンカウント
#[derive(Resource)]
pub struct Turn(pub u32);

/// 1ターン内の実行済み行動数（3回でターン終了）
#[derive(Resource, Default)]
pub struct ActionsExecutedThisTurn(pub u32);

/// 事前決定済み敵行動（3回分）
#[derive(Resource, Default)]
pub struct EnemyPlannedActions(pub Vec<BattleConduct>);

/// バトルデータ（UIも読み取り可）
#[derive(Resource)]
pub struct BattleResource(pub Battle);

/// プレイヤーの基本アーツ（UIも読み取り可）
#[derive(Resource)]
pub struct PlayerBasicArts(pub Vec<std::sync::Arc<Art>>);

/// プレイヤーの装備武器（UIも読み取り可）
#[derive(Resource)]
pub struct PlayerEquippedWeapons {
    pub weapons: Vec<EquippedWeaponWithArts>,
}

#[derive(Clone)]
pub struct EquippedWeaponWithArts {
    pub weapon: Weapon,
    pub skills: Vec<std::sync::Arc<Art>>,
    pub sorceries: Vec<std::sync::Arc<Art>>,
    pub battle_weapon_id: BattleWeaponId,
}
