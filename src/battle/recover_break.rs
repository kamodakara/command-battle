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
            status_effects: vec![],
        };
    } else if let Some(enemy) = battle
        .enemies
        .iter_mut()
        .find(|c| c.character_id == request.character_id)
    {
        // 敵キャラクターの場合のブレイク回復処理

        // ブレイク状態かどうか
        let mut is_break = false;
        let mut break_status_effect_index = 0;
        for (index, se) in enemy.base.status_effects.iter().enumerate() {
            if let StatusEffectPotency::Break(_) = &se.potency {
                is_break = true;
                break_status_effect_index = index;
            }
        }

        let break_max_turns = enemy.current_enemy_only_stats.max_break_turn;
        let break_turns = enemy.current_enemy_only_stats.break_turns;
        let mut stats_change_incidents = vec![];
        let mut status_effect_incidents = vec![];
        if is_break {
            // ブレイク中

            // ブレイク回復処理
            if break_turns >= break_max_turns {
                // ブレイク状態回復
                enemy.current_enemy_only_stats.break_turns = 0;
                enemy.current_enemy_only_stats.current_break =
                    enemy.current_enemy_only_stats.max_break;

                // ステータス効果からブレイクを削除
                let battle_status_effect =
                    enemy.base.status_effects.remove(break_status_effect_index);

                // ブレイク回復インシデント
                status_effect_incidents.push(BattleIncidentStatusEffect {
                    status_effect: battle_status_effect,
                    status_effect_handling: BattleIncidentStatusEffectHandling::Removed(
                        BattleIncidentStatusEffectRemoved {},
                    ),
                });
            } else {
                // ブレイクターンを進める
                enemy.current_enemy_only_stats.break_turns += 1;
            }
        } else {
            print!("Not in break state, recovering break.");

            // 2ターンブレイクダメージを受けていなければ回復
            if enemy.current_enemy_only_stats.break_not_damaged_turns >= 2 {
                let break_recovery = enemy.current_enemy_only_stats.break_recovery;
                let (brefore_break, after_break) =
                    enemy.current_enemy_only_stats.break_add(break_recovery);
                stats_change_incidents.push(BattleIncidentStats::RecoverBreak(
                    BattleIncidentRecoverBreak {
                        recover: break_recovery,
                        before: brefore_break,
                        after: after_break,
                    },
                ));
            }

            // ブレイクを受けていないターン数を増やす
            enemy.current_enemy_only_stats.break_not_damaged_turns += 1;
        }

        return BattleIncidentAutoTrigger {
            character_id: request.character_id,
            stats_changes: stats_change_incidents,
            status_effects: status_effect_incidents,
        };
    }

    // キャラクターが見つからなかった場合はエラー
    // TODO: エラー処理
    panic!("Character not found");
}
