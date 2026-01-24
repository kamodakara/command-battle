mod character;
mod incident;
mod karma;

use super::character::AbilityType;
use super::common::*;
use super::conduct::Art;
use super::equipment::{Equipment, Weapon};
use super::karma::*;
use super::status_ailment::StatusConditionPotency;
use crate::types::StatusAilment;
use std::sync::Arc;

pub use character::*;
pub use incident::*;
pub use karma::*;

// 戦闘中の状態異常
pub struct BattleStatusAilment {
    pub poison: BattleStatusAilmentStatus,    // 毒
    pub sleep: BattleStatusAilmentStatus,     // 眠気
    pub chill: BattleStatusAilmentStatus,     // 寒気
    pub bleed: BattleStatusAilmentStatus,     // 出血
    pub burn: BattleStatusAilmentStatus,      // 火傷
    pub paralysis: BattleStatusAilmentStatus, // 麻痺
    pub fear: BattleStatusAilmentStatus,      // 恐怖
    pub rage: BattleStatusAilmentStatus,      // 激昂
}
// 戦闘中の状態異常ステータス
pub struct BattleStatusAilmentStatus {
    // 蓄積量
    pub accumulation: u32,
    // 状態異常になってるか
    pub is_ailment: bool,
}
impl BattleStatusAilmentStatus {
    pub fn new() -> Self {
        Self {
            accumulation: 0,
            is_ailment: false,
        }
    }
}

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
    pub target: BattleConductTargetType,
    pub art: Arc<Art>,                            // 使用アーツ
    pub battle_weapon_id: Option<BattleWeaponId>, // 使用武器ID
}

#[derive(Clone, PartialEq, Eq)]
pub enum BattleConductTargetType {
    PlayerSingle(BattleCharacterId), // プレイヤー側単体
    EnemySingle(BattleCharacterId),  // 敵側単体
    PlayerAll,                       // プレイヤー側全体
    EnemyAll,                        // 敵側全体
}
