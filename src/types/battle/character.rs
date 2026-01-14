use super::*;

use std::sync::Arc;

pub enum BattleCharacter<'a> {
    Player(&'a mut BattlePlayer),
    Enemy(&'a mut BattleEnemy),
}
impl<'a> BattleCharacter<'a> {
    pub fn character_id(&self) -> u32 {
        match self {
            BattleCharacter::Player(c) => c.character_id,
            BattleCharacter::Enemy(c) => c.character_id,
        }
    }
    pub fn current_ability(&self) -> &BattleAbility {
        match self {
            BattleCharacter::Player(c) => &c.base.current_ability,
            BattleCharacter::Enemy(c) => &c.base.current_ability,
        }
    }
    pub fn current_stats(&self) -> &BattleStats {
        match self {
            BattleCharacter::Player(c) => &c.base.current_stats,
            BattleCharacter::Enemy(c) => &c.base.current_stats,
        }
    }
    pub fn status_effects(&self) -> &Vec<BattleStatusEffect> {
        match self {
            BattleCharacter::Player(c) => &c.base.status_effects,
            BattleCharacter::Enemy(c) => &c.base.status_effects,
        }
    }
    pub fn defense_power(&self) -> &DefensePower {
        match self {
            BattleCharacter::Player(c) => &c.base.defense_power,
            BattleCharacter::Enemy(c) => &c.base.defense_power,
        }
    }

    pub fn current_stats_mut(&mut self) -> &mut BattleStats {
        match self {
            BattleCharacter::Player(c) => &mut c.base.current_stats,
            BattleCharacter::Enemy(c) => &mut c.base.current_stats,
        }
    }
    pub fn status_effects_mut(&mut self) -> &mut Vec<BattleStatusEffect> {
        match self {
            BattleCharacter::Player(c) => &mut c.base.status_effects,
            BattleCharacter::Enemy(c) => &mut c.base.status_effects,
        }
    }
}

// 戦闘者
pub struct BattleCharacterBase {
    pub current_ability: BattleAbility,
    pub current_stats: BattleStats,
    pub defense_power: DefensePower,
    pub status_effects: Vec<BattleStatusEffect>,
}

pub type BattleCharacterId = u32;

// バトル中のプレイヤーの状態
pub struct BattlePlayer {
    pub character_id: BattleCharacterId,
    pub original: Arc<Player>,

    pub base: BattleCharacterBase,
}
// バトル中の敵の状態
pub struct BattleEnemy {
    pub character_id: BattleCharacterId,
    pub original: Arc<Enemy>,

    pub base: BattleCharacterBase,
    pub current_enemy_only_stats: BattleEnemyOnlyStats,
}
