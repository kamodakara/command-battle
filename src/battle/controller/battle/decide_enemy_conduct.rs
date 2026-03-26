use super::*;
use std::sync::Arc;

pub struct DecideEnemyConductRequest {
    pub enemy_character_id: BattleCharacterId,
}

pub fn decide_enemy_conduct(
    battle: &mut Battle,
    request: DecideEnemyConductRequest,
) -> BattleConduct {
    // 行動進行中の場合は次のコマンドを返す
    if let Some(progress) = battle.enemy_action_progress.as_mut() {
        let command_id = progress.enemy_action.commands[progress.current_command_index];
        progress.current_command_index += 1;
        if progress.current_command_index >= progress.enemy_action.commands.len() {
            battle.enemy_action_progress = None;
        }
        return find_conduct(&battle.enemy_commands, request.enemy_character_id, command_id);
    }

    // ビヘイビアツリーで行動セット（3コマンド固定）を選択
    let hp_percent = battle
        .enemies
        .iter()
        .find(|e| e.character_id == request.enemy_character_id)
        .map(|e| e.hp.current_hp as f32 / e.hp.max_hp as f32)
        .unwrap_or(1.0);

    let mut context = AiContext {
        hp_percent,
        turn: 0, // TODO: ターン数をBattleで管理して渡す
        ai_state: &mut battle.enemy_ai_state,
    };
    let mut rng = rand::rng();
    let action_set = evaluate_turn(&battle.enemy_behavior_tree, &mut context, &mut rng)
        .expect("behavior tree returned None - no fallback defined");

    // 1コマンド目を即座に返し、残り2コマンドをプログレスに保存
    let first = action_set.commands[0];
    battle.enemy_action_progress = Some(EnemyActionProgress {
        enemy_action: EnemyAction {
            name: action_set.name,
            commands: vec![action_set.commands[1], action_set.commands[2]],
            hint: action_set.hint,
        },
        current_command_index: 0,
    });

    find_conduct(&battle.enemy_commands, request.enemy_character_id, first)
}

fn find_conduct(
    commands: &[EnemyCommand],
    character_id: BattleCharacterId,
    command_id: EnemyCommandId,
) -> BattleConduct {
    let command = commands
        .iter()
        .find(|c| c.id == command_id)
        .unwrap_or_else(|| panic!("Command {:?} not found", command_id));
    BattleConduct {
        actor_character_id: character_id,
        target: BattleConductTargetType::Player,
        art: Arc::clone(&command.art),
        battle_weapon_id: command.battle_weapon_id.clone(),
    }
}
