use super::*;

pub struct RecoverBreakRequest {
    pub character_id: BattleCharacterId,
}

// ブレイク回復処理
pub fn recover_break(
    battle: &mut Battle,
    request: RecoverBreakRequest,
) -> BattleIncidentAutoTrigger {
    if let Some(_player) = battle
        .players
        .iter_mut()
        .find(|c| c.character_id == request.character_id)
    {
        // プレイヤーキャラクターの場合は何もしない

        return BattleIncidentAutoTrigger {
            character_id: request.character_id,
            stats_changes: vec![],
            status_conditions: vec![],
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

        let mut stats_change_incidents = vec![];
        let mut status_condition_incidents = vec![];
        if is_break {
            // ブレイク中

            // ブレイク回復処理
            if enemy.break_resistance.remaining_breaking_turns == 0 {
                // ブレイク中解除
                enemy.break_resistance.clear_breaking();

                // ステータス効果からブレイクを削除
                let battle_status_condition =
                    enemy.status_conditions.remove(break_status_condition_index);

                // ブレイク回復インシデント
                status_condition_incidents.push(BattleIncidentStatusCondition {
                    status_condition: battle_status_condition,
                    status_condition_handling: BattleIncidentStatusConditionHandling::Removed(
                        BattleIncidentStatusConditionRemoved {},
                    ),
                });
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
                let (brefore_break, after_break) = enemy.break_resistance.recover(break_recovery);
                stats_change_incidents.push(BattleIncidentStats::RecoverBreak(
                    BattleIncidentRecoverBreak {
                        recover: break_recovery,
                        before: brefore_break,
                        after: after_break,
                    },
                ));
            }

            // ブレイクを受けていないターン数を増やす
            enemy.break_resistance.break_not_damaged_turns += 1;
        }

        return BattleIncidentAutoTrigger {
            character_id: request.character_id,
            stats_changes: stats_change_incidents,
            status_conditions: status_condition_incidents,
        };
    }

    // キャラクターが見つからなかった場合はエラー
    // TODO: エラー処理
    panic!("Character not found");
}
