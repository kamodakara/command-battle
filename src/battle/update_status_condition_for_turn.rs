use std::{char, path::PathBuf};

use super::*;

pub struct UpdateStatusConditionRequest {
    pub character_id: BattleCharacterId,
}

// 状態変化の更新
pub fn update_status_condition_for_turn(
    battle: &mut Battle,
    request: UpdateStatusConditionRequest,
) -> BattleIncidentAutoTrigger {
    let character_id = request.character_id;
    let status_conditions = if let Some(player) = battle
        .players
        .iter_mut()
        .find(|c| c.character_id == character_id)
    {
        &mut player.base.status_conditions
    } else if let Some(enemy) = battle
        .enemies
        .iter_mut()
        .find(|c| c.character_id == character_id)
    {
        &mut enemy.base.status_conditions
    } else {
        panic!("Character not found");
    };

    let mut finished_conditions: Vec<usize> = vec![];
    let mut finished_condition_incidents: Vec<BattleIncidentStatusCondition> = vec![];
    for (index, es) in status_conditions.iter_mut().enumerate() {
        if let BattleStatusConditionDuration::Turn(turn_duration) = &mut es.duration {
            turn_duration.elapsed_turns += 1;
            if turn_duration.elapsed_turns >= turn_duration.turns {
                // 効果ターン終了
                finished_conditions.push(index);
                finished_condition_incidents.push(BattleIncidentStatusCondition {
                    status_condition: es.clone(),
                    status_condition_handling: BattleIncidentStatusConditionHandling::Removed(
                        BattleIncidentStatusConditionRemoved {},
                    ),
                });
            }
        }
    }
    for index in finished_conditions.iter() {
        status_conditions.remove(*index);
    }

    BattleIncidentAutoTrigger {
        character_id,
        stats_changes: vec![],
        status_conditions: finished_condition_incidents,
    }
}
