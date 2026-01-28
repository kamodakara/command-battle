mod conduct_effect;

use super::*;

struct AttackerData {
    character_id: BattleCharacterId,
    weapon_performance: WeaponPerformance,
    // TODO: 他に必要なデータがあれば追加
}

pub struct BattleExecuteConductRequest {
    pub conduct: BattleConduct,
}

// 行動実行
pub fn execute_conduct(
    battle: &mut Battle,
    request: BattleExecuteConductRequest,
) -> BattleIncidentConduct {
    let conduct = request.conduct;

    // 行動者の決定
    let attacker = if let Some(character) = battle.character_mut(&conduct.actor_character_id) {
        character
    } else {
        panic!("Attacker not found");
    };
    let attacker_id = attacker.character_id;

    // 行動成否判定
    if let Some(failure_reason) = determine_action_outcome_failure(&conduct, &attacker) {
        // TODO: 不発理由に応じた処理
        return BattleIncidentConduct {
            actor_character_id: attacker_id,
            target: conduct.target.clone(),
            conduct,
            outcome: BattleIncidentConductOutcome::Failure(BattleIncidentConductOutcomeFailure {
                reason: failure_reason,
            }),
        };
    }

    // 攻撃者インシデントの準備
    let mut attacker_incident_character = BattleIncidentCharacter::new(attacker_id);
    let mut attacker_incident =
        BattleCharacterIncident::new(BattleCharacterIncidentReason::ConductConsumption);

    // SP消費
    let sp_cost = conduct.art.sp_cost;
    let (before_sp, after_sp) = attacker.sp.damage(sp_cost);
    // インシデント
    attacker_incident.add_concrete(BattleCharacterIncidentConcrete::DamageSp(
        BattleIncidentDamageSp::new(sp_cost, before_sp, after_sp),
    ));

    // スタミナ消費
    if attacker.character_type == BattleCharacterType::Player {
        // プレイヤーの場合のみスタミナ消費処理
        let stamina_cost = conduct.art.stamina_cost;
        let (before_stamina, after_stamina) = attacker.stamina.damage(stamina_cost);
        // インシデント
        attacker_incident.add_concrete(BattleCharacterIncidentConcrete::DamageStamina(
            BattleIncidentDamageStamina::new(stamina_cost, before_stamina, after_stamina),
        ));
    }

    // 能力補正済みの武器性能取得
    let attacker_weapon_performance = if let Some(battle_weapon_id) = &conduct.battle_weapon_id {
        if let Some(performance) = attacker.weapon_performance(&battle_weapon_id) {
            performance
        } else {
            // 指定された武器の性能が見つからなかった場合
            // TODO: エラー処理
            panic!("Attacker weapon performance not found");
        }
    } else {
        // 武器なし
        unarmed_weapon_performance()
    };
    // 攻撃者データ準備
    let attacker_data = AttackerData {
        character_id: attacker_id,
        weapon_performance: attacker_weapon_performance,
    };

    let sorcery_power = attacker_data.weapon_performance.final_sorcery_power();
    // 効果ランク判定
    let rank = conduct.art.effective_rank(sorcery_power);

    // ターゲットの決定
    let target_character_ids = determine_targets(battle, &conduct.target, &rank.target);

    // ターゲットごとに効果処理
    let mut target_incidents = Vec::new();
    for target_id in target_character_ids.iter() {
        let target = if let Some(character) = battle.character_mut(&target_id) {
            character
        } else {
            // TODO: エラー処理
            panic!("Target not found");
        };

        let mut target_incident_character = BattleIncidentCharacter::new(target.character_id);

        // 回避判定
        let mut is_evaded = false;
        for se in target.status_conditions.iter() {
            match &se.potency {
                StatusConditionPotency::Evasion => {
                    // 回避効果処理
                    is_evaded = true;
                    break;
                }
                StatusConditionPotency::Airborne => {
                    // 空中効果処理
                    // 遠距離攻撃でない時は回避
                    if !conduct.art.perks.contains(&ArtPerk::Ranged) {
                        is_evaded = true;
                        break;
                    }
                }
                StatusConditionPotency::Floating => {
                    // 浮遊効果処理
                    // 足元攻撃は回避
                    if conduct.art.perks.contains(&ArtPerk::AtFeet) {
                        is_evaded = true;
                        break;
                    }
                }
                StatusConditionPotency::Ranged => {
                    // 遠距離効果処理
                    // 近距離の攻撃を回避
                    if !conduct.art.perks.contains(&ArtPerk::Ranged) {
                        is_evaded = true;
                        break;
                    }
                }
                _ => {
                    // その他
                }
            }
        }
        if is_evaded {
            // 回避された場合の処理
            // 防御者インシデント作成
            target_incidents.push(BattleIncidentConductOutcomeSuccessDefender {
                character: target_incident_character,
                is_evaded: true,
                is_defended: false,
                is_dead: false,
            });
            // 回避して、効果処理は行わない
            continue;
        }

        // 発生インシデント
        let mut target_character_incident =
            BattleCharacterIncident::new(BattleCharacterIncidentReason::ConductEffect);

        // 効果処理
        match &rank.potency {
            ArtPotency::Attack(art_attack) => {
                // 武器性能取得
                let weapon_attack_power = attacker_data.weapon_performance.final_attack_power();
                let weapon_break_power = attacker_data.weapon_performance.final_break_power();

                // アーツ攻撃力算出
                let mut attack_power = art_attack.final_attack_power(&weapon_attack_power);

                // 術力補正
                if conduct.art.art_type == ArtType::Sorcery {
                    // 魔法タイプの場合、術力補正をかける
                    let sorcery_attack_power_rate = 1.0 + (sorcery_power as f32 / 100.0);
                    attack_power.multiply(sorcery_attack_power_rate);
                }

                // 防御時の攻撃力カット処理
                let mut is_defended = false;
                for se in target.status_conditions.iter() {
                    match &se.potency {
                        StatusConditionPotency::Resistance(resistance) => {
                            // 防御時の攻撃力カット処理
                            attack_power = resistance.cut_rate.apply_guard_cut(&attack_power);
                            is_defended = true;
                        }
                        _ => {
                            // その他
                        }
                    }
                }

                // ブレイク力算出
                let break_power = art_attack.final_break_power(weapon_break_power);

                // ダメージ
                let damage = calc_damage(&attack_power, &target.defense_power());
                // TODO: 攻撃側のダメージ補正
                // TODO: 防御側のダメージ補正
                let (before_hp_damage, after_hp_damage) = target.hp.damage(damage);
                // HPダメージのインシデント
                target_character_incident.add_concrete(BattleCharacterIncidentConcrete::DamageHp(
                    BattleIncidentDamageHp::new(damage, before_hp_damage, after_hp_damage),
                ));

                // 防御時のスタミナダメージ
                if is_defended {
                    let sta_damage = break_power / 4; // TODO: 固定値ではなくガード強度
                    let (before_sta, after_sta) = target.stamina.damage(sta_damage);

                    // スタミナダメージのインシデント
                    target_character_incident.add_concrete(
                        BattleCharacterIncidentConcrete::DamageStamina(
                            BattleIncidentDamageStamina::new(sta_damage, before_sta, after_sta),
                        ),
                    );
                }

                // ブレイクダメージ処理
                if target.character_type == BattleCharacterType::Enemy {
                    // ブレイク中でない時
                    let mut is_break = false;
                    for se in target.status_conditions.iter() {
                        if let StatusConditionPotency::Break(_) = &se.potency {
                            is_break = true
                        }
                    }
                    if !is_break {
                        // 敵のブレイクダメージ処理
                        let (before_break, after_break) =
                            target.break_resistance.damage(break_power);

                        if after_break == 0 {
                            // ブレイク状態にする
                            support_status_effect(
                                &vec![StatusCondition {
                                    potency: StatusConditionPotency::Break(StatusConditionBreak {}),
                                    duration: StatusConditionDuration::Permanent,
                                }],
                                target,
                                &mut target_character_incident,
                            );
                        }

                        // ブレイクダメージインシデント追加
                        target_character_incident.add_concrete(
                            BattleCharacterIncidentConcrete::DamageBreak(
                                BattleIncidentDamageBreak::new(
                                    break_power,
                                    before_break,
                                    after_break,
                                ),
                            ),
                        );
                    }
                }

                target_incident_character.add_incident(target_character_incident);
                target_incidents.push(BattleIncidentConductOutcomeSuccessDefender {
                    character: target_incident_character,
                    is_defended,
                    is_evaded: false,
                    is_dead: target.hp.current_hp == 0, // TODO: 戦闘不能判定
                });
            }
            ArtPotency::Support(support) => {
                // 支援処理
                match &support {
                    ArtPotencySupport::StatusCondition(status_condition) => {
                        // 支援状態変化処理
                        support_status_effect(
                            &status_condition.status_conditions,
                            target,
                            &mut target_character_incident,
                        );

                        target_incident_character.add_incident(target_character_incident);
                        target_incidents.push(BattleIncidentConductOutcomeSuccessDefender {
                            character: target_incident_character,
                            is_defended: false,
                            is_evaded: false,
                            is_dead: false,
                        });
                    }
                    ArtPotencySupport::Recover(recover) => {
                        support_recover(recover, target, &mut target_character_incident);

                        target_incident_character.add_incident(target_character_incident);
                        target_incidents.push(BattleIncidentConductOutcomeSuccessDefender {
                            character: target_incident_character,
                            is_defended: false,
                            is_evaded: false,
                            is_dead: false,
                        })
                    }
                }
            }
        }
    }

    attacker_incident_character.add_incident(attacker_incident);
    BattleIncidentConduct {
        actor_character_id: attacker_id,
        target: conduct.target.clone(),
        conduct,
        outcome: BattleIncidentConductOutcome::Success(BattleIncidentConductOutcomeSuccess {
            attacker: attacker_incident_character,
            defenders: target_incidents,
        }),
    }
}

