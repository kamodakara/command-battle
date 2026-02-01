mod character;
mod incident;
mod karma;
mod trance;

use super::StatusAilment;
use super::character::Ability;
use super::character::AbilityType;
use super::common::*;
use super::conduct::Art;
use super::effect::{
    EffectAbilityModifier, EffectAttackDamageModifier, EffectHpPercentageDamage,
    EffectReceiveDamageModifier, EffectRemoveStatusAilment, EffectSpPercentageDamage,
};
use super::equipment::{Equipment, Weapon};
use super::karma::*;
use super::status_ailment::StatusConditionPotency;
use super::trance::*;
use std::sync::Arc;

pub use character::*;
pub use incident::*;
pub use karma::*;
pub use trance::*;

// 戦闘中の状態異常
#[derive(Clone)]
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
#[derive(Clone)]
pub struct BattleStatusAilmentStatus {
    // 蓄積上限値
    pub max_accumulation: u32,
    // 状態異常値回復量
    pub recovery_amount: u32,
    // 状態異常中の状態異常値回復割合
    pub ailment_recovery_rate: f32,

    // 蓄積量
    pub accumulation: u32,
    // 状態異常になってるか
    pub is_ailment: bool,
    // 状態異常値の蓄積がないターン数
    pub no_accumulation_turns: u32,
}

// 状態異常になった瞬間の効果
// 状態異常付与時に一度だけ効果を発揮する
pub enum BattleStatusAilmentOnAilmentEffect {
    HpPercentageDamage(EffectHpPercentageDamage), // HP最大値の割合ダメージ
    SpPercentageDamage(EffectSpPercentageDamage), // SP最大値の割合ダメージ
                                                  // TODO: トランス値ダメージ
}
// 状態異常の継続効果
pub enum BattleStatusAilmentOngoingEffect {
    HpPercentageDamage(EffectHpPercentageDamage), // HP最大値の割合ダメージ
    SpPercentageDamage(EffectSpPercentageDamage), // SP最大値の割合ダメージ
    AttackDamageModifier(EffectAttackDamageModifier), // 与ダメージ補正
    ReceiveDamageModifier(EffectReceiveDamageModifier), // 被ダメージ補正
    RemoveStatusAilment(EffectRemoveStatusAilment), // 状態異常解除
    AbilityModifier(EffectAbilityModifier),       // 能力補正
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
#[derive(Clone, PartialEq, Debug)]
pub struct BattleWeaponId(pub u32);

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
