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
    // 行動をランダムで選択
    let random_index = rand::random::<u32>() % battle.enemy_actions.len() as u32;
    let action = &battle.enemy_actions[random_index as usize];
    BattleConduct {
        actor_character_id: request.enemy_character_id,
        target: BattleConductTargetType::Player,
        art: Arc::clone(&action.art),
        battle_weapon_id: action.battle_weapon_id.clone(),
    }
}