// ターゲット決定
// ターゲット範囲が変化している場合、それに応じてターゲットを変更
fn determine_targets(
    battle: &Battle,
    conduct_target: &BattleConductTargetType,
    art_target: &ArtTarget,
) -> Vec<BattleCharacterId> {
    let conduct_target = match conduct_target {
        BattleConductTargetType::PlayerSingle(_) => {
            if art_target == &ArtTarget::All {
                &BattleConductTargetType::PlayerAll
            } else {
                conduct_target
            }
        }
        BattleConductTargetType::EnemySingle(_) => {
            if art_target == &ArtTarget::All {
                &BattleConductTargetType::EnemyAll
            } else {
                conduct_target
            }
        }
        BattleConductTargetType::PlayerAll => {
            if art_target == &ArtTarget::Single {
                if let Some(character) = battle.players.first() {
                    let target_character_id = character.character_id;
                    &BattleConductTargetType::PlayerSingle(target_character_id)
                } else {
                    // TODO: エラー処理
                    panic!("No player characters available");
                }
            } else {
                conduct_target
            }
        }
        BattleConductTargetType::EnemyAll => {
            if art_target == &ArtTarget::Single {
                if let Some(character) = battle.enemies.first() {
                    let target_character_id = character.character_id;
                    &BattleConductTargetType::EnemySingle(target_character_id)
                } else {
                    // TODO: エラー処理
                    panic!("No enemy characters available");
                }
            } else {
                conduct_target
            }
        }
    };

    // ターゲットIDリスト取得
    let target_character_ids = match conduct_target {
        BattleConductTargetType::PlayerSingle(character_id) => {
            if let Some(character) = battle.character(&character_id) {
                vec![character.character_id]
            } else {
                // TODO: エラー処理
                panic!("Defender not found");
            }
        }
        BattleConductTargetType::EnemySingle(character_id) => {
            if let Some(character) = battle.character(&character_id) {
                vec![character.character_id]
            } else {
                // TODO: エラー処理
                panic!("Defender not found");
            }
        }
        BattleConductTargetType::PlayerAll => {
            // playersのcharacter_id全て
            battle.players.iter().map(|c| c.character_id).collect()
        }
        BattleConductTargetType::EnemyAll => {
            // enemiesのcharacter_id全て
            battle.enemies.iter().map(|c| c.character_id).collect()
        }
    };

    target_character_ids
}

