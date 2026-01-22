mod character;
mod incident;

use super::character::AbilityType;
use super::common::*;
use super::conduct::Art;
use super::equipment::{Equipment, Weapon};
use super::status_ailment::StatusConditionPotency;
use std::sync::Arc;

pub use character::*;
pub use incident::*;

// 戦闘中の状態変化
#[derive(Clone)]
pub struct BattleStatusCondition {
    pub potency: StatusConditionPotency,         // 状態変化効果
    pub duration: BattleStatusConditionDuration, // 継続時間
}
#[derive(Clone)]
pub enum BattleStatusConditionDuration {
    Permanent,                                 // 永続
    Turn(BattleStatusConditionDurationTurn),   // ターン数
    Count(BattleStatusConditionDurationCount), // 回数
    UntilNextAction,                           // 次の行動まで
}
#[derive(Clone)]
pub struct BattleStatusConditionDurationTurn {
    // 効果ターン数
    pub turns: u32,
    // 経過ターン数
    pub elapsed_turns: u32,
}
#[derive(Clone)]
pub struct BattleStatusConditionDurationCount {
    // 効果回数
    pub count: u32,
    // 経過回数
    pub elapsed_count: u32,
}

pub struct Battle {
    pub players: Vec<BattleCharacter>,
    pub enemies: Vec<BattleCharacter>,
}

#[derive(Clone)]
pub struct BattleWeapon {
    pub id: BattleWeaponId, // 武器ID
    pub weapon: Weapon,     // 武器情報
}
// 武器ID
#[derive(Clone, PartialEq)]
pub struct BattleWeaponId(u32);

#[derive(Clone)]
pub struct BattleConduct {
    pub actor_character_id: u32,
    pub target_character_id: u32,
    pub art: Arc<Art>,                            // 使用アーツ
    pub battle_weapon_id: Option<BattleWeaponId>, // 使用武器ID
}
