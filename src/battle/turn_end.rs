use std::vec;

use super::*;

pub fn turn_end(battle: &mut Battle) -> Vec<BattleIncidentCharacter> {
    let mut incident_characters = vec![];

    // 全キャラクターの状態変化更新
    for player in &mut battle.players {
        let incident = update_status_condition(player);
        incident_characters.push(incident);
    }
    for enemy in &mut battle.enemies {
        let incident = update_status_condition(enemy);
        incident_characters.push(incident);
    }

    // 全敵キャラクターのブレイク回復
    for enemy in &mut battle.enemies {
        let incident = recover_break(enemy);
        incident_characters.push(incident);
    }

    // 全プレイヤーキャラクターのスタミナ回復
    for player in &mut battle.players {
        let incident = recover_stamina(player);
        incident_characters.push(incident);
    }

    // TODO: 仮
    let player = &mut battle.players.first_mut().unwrap();
    // カルマのターン終了時処理
    karma(player);

    // 状態異常終了
    for player in &mut battle.players {
        incident_characters.push(recover_character_status_ailments(player));
    }
    // 敵キャラクターの状態異常終了
    for enemy in &mut battle.enemies {
        incident_characters.push(recover_character_status_ailments(enemy));
    }

    incident_characters
}

// 状態変化の更新
pub fn update_status_condition(character: &mut BattleCharacter) -> BattleIncidentCharacter {
    let status_conditions = &mut character.status_conditions;

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
        character_id: character.character_id,
        incidents: vec![incident],
    }
}

// ブレイク回復処理
pub fn recover_break(enemy: &mut BattleCharacter) -> BattleIncidentCharacter {
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

    let mut incident = BattleCharacterIncident::new(BattleCharacterIncidentReason::TurnEndRecovery);

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
        // 2ターンブレイクダメージを受けていなければ回復
        if enemy.break_resistance.break_not_damaged_turns >= 2 {
            let break_recovery = enemy.break_resistance.break_recovery;
            let (brefore_break, after_break) = enemy.break_resistance.recover(break_recovery);

            // ブレイク値回復インシデント
            incident.add_concrete(BattleCharacterIncidentConcrete::RecoverBreak(
                BattleIncidentRecoverBreak::new(break_recovery, brefore_break, after_break),
            ));
        }

        // ブレイクを受けていないターン数を増やす
        enemy.break_resistance.break_not_damaged_turns += 1;
    }

    BattleIncidentCharacter {
        character_id: enemy.character_id,
        incidents: vec![incident],
    }
}

// スタミナ回復
fn recover_stamina(player: &mut BattleCharacter) -> BattleIncidentCharacter {
    // スタミナ回復
    let stamina_recovery = player.stamina.stamina_recovery;
    let (before_stamina, after_stamina) = player.stamina.recover(stamina_recovery);

    let mut incident = BattleCharacterIncident::new(BattleCharacterIncidentReason::TurnEndRecovery);
    incident.add_concrete(BattleCharacterIncidentConcrete::RecoverStamina(
        BattleIncidentRecoverStamina::new(stamina_recovery, before_stamina, after_stamina),
    ));

    BattleIncidentCharacter {
        character_id: player.character_id,
        incidents: vec![incident],
    }
}

fn karma(player: &mut BattleCharacter) {
    let karma = if let Some(karma) = player.karma.as_mut() {
        karma
    } else {
        // TODO: エラーハンドリングを追加
        panic!("Player does not have karma");
    };

    // 場のカルマカードのターン経過処理
    for card in &mut karma.field_cards {
        if card.max_turn > 0 {
            card.max_turn -= 1;
        }
    }

    // ターン終了時にターン数が0になったカードを捨て札に移動
    let (to_discard, to_keep): (Vec<KarmaCard>, Vec<KarmaCard>) = karma
        .field_cards
        .drain(..)
        .partition(|card| card.max_turn == 0);

    // 捨て札に移動
    karma.discard_pile.extend(to_discard);
    // TODO: カルマカードが捨て札に移動したインシデントの追加

    // 残りのカードを場に戻す
    karma.field_cards = to_keep;

    // カルマコストを超えている場合ペナルティ処理
    // 場のカルマコストの総数
    let total_karma_cost: u32 = karma.field_cards.iter().map(|card| card.cost).sum();
    let max_karma = player.max_karma();
    if total_karma_cost > max_karma {
        // HP、SPに最大値の4分の1のダメージを与える
        let penalty_damage = player.hp.max_hp / 4;
        let (before_hp, after_hp) = player.hp.damage(penalty_damage);
        let penalty_sp_damage = player.sp.max_sp / 4;
        let (before_sp, after_sp) = player.sp.damage(penalty_sp_damage);

        // TODO: カルマコスト超過ペナルティのインシデントの追加
    }
}

