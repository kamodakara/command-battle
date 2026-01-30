use super::*;

pub struct RecoverBreakRequest {
    pub character_id: BattleCharacterId,
}

// ブレイク回復処理
pub fn recover_break(battle: &mut Battle, request: RecoverBreakRequest) -> BattleIncidentCharacter {
    if let Some(_player) = battle
        .players
        .iter_mut()
        .find(|c| c.character_id == request.character_id)
    {
        // プレイヤーキャラクターの場合は何もしない

        return BattleIncidentCharacter {
            character_id: request.character_id,
            incidents: vec![],
        };
    } else if let Some(enemy) = battle
        .enemies
        .iter_mut()
        .find(|c| c.character_id == request.character_id)
    {
        // 敵キャラクターの場合のブレイク回復処理

        // ブレイク状態かどうか
        let mut is_break = false;
        let mut break_status_condition_index = 0;
        for (index, se) in enemy.status_conditions.iter().enumerate() {
            if let StatusConditionPotency::Break(_) = &se.potency {
                is_break = true;
                break_status_condition_index = index;
            }
        }

        let mut incident =
            BattleCharacterIncident::new(BattleCharacterIncidentReason::TurnEndRecovery);
        if is_break {
            // ブレイク中

            // ブレイク回復処理
            if enemy.break_resistance.remaining_breaking_turns == 0 {
                // ブレイク中解除
                enemy.break_resistance.clear_breaking();

                // ステータス効果からブレイクを削除
                let battle_status_condition =
                    enemy.status_conditions.remove(break_status_condition_index);

                // ブレイク状態回復インシデント
                incident.add_concrete(BattleCharacterIncidentConcrete::StatusConditionRemoved(
                    BattleIncidentStatusConditionRemoved {
                        status_condition: battle_status_condition,
                    },
                ));
            } else {
                // ブレイク中ターン経過
                let (_before_breaking_turns, _after_breaking_turns) =
                    enemy.break_resistance.elapse_breaking_turn();
            }
        } else {
            print!("Not in break state, recovering break.");

            // 2ターンブレイクダメージを受けていなければ回復
            if enemy.break_resistance.break_not_damaged_turns >= 2 {
                let break_recovery = enemy.break_resistance.break_recovery;
                let (before_break, after_break) = enemy.break_resistance.recover(break_recovery);

                // ブレイク値回復インシデント
                incident.add_concrete(BattleCharacterIncidentConcrete::RecoverBreak(
                    BattleIncidentRecoverBreak::new(break_recovery, before_break, after_break),
                ));
            }

            // ブレイクを受けていないターン数を増やす
            enemy.break_resistance.break_not_damaged_turns += 1;
        }

        return BattleIncidentCharacter {
            character_id: request.character_id,
            incidents: vec![incident],
        };
    }

    // キャラクターが見つからなかった場合はエラー
    // TODO: エラー処理
    panic!("Character not found");
}
