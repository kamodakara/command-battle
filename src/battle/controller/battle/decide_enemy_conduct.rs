use bevy::ecs::system::command;

use super::*;
use std::sync::Arc;

// TODO: 実装
// 敵キャラクターの行動決定
// どういうデータを返すか

pub struct DecideEnemyConductRequest {
    pub enemy_character_id: BattleCharacterId,
}

pub fn decide_enemy_conduct(
    battle: &mut Battle,
    request: DecideEnemyConductRequest,
) -> BattleConduct {
    // TODO: 実装

    // 仮実装

    if let Some(progress_action) = battle.enemy_action_progress.as_mut() {
        if progress_action.enemy_action.commands.len() > progress_action.current_command_index {
            let command_id = progress_action.enemy_action.commands
                [progress_action.current_command_index]
                .clone();

            // インデックスを進める
            progress_action.current_command_index += 1;
            if progress_action.current_command_index >= progress_action.enemy_action.commands.len()
            {
                // 行動が終了したらリセット
                battle.enemy_action_progress = None;
            }

            if let Some(command) = battle.enemy_commands.iter().find(|c| c.id == command_id) {
                // コマンドに対応する行動を返す
                return BattleConduct {
                    actor_character_id: request.enemy_character_id,
                    target: BattleConductTargetType::Player,
                    art: Arc::clone(&command.art),
                    battle_weapon_id: command.battle_weapon_id.clone(),
                };
            } else {
                // TODO: コマンドが見つからない場合のエラーハンドリング
                panic!("Command with id {:?} not found", command_id);
            }
        }
    }

    let enemy = battle
        .enemies
        .iter()
        .find(|e| e.character_id == request.enemy_character_id)
        .expect("Enemy character not found");
    if enemy.hp.current_hp < (enemy.hp.max_hp as f32 * 0.6) as u32 {
        battle.enemy_second_stage = true;
    }

    let command_lots = if battle.enemy_second_stage {
        &battle.enemy_second_stage_action_lots
    } else {
        &battle.enemy_action_lots
    };

    // 行動パターンをランダムで選択
    let total_weight: u32 = command_lots.iter().map(|lot| lot.weight).sum();
    let mut rng = rand::rng();
    let mut random_weight = rand::Rng::random_range(&mut rng, 0..total_weight);
    let selected_action = command_lots
        .iter()
        .find(|lot| {
            if random_weight < lot.weight {
                true
            } else {
                random_weight -= lot.weight;
                false
            }
        })
        .expect("No enemy actions available")
        .action
        .clone();
    battle.enemy_action_progress = Some(EnemyActionProgress {
        enemy_action: selected_action,
        current_command_index: 0,
    });

    // let random_index = rand::random::<u32>() % battle.enemy_actions.len() as u32;
    // let action = &battle.enemy_actions[random_index as usize];
    // battle.enemy_action_progress = Some(EnemyActionProgress {
    //     enemy_action: action.clone(),
    //     current_command_index: 0,
    // });

    // このターンは待機
    if let Some(command) = battle
        .enemy_commands
        .iter()
        .find(|c| c.id == EnemyCommandId(0))
    {
        return BattleConduct {
            actor_character_id: request.enemy_character_id,
            target: BattleConductTargetType::Player,
            art: Arc::clone(&command.art),
            battle_weapon_id: command.battle_weapon_id.clone(),
        };
    } else {
        panic!("Command with id {:?} not found", EnemyCommandId(0));
    }
}