// 素手の攻撃性能取得
fn unarmed_weapon_performance() -> WeaponPerformance {
    // TODO: 仮
    WeaponPerformance {
        attack_power: AttackPower {
            slash: 0,
            strike: 10,
            thrust: 0,
            impact: 0,
            magic: 0,
            fire: 0,
            lightning: 0,
            chaos: 0,
        },
        ability_attack_power: AttackPower::default(),
        sorcery_power: 0,
        ability_sorcery_power: 0,
        break_power: 0,
        ability_break_power: 0,
        guard_strength: 10,
        penalty: None,
    }
}

// ダメージ計算
fn calc_damage(attack_power: &AttackPower, defender: &DefensePower) -> u32 {
    let damage = (attack_power.slash as f32 / defender.slash as f32)
        + (attack_power.strike as f32 / defender.strike as f32)
        + (attack_power.thrust as f32 / defender.thrust as f32)
        + (attack_power.impact as f32 / defender.impact as f32)
        + (attack_power.magic as f32 / defender.magic as f32)
        + (attack_power.fire as f32 / defender.fire as f32)
        + (attack_power.lightning as f32 / defender.lightning as f32)
        + (attack_power.chaos as f32 / defender.chaos as f32);
    damage as u32
}

fn support_status_effect(
    status_conditions: &Vec<StatusCondition>,
    target: &mut BattleCharacter,
    incident: &mut BattleCharacterIncident,
) {
    // 支援行動処理
    for status_condition in status_conditions {
        // 状態変化付与処理
        let battle_status_condition = create_battle_status_condition(status_condition);
        // 状態変化付与
        // TODO: 状態変化の重複処理
        target
            .status_conditions
            .push(battle_status_condition.clone());

        // インシデント追加
        incident.add_concrete(BattleCharacterIncidentConcrete::StatusConditionApplied(
            BattleIncidentStatusConditionApplied {
                status_condition: battle_status_condition,
            },
        ));
    }
}

