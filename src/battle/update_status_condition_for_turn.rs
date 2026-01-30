use super::*;

pub struct UpdateStatusConditionRequest {
    pub character_id: BattleCharacterId,
}

// 状態変化の更新
pub fn update_status_condition_for_turn(
    battle: &mut Battle,
    request: UpdateStatusConditionRequest,
) -> BattleIncidentCharacter {
    let character_id = request.character_id;
    let status_conditions = if let Some(player) = battle
        .players
        .iter_mut()
        .find(|c| c.character_id == character_id)
    {
        &mut player.status_conditions
    } else if let Some(enemy) = battle
        .enemies
        .iter_mut()
        .find(|c| c.character_id == character_id)
    {
        &mut enemy.status_conditions
    } else {
        panic!("Character not found");
    };

    // TODO: reason調整が必要?
    let mut incident = BattleCharacterIncident::new(BattleCharacterIncidentReason::TurnEndRecovery);
    let mut finished_conditions = vec![];
    for (index, es) in status_conditions.iter_mut().enumerate() {
        if let BattleStatusConditionDuration::Turn(turn_duration) = &mut es.duration {
            turn_duration.elapsed_turns += 1;
            if turn_duration.elapsed_turns >= turn_duration.turns {
                // 効果ターン終了の状態変化インシデント
                finished_conditions.push(index);

                incident.add_concrete(BattleCharacterIncidentConcrete::StatusConditionRemoved(
                    BattleIncidentStatusConditionRemoved {
                        status_condition: es.clone(),
                    },
                ));
            }
        }
    }
    for index in finished_conditions.iter() {
        status_conditions.remove(*index);
    }

    BattleIncidentCharacter {
        character_id,
        incidents: vec![incident],
    }
}
