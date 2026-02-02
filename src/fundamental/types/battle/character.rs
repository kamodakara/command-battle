use super::Ability;

use super::*;

use std::sync::Arc;

pub struct BattleCharacter {
    pub character_id: BattleCharacterId,

    pub raw_ability: Ability,
    pub raw_base_defense_power: DefensePower,
    pub raw_equipment: Equipment,

    pub character_type: BattleCharacterType,

    pub hp: BattleCharacterHP,           // HP
    pub sp: BattleCharacterSP,           // SP
    pub stamina: BattleCharacterStamina, // スタミナ (プレイヤーのみ)

    pub weapons: Vec<BattleWeapon>, // 装備武器

    pub status_ailment: BattleStatusAilment, // 戦闘中の状態異常
    pub status_conditions: Vec<BattleStatusCondition>, // 状態変化

    pub karma: Option<BattleKarma>,   // カルマ (プレイヤーのみ)
    pub trance: Option<BattleTrance>, // トランス (プレイヤーのみ)
    pub combination_skill: Option<BattleCombinationSkill>, // コンビネーション技 (プレイヤーのみ)
}

pub type BattleCharacterId = u32;

#[derive(Clone, PartialEq)]
pub enum BattleCharacterType {
    Player,
    Enemy,
}

// HP
#[derive(Clone)]
pub struct BattleCharacterHP {
    pub max_hp: u32,
    pub current_hp: u32,
    pub is_dead: bool, // 死亡状態
}
// SP
#[derive(Clone)]
pub struct BattleCharacterSP {
    pub max_sp: u32,
    pub current_sp: u32,
}
// スタミナ
#[derive(Clone)]
pub struct BattleCharacterStamina {
    pub max_stamina: u32,
    pub current_stamina: u32,
    pub stamina_recovery: u32,
}
