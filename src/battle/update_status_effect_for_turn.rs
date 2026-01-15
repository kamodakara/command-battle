use std::{char, path::PathBuf};

use super::*;

pub struct UpdateStatusEffectRequest {
    pub character_id: BattleCharacterId,
}

// 状態変化の更新
pub fn update_status_effect_for_turn(
    battle: &mut Battle,
    request: UpdateStatusEffectRequest,
) -> BattleIncidentAutoTrigger {
    let character_id = request.character_id;
    let status_effects = if let Some(player) = battle
        .players
        .iter_mut()
        .find(|c| c.character_id == character_id)
    {
        &mut player.base.status_effects
    } else if let Some(enemy) = battle
        .enemies
        .iter_mut()
        .find(|c| c.character_id == character_id)
    {
        &mut enemy.base.status_effects
    } else {
        panic!("Character not found");
    };

    let mut finished_effects: Vec<usize> = vec![];
    let mut finished_effect_incidents: Vec<BattleIncidentStatusEffect> = vec![];
    for (index, es) in status_effects.iter_mut().enumerate() {
        if let BattleStatusEffectDuration::Turn(turn_duration) = &mut es.duration {
            turn_duration.elapsed_turns += 1;
            if turn_duration.elapsed_turns >= turn_duration.turns {
                // 効果ターン終了
                finished_effects.push(index);
                finished_effect_incidents.push(BattleIncidentStatusEffect {
                    status_effect: es.clone(),
                    status_effect_handling: BattleIncidentStatusEffectHandling::Removed(
                        BattleIncidentStatusEffectRemoved {},
                    ),
                });
            }
        }
    }
    for index in finished_effects.iter() {
        status_effects.remove(*index);
    }

    BattleIncidentAutoTrigger {
        character_id,
        stats_changes: vec![],
        status_effects: finished_effect_incidents,
    }
}
