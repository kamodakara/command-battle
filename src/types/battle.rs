mod character;
mod incident;

use super::character::{Enemy, Player};
use super::common::*;
use super::conduct::Conduct;
use super::equipment::Weapon;
use super::status_ailment::StatusEffectPotency;
use std::rc::Rc;

pub use character::*;
pub use incident::*;

pub struct BattleAbility {
    pub agility: u32,      // 敏捷性
    pub strength: u32,     // 筋力
    pub dexterity: u32,    // 技量
    pub intelligence: u32, // 知力
    pub faith: u32,        // 信仰
    pub arcane: u32,       // 神秘
}

pub struct BattleStats {
    pub max_hp: u32,         // HP
    pub max_sp: u32,         // SP
    pub max_stamina: u32,    // スタミナ 敵は使用しない
    pub hp_damage: u32,      // 受けたHPダメージ
    pub sp_damage: u32,      // 受けたSPダメージ
    pub stamina_damage: u32, // 受けたスタミナダメージ
}
pub struct BattleEnemyOnlyStats {
    pub max_break: u32,      // ブレイク最大値
    pub break_recovery: u32, // ブレイク回復量
    pub break_max_turn: u32, // ブレイク最大ターン
    pub break_damage: u32,   // 受けたブレイクダメージ
    pub break_turns: u32,    // 現在のブレイク経過ターン
}

// 戦闘中の状態変化
#[derive(Clone)]
pub struct BattleStatusEffect {
    pub potency: StatusEffectPotency,         // 状態変化効果
    pub duration: BattleStatusEffectDuration, // 継続時間
}
#[derive(Clone)]
pub enum BattleStatusEffectDuration {
    Permanent,                              // 永続
    Turn(BattleStatusEffectDurationTurn),   // ターン数
    Count(BattleStatusEffectDurationCount), // 回数
    UntilNextAction,                        // 次の行動まで
}
#[derive(Clone)]
pub struct BattleStatusEffectDurationTurn {
    // 効果ターン数
    pub turns: u32,
    // 経過ターン数
    pub elapsed_turns: u32,
}
#[derive(Clone)]
pub struct BattleStatusEffectDurationCount {
    // 効果回数
    pub count: u32,
    // 経過回数
    pub elapsed_count: u32,
}

pub struct Battle {
    pub players: Vec<BattlePlayer>,
    pub enemies: Vec<BattleEnemy>,
}

pub struct BattleWeapon {
    pub original: Rc<Weapon>,
    pub attack_power: AttackPower, // 攻撃性能
    pub sorcery_power: f32,        // 術力
    pub break_power: u32,          // ブレイク力
}

pub struct BattleConduct {
    pub actor_character_id: u32,
    pub target_character_id: u32,
    pub conduct: Conduct,
    pub weapon: Option<BattleWeapon>,
}
