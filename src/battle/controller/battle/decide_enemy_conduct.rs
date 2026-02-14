use super::*;
use std::sync::Arc;

// TODO: 実装
// 敵キャラクターの行動決定
// どういうデータを返すか

pub struct DecideEnemyConductRequest {
    pub enemy_character_id: BattleCharacterId,
}

pub fn decide_enemy_conduct(battle: &Battle, request: DecideEnemyConductRequest) -> BattleConduct {
    // TODO: 実装

    // 仮
    BattleConduct {
        actor_character_id: request.enemy_character_id,
        target: BattleConductTargetType::Player,
        art: Arc::new(Art {
            name: "爪".to_string(),
            sp_cost: 0,
            stamina_cost: 0,
            perks: vec![],
            requirement: ArtRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            art_type: ArtType::Basic,
            usable_weapon: ArtUsableWeapon::All,
            always_hits: false,
            priority: 0,
            rank1: ArtRank {
                threshold: 0,
                target: ArtTarget::Single,
                potency: ArtPotency::Attack(ArtPotencyAttack {
                    attack_power: AttackPower {
                        slash: 500,
                        strike: 0,
                        thrust: 0,
                        impact: 0,
                        magic: 0,
                        fire: 0,
                        lightning: 0,
                        chaos: 0,
                    },
                    weapon_attack_power_scaling: AttackPowerScaling::default(),
                    break_power: 0,
                    weapon_break_power_scaling: 0.0,
                }),
            },
            rank2: None,
            rank3: None,
        }),
        battle_weapon_id: None,
    }
}