// キャラクターの状態異常回復
fn recover_character_status_ailments(character: &mut BattleCharacter) -> BattleIncidentCharacter {
    let mut incident = BattleCharacterIncident {
        reason: BattleCharacterIncidentReason::TurnEndRecovery,
        concretes: vec![],
    };

    // 毒
    let poison_incidents =
        recover_ailment_status(StatusAilment::Poison, &mut character.status_ailment.poison);
    incident.extend_concretes(poison_incidents);

    // 眠気
    let sleep_incidents =
        recover_ailment_status(StatusAilment::Sleep, &mut character.status_ailment.sleep);
    incident.extend_concretes(sleep_incidents);

    // 寒気
    let chill_incidents =
        recover_ailment_status(StatusAilment::Chill, &mut character.status_ailment.chill);
    incident.extend_concretes(chill_incidents);

    // 出血
    let bleed_incidents =
        recover_ailment_status(StatusAilment::Bleed, &mut character.status_ailment.bleed);
    incident.extend_concretes(bleed_incidents);

    // 火傷
    let burn_incidents =
        recover_ailment_status(StatusAilment::Burn, &mut character.status_ailment.burn);
    incident.extend_concretes(burn_incidents);

    // 麻痺
    let paralysis_incidents = recover_ailment_status(
        StatusAilment::Paralysis,
        &mut character.status_ailment.paralysis,
    );
    incident.extend_concretes(paralysis_incidents);
    // 恐怖
    let fear_incidents =
        recover_ailment_status(StatusAilment::Fear, &mut character.status_ailment.fear);
    incident.extend_concretes(fear_incidents);
    // 激昂
    let rage_incidents =
        recover_ailment_status(StatusAilment::Rage, &mut character.status_ailment.rage);
    incident.extend_concretes(rage_incidents);

    BattleIncidentCharacter {
        character_id: character.character_id,
        incidents: vec![incident],
    }
}

//　状態異常回復
fn recover_ailment_status(
    status_ailment: StatusAilment,
    status: &mut BattleStatusAilmentStatus,
) -> Vec<BattleCharacterIncidentConcrete> {
    if status.accumulation == 0 {
        // 蓄積量が0なら何もしない
        return vec![];
    }

    let mut incidents = vec![];
    if status.is_ailment {
        // 状態異常中

        // 状態異常中は割合で回復
        let recover = (status.max_accumulation as f32 * status.ailment_recovery_rate) as u32;
        let (before_accumulation, after_accumulation) = status.recover_accumulation(recover);

        // インシデント
        incidents.push(BattleCharacterIncidentConcrete::StatusAilmentRecovery(
            BattleIncidentStatusAilmentRecovery {
                status_ailment: status_ailment.clone(),
                recover,
                before_accumulation,
                after_accumulation,
            },
        ));

        if status.accumulation == 0 {
            // 状態異常解除
            status.is_ailment = false;

            incidents.push(BattleCharacterIncidentConcrete::StatusAilmentRemoved(
                BattleIncidentStatusAilmentRemoved { status_ailment },
            ));
        }
    } else {
        // 2ターン状態異常値の蓄積がない場合に回復
        if status.no_accumulation_turns >= 2 {
            let recover_amount = status.recovery_amount;
            let (before_accumulation, after_accumulation) =
                status.recover_accumulation(recover_amount);

            // インシデント
            incidents.push(BattleCharacterIncidentConcrete::StatusAilmentRecovery(
                BattleIncidentStatusAilmentRecovery {
                    status_ailment,
                    recover: recover_amount,
                    before_accumulation,
                    after_accumulation,
                },
            ))
        }

        // 状態異常値の蓄積がないターン数を増やす
        status.no_accumulation_turns += 1;
    }

    incidents
}
