use std::vec;

use super::*;

pub fn turn_end(battle: &mut Battle) -> Vec<BattleIncidentCharacter> {
    let mut incident_characters = vec![];

    // プレイヤー
    let player = &mut battle.player;

    // 現在の効果を取得
    let effects = player.current_effects();

    // コンビネーションのターン終了処理
    if let Some(combination_skill) = &mut player.combination_skill {
        // コンビネーションのターン終了処理
        combination_skill.finalize_current_conduct();
    }
    if let Some(trance) = &mut player.trance {
        // トランスのターン終了の減少処理
        // 現在、減少量はトランス値の10分の1(切り捨て)
        let reduce = (trance.current_trance as f32 / 10.0) as u32;
        trance.reduce_trance(reduce);
    }

    // プレイヤーの状態変化更新
    incident_characters.push(update_status_condition(player));

    // プレイヤーのスタミナ回復
    incident_characters.push(recover_stamina(player, &effects));
    // カルマのターン終了時処理
    karma(player);
    // TODO: カルマのインシデント追加

    // 状態異常の継続ダメージ
    incident_characters.push(apply_ongoing_ailment_damage(player));

    // 状態異常終了
    incident_characters.push(recover_character_status_ailments(player));

    // ====================
    // 敵キャラクター
    // 生存中の敵キャラクターに対して処理を行う
    let mut enemies = battle.alive_enemies_mut();
    for enemy in &mut enemies {
        if let Some(combination_skill) = &mut enemy.combination_skill {
            // コンビネーションのターン終了処理
            combination_skill.finalize_current_conduct();
        }

        // 敵の状態変化更新
        incident_characters.push(update_status_condition(enemy));

        // 敵の状態異常の継続ダメージ
        incident_characters.push(apply_ongoing_ailment_damage(enemy));

        // 敵キャラクターの状態異常自然回復処理
        incident_characters.push(recover_character_status_ailments(enemy));
    }

    incident_characters
}

// 状態変化の更新
pub fn update_status_condition(character: &mut BattleCharacter) -> BattleIncidentCharacter {
    let status_conditions = &mut character.status_conditions;

    // TODO: reason調整が必要?
    for (_, es) in status_conditions.iter_mut().enumerate() {
        if let BattleStatusConditionDuration::Turn(turn_duration) = &mut es.duration {
            // ターン経過を増やす
            turn_duration.elapsed_turns += 1;
        }
    }
    // 終了する状態変化とのこる状態変化を分ける
    let (to_remove, to_keep): (Vec<BattleStatusCondition>, Vec<BattleStatusCondition>) =
        status_conditions.iter().cloned().partition(|conditions| {
            if let BattleStatusConditionDuration::Turn(turn_duration) = &conditions.duration {
                turn_duration.elapsed_turns >= turn_duration.turns
            } else {
                false
            }
        });
    character.status_conditions = to_keep;

    let mut incident = BattleCharacterIncident::new(BattleCharacterIncidentReason::TurnEndRecovery);
    for condition in to_remove {
        incident.add_concrete(BattleCharacterIncidentConcrete::StatusConditionRemoved(
            BattleIncidentStatusConditionRemoved {
                status_condition: condition,
            },
        ));
    }

    BattleIncidentCharacter {
        character_id: character.character_id,
        incidents: vec![incident],
    }
}

// スタミナ回復
fn recover_stamina(player: &mut BattleCharacter, effects: &Vec<Effect>) -> BattleIncidentCharacter {
    // スタミナ回復
    let mut stamina_recovery = player.stamina.stamina_recovery;
    // 効果によるスタミナ回復量の補正
    for effect in effects {
        match effect {
            Effect::StaminaRecoveryModifier(e) => {
                stamina_recovery = (stamina_recovery as f32 * e.modifier) as u32;
            }
            _ => {}
        }
    }

    // スタミナ回復処理
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
        if card.remaining_turns > 0 {
            card.remaining_turns -= 1;
        }
    }

    // ターン終了時にターン数が0になったカードを捨て札に移動
    let (to_discard, to_keep): (Vec<BattleKarmaCard>, Vec<BattleKarmaCard>) = karma
        .field_cards
        .drain(..)
        .partition(|card| card.remaining_turns == 0);

    // 捨て札に移動
    let discard_cards = to_discard.into_iter().map(|card| card.card);
    karma.discard_pile.extend(discard_cards);
    // TODO: カルマカードが捨て札に移動したインシデントの追加

    // 残りのカードを場に戻す
    karma.field_cards = to_keep;

    // カルマコストを超えている場合ペナルティ処理
    // 場のカルマコストの総数
    let total_karma_cost: u32 = karma.field_cards.iter().map(|card| card.card.cost).sum();
    let max_karma = player.max_karma();
    if total_karma_cost > max_karma {
        // HP、SPに最大値の4分の1のダメージを与える
        let penalty_damage = player.hp.max_hp / 4;
        let (before_hp, after_hp, is_dead) = player.hp.damage(penalty_damage);
        let penalty_sp_damage = player.sp.max_sp / 4;
        let (before_sp, after_sp) = player.sp.damage(penalty_sp_damage);

        // TODO: カルマコスト超過ペナルティのインシデントの追加
        // HPダメージのインシデント
        // SPダメージのインシデント
        if is_dead {
            // TODO: 死亡インシデントの追加
        }
    }
}

// 状態異常の継続ダメージ（HP・SP割合ダメージ）
fn apply_ongoing_ailment_damage(character: &mut BattleCharacter) -> BattleIncidentCharacter {
    let mut incident = BattleCharacterIncident::new(BattleCharacterIncidentReason::TurnEndRecovery);

    let ongoing_effects = character.status_ailment.current_ongoing_effects();
    for effect in ongoing_effects {
        match effect {
            BattleStatusAilmentOngoingEffect::HpPercentageDamage(e) => {
                let damage = (character.hp.max_hp as f32 * e.percentage / 100.0) as u32;
                let (before_hp, after_hp, is_dead) = character.hp.damage(damage);
                incident.add_concrete(BattleCharacterIncidentConcrete::DamageHp(
                    BattleIncidentDamageHp::new(damage, before_hp, after_hp),
                ));
                if is_dead {
                    incident.add_concrete(BattleCharacterIncidentConcrete::Death(
                        BattleIncidentDeath {},
                    ));
                }
            }
            BattleStatusAilmentOngoingEffect::SpPercentageDamage(e) => {
                let damage = (character.sp.max_sp as f32 * e.percentage / 100.0) as u32;
                let (before_sp, after_sp) = character.sp.damage(damage);
                incident.add_concrete(BattleCharacterIncidentConcrete::DamageSp(
                    BattleIncidentDamageSp::new(damage, before_sp, after_sp),
                ));
            }
            _ => {
                // AbilityModifier・ReceiveDamageModifier 等は current_effects() 経由で適用
            }
        }
    }

    BattleIncidentCharacter {
        character_id: character.character_id,
        incidents: vec![incident],
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

    // ブレイク
    let breaking_incidents = recover_ailment_status(
        StatusAilment::Breaking,
        &mut character.status_ailment.breaking,
    );
    incident.extend_concretes(breaking_incidents);

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
        if status.no_accumulation_turns >= 1 {
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
