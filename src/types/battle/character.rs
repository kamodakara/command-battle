use crate::types::Ability;

use super::*;

use std::sync::Arc;

pub struct BattleCharacter {
    pub character_id: BattleCharacterId,

    pub raw_ability: Ability,
    pub raw_base_defense_power: DefensePower,
    pub raw_equipment: Equipment,

    pub character_type: BattleCharacterType,

    pub hp: BattleCharacterHP,                  // HP
    pub sp: BattleCharacterSP,                  // SP
    pub stamina: BattleCharacterStamina,        // スタミナ (プレイヤーのみ)
    pub break_resistance: BattleCharacterBreak, // ブレイク耐性 (敵のみ)

    pub weapons: Vec<BattleWeapon>, // 装備武器

    pub is_dead: bool,                                 // 死亡状態
    pub status_ailment: BattleStatusAilment,           // 戦闘中の状態異常
    pub status_conditions: Vec<BattleStatusCondition>, // 状態変化
}

pub type BattleCharacterId = u32;

#[derive(PartialEq)]
pub enum BattleCharacterType {
    Player,
    Enemy,
}

// HP
pub struct BattleCharacterHP {
    pub max_hp: u32,
    pub current_hp: u32,
}
// SP
pub struct BattleCharacterSP {
    pub max_sp: u32,
    pub current_sp: u32,
}
// スタミナ
pub struct BattleCharacterStamina {
    pub max_stamina: u32,
    pub current_stamina: u32,
    pub stamina_recovery: u32,
}
// ブレイク
pub struct BattleCharacterBreak {
    pub max_break: u32,               // ブレイク最大値
    pub current_break: u32,           // 現在のブレイク値
    pub break_recovery: u32,          // ブレイク回復量
    pub break_not_damaged_turns: u32, // ブレイクダメージを受けてないターン数

    pub is_breaking: bool,             // ブレイク中
    pub max_breaking_turns: u32,       // ブレイク中、最大ターン
    pub remaining_breaking_turns: u32, // ブレイク中、残りターン数
}
