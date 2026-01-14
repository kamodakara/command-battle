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
    let target = battle.players.first().unwrap();
    BattleConduct {
        actor_character_id: request.enemy_character_id,
        target_character_id: target.character_id,
        conduct: Arc::new(Conduct {
            name: "敵の攻撃".to_string(),
            sp_cost: 0,
            stamina_cost: 0,
            perks: vec![],
            requirement: ConductRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            conduct_type: ConductType::Basic(ConductTypeBasic::Attack(ConductTypeBasicAttack {
                attack_power: AttackPower {
                    slash: 10,
                    strike: 0,
                    thrust: 0,
                    impact: 0,
                    magic: 0,
                    fire: 0,
                    lightning: 0,
                    chaos: 0,
                },
                break_power: 5,
            })),
        }),
        weapon: None,
    }
}