fn support_recover(
    recover: &ArtPotencySupportRecover,
    target: &mut BattleCharacter,
    incident: &mut BattleCharacterIncident,
) {
    // 支援回復処理
    for potency in &recover.potencies {
        match potency {
            SupportRecoverPotency::Hp(hp_recover) => {
                let hp_rcv = hp_recover.hp_recover;
                let (before_hp, after_hp) = target.hp.recover(hp_rcv);

                // HP回復のインシデント
                incident.add_concrete(BattleCharacterIncidentConcrete::RecoverHp(
                    BattleIncidentRecoverHp::new(hp_rcv, before_hp, after_hp),
                ));
            }
            SupportRecoverPotency::Sp(sp_recover) => {
                let sp_rcv = sp_recover.sp_recover;
                let (before_sp, after_sp) = target.sp.recover(sp_rcv);

                // SP回復のインシデント
                incident.add_concrete(BattleCharacterIncidentConcrete::RecoverSp(
                    BattleIncidentRecoverSp::new(sp_rcv, before_sp, after_sp),
                ));
            }
            SupportRecoverPotency::Stamina(stamina_recover) => {
                // スタミナ回復処理はプレイヤーキャラクターのみ
                if target.character_type == BattleCharacterType::Player {
                    let stamina_rcv = stamina_recover.stamina_recover;
                    let (before_stamina, after_stamina) = target.stamina.recover(stamina_rcv);

                    // スタミナ回復のインシデント
                    incident.add_concrete(BattleCharacterIncidentConcrete::RecoverStamina(
                        BattleIncidentRecoverStamina::new(
                            stamina_rcv,
                            before_stamina,
                            after_stamina,
                        ),
                    ));
                }
            }
        }
    }
}

fn create_battle_status_condition(status_condition: &StatusCondition) -> BattleStatusCondition {
    let duration = match &status_condition.duration {
        StatusConditionDuration::Permanent => BattleStatusConditionDuration::Permanent,
        StatusConditionDuration::Turn(d) => {
            BattleStatusConditionDuration::Turn(BattleStatusConditionDurationTurn {
                turns: d.turns,
                elapsed_turns: 0,
            })
        }
        StatusConditionDuration::Count(d) => {
            BattleStatusConditionDuration::Count(BattleStatusConditionDurationCount {
                count: d.count,
                elapsed_count: 0,
            })
        }
        StatusConditionDuration::UntilNextAction => BattleStatusConditionDuration::UntilNextAction,
    };
    BattleStatusCondition {
        potency: status_condition.potency.clone(),
        duration,
    }
}

// 行動成否判定
/// 不発の場合、理由を返す
/// 発動の場合、Noneを返す
fn determine_action_outcome_failure(
    conduct: &BattleConduct,
    attacker: &BattleCharacter,
) -> Option<BattleIncidentConductOutcomeFailureReason> {
    if attacker.character_type == BattleCharacterType::Player {
        // プレイヤーキャラクターの場合のみスタミナチェック

        // スタミナが足りないと不発
        if attacker.stamina.current_stamina < conduct.art.stamina_cost {
            return Some(BattleIncidentConductOutcomeFailureReason::InsufficientStamina);
        }
    }

    // ブレイク中行動不能
    for se in attacker.status_conditions.iter() {
        if let StatusConditionPotency::Break(_) = &se.potency {
            // ブレイク中
            return Some(BattleIncidentConductOutcomeFailureReason::IsBreak);
        }
    }

    // 必要能力が足りないと不発
    let req = &conduct.art.requirement;
    let ability = &attacker.current_ability();
    if ability.strength < req.strength
        || ability.dexterity < req.dexterity
        || ability.intelligence < req.intelligence
        || ability.faith < req.faith
        || ability.arcane < req.arcane
        || ability.agility < req.agility
    {
        return Some(BattleIncidentConductOutcomeFailureReason::InsufficientAbility);
    }

    // SPが足りないと不発
    let sp_cost = conduct.art.sp_cost;
    if attacker.sp.current_sp < sp_cost {
        return Some(BattleIncidentConductOutcomeFailureReason::InsufficientSp);
    }

    None
}
