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

// --------------

// pub enum BattleCharacter<'a> {
//     Player(&'a mut BattlePlayer),
//     Enemy(&'a mut BattleEnemy),
// }
// impl<'a> BattleCharacter<'a> {
//     pub fn character_id(&self) -> u32 {
//         match self {
//             BattleCharacter::Player(c) => c.character_id,
//             BattleCharacter::Enemy(c) => c.character_id,
//         }
//     }
//     pub fn current_ability(&self) -> &BattleAbility {
//         match self {
//             BattleCharacter::Player(c) => &c.base.current_ability,
//             BattleCharacter::Enemy(c) => &c.base.current_ability,
//         }
//     }
//     pub fn current_stats(&self) -> &BattleStats {
//         match self {
//             BattleCharacter::Player(c) => &c.base.current_stats,
//             BattleCharacter::Enemy(c) => &c.base.current_stats,
//         }
//     }
//     pub fn status_conditions(&self) -> &Vec<BattleStatusCondition> {
//         match self {
//             BattleCharacter::Player(c) => &c.base.status_conditions,
//             BattleCharacter::Enemy(c) => &c.base.status_conditions,
//         }
//     }
//     pub fn defense_power(&self) -> &DefensePower {
//         match self {
//             BattleCharacter::Player(c) => &c.base.defense_power,
//             BattleCharacter::Enemy(c) => &c.base.defense_power,
//         }
//     }

//     pub fn current_stats_mut(&mut self) -> &mut BattleStats {
//         match self {
//             BattleCharacter::Player(c) => &mut c.base.current_stats,
//             BattleCharacter::Enemy(c) => &mut c.base.current_stats,
//         }
//     }
//     pub fn status_conditions_mut(&mut self) -> &mut Vec<BattleStatusCondition> {
//         match self {
//             BattleCharacter::Player(c) => &mut c.base.status_conditions,
//             BattleCharacter::Enemy(c) => &mut c.base.status_conditions,
//         }
//     }
// }

// pub struct BattleStats {
//     pub max_hp: u32, // HP
//     pub max_sp: u32, // SP
//     // プレイヤーのみ使用
//     pub max_stamina: u32,      // スタミナ 敵は使用しない
//     pub stamina_recovery: u32, // スタミナ回復量 敵は使用しない

//     pub current_hp: u32,      // 現在のHP
//     pub current_sp: u32,      // 現在のSP
//     pub current_stamina: u32, // 現在のスタミナ 敵は使用しない
// }
// impl BattleStats {
//     // HPに加算
//     pub fn hp_add(&mut self, amount: u32) -> (u32, u32) {
//         let before = self.current_hp;
//         self.current_hp = (self.current_hp + amount).min(self.max_hp);

//         (before, self.current_hp)
//     }
//     // SPに加算
//     pub fn sp_add(&mut self, amount: u32) -> (u32, u32) {
//         let before = self.current_sp;
//         self.current_sp = (self.current_sp + amount).min(self.max_sp);
//         (before, self.current_sp)
//     }
//     // スタミナに加算
//     pub fn stamina_add(&mut self, amount: u32) -> (u32, u32) {
//         let before = self.current_stamina;
//         self.current_stamina = (self.current_stamina + amount).min(self.max_stamina);
//         (before, self.current_stamina)
//     }
//     // HPに減算
//     pub fn hp_subtract(&mut self, amount: u32) -> (u32, u32) {
//         let before = self.current_hp;
//         self.current_hp = self.current_hp.saturating_sub(amount);
//         (before, self.current_hp)
//     }
//     // SPに減算
//     pub fn sp_subtract(&mut self, amount: u32) -> (u32, u32) {
//         let before = self.current_sp;
//         self.current_sp = self.current_sp.saturating_sub(amount);
//         (before, self.current_sp)
//     }
//     // スタミナに減算
//     pub fn stamina_subtract(&mut self, amount: u32) -> (u32, u32) {
//         let before = self.current_stamina;
//         self.current_stamina = self.current_stamina.saturating_sub(amount);
//         (before, self.current_stamina)
//     }
// }

// pub struct BattleEnemyOnlyStats {
//     pub max_break: u32,      // ブレイク最大値
//     pub max_break_turn: u32, // ブレイク最大ターン
//     pub break_recovery: u32, // ブレイク回復量

//     pub current_break: u32,           // 現在のブレイク値
//     pub break_not_damaged_turns: u32, // ブレイクダメージを受けてないターン数
//     pub break_turns: u32,             // 現在のブレイク経過ターン
// }
// impl BattleEnemyOnlyStats {
//     // ブレイク値に加算
//     pub fn break_add(&mut self, amount: u32) -> (u32, u32) {
//         let before = self.current_break;
//         self.current_break = (self.current_break + amount).min(self.max_break);
//         (before, self.current_break)
//     }
//     // ブレイク値に減算
//     pub fn break_subtract(&mut self, amount: u32) -> (u32, u32) {
//         let before = self.current_break;
//         self.current_break = self.current_break.saturating_sub(amount);
//         (before, self.current_break)
//     }
// }

// // 戦闘者
// pub struct BattleCharacterBase {
//     pub current_ability: BattleAbility,
//     pub current_stats: BattleStats,
//     pub defense_power: DefensePower,

//     pub status_conditions: Vec<BattleStatusCondition>, // 状態変化
//     pub is_dead: bool,                                 // 死亡状態
// }

// // バトル中のプレイヤーの状態
// pub struct BattlePlayer {
//     pub character_id: BattleCharacterId,
//     pub original: Arc<Player>,

//     pub base: BattleCharacterBase,
// }
// // バトル中の敵の状態
// pub struct BattleEnemy {
//     pub character_id: BattleCharacterId,
//     pub original: Arc<Enemy>,

//     pub base: BattleCharacterBase,
//     pub current_enemy_only_stats: BattleEnemyOnlyStats,
// }
