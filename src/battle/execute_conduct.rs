mod conduct_effect;

use super::*;
use conduct_effect::conduct_effect;
use std::sync::Arc;

pub struct BattleExecuteConductRequest {
    pub conduct: BattleConduct,
}

// 行動実行
pub fn execute_conduct(
    battle: &mut Battle,
    request: BattleExecuteConductRequest,
) -> BattleIncident {
    let conduct = request.conduct;

    // 行動者の決定
    let mut attacker = if let Some(player) = battle
        .players
        .iter_mut()
        .find(|p| p.character_id == conduct.actor_character_id)
    {
        player
    } else if let Some(enemy) = battle
        .enemies
        .iter_mut()
        .find(|e| e.character_id == conduct.actor_character_id)
    {
        enemy
    } else {
        panic!("Attacker not found");
    };
    let attacker_id = attacker.character_id;

    // 行動成否判定
    if let Some(failure_reason) = determine_action_outcome_failure(&conduct, &attacker) {
        // TODO: 不発理由に応じた処理
        return BattleIncident::Conduct(BattleIncidentConduct {
            attacker_id,
            defender_id: conduct.target_character_id,
            conduct,
            outcome: BattleIncidentConductOutcome::Failure(BattleIncidentConductOutcomeFailure {
                reason: failure_reason,
            }),
        });
    }

    //
    let mut attacker_stats_changes = Vec::new();

    // SP消費
    let sp_cost = conduct.art.sp_cost;
    let (before_sp, after_sp) = attacker.sp.damage(sp_cost);
    // インシデント
    attacker_stats_changes.push(BattleIncidentStats::DamageSp(BattleIncidentDamageSp {
        damage: sp_cost,
        before: before_sp,
        after: after_sp,
    }));

    // スタミナ消費
    if attacker.character_type == BattleCharacterType::Player {
        // プレイヤーの場合のみスタミナ消費処理
        let stamina_cost = conduct.art.stamina_cost;
        let (before_stamina, after_stamina) = attacker.stamina.damage(stamina_cost);
        // インシデント
        attacker_stats_changes.push(BattleIncidentStats::DamageStamina(
            BattleIncidentDamageStamina {
                damage: stamina_cost,
                before: before_stamina,
                after: after_stamina,
            },
        ));
    }

    // 行動者インシデント
    let attacker_incident = BattleIncidentConductOutcomeSuccessAttacker {
        character_id: attacker_id,
        stats_changes: attacker_stats_changes,
    };

    // ターゲットの決定
    let mut target = if let Some(player) = battle
        .players
        .iter_mut()
        .find(|p| p.character_id == conduct.target_character_id)
    {
        player
    } else if let Some(enemy) = battle
        .enemies
        .iter_mut()
        .find(|e| e.character_id == conduct.target_character_id)
    {
        enemy
    } else {
        panic!("Defender not found");
    };
    let target_id = target.character_id;

    // TODO: 複数ターゲットが存在した時のターゲットごとに効果処理

    let defender_incident = conduct_effect(battle, &conduct, &attacker_id, &target_id);

    BattleIncident::Conduct(BattleIncidentConduct {
        attacker_id,
        defender_id: target_id,
        conduct,
        outcome: BattleIncidentConductOutcome::Success(BattleIncidentConductOutcomeSuccess {
            attacker: attacker_incident,
            defenders: vec![defender_incident],
        }),
    })
}

// 攻撃力補正
fn calc_attack_power_modifier(
    base_attack_power: &AttackPower,
    modify_attack_power: &AttackPower,
    modify_attack_power_scaling: &AttackPowerScaling,
) -> AttackPower {
    AttackPower {
        slash: base_attack_power.slash
            + (modify_attack_power.slash as f32 * modify_attack_power_scaling.slash) as u32,
        strike: base_attack_power.strike
            + (modify_attack_power.strike as f32 * modify_attack_power_scaling.strike) as u32,
        thrust: base_attack_power.thrust
            + (modify_attack_power.thrust as f32 * modify_attack_power_scaling.thrust) as u32,
        impact: base_attack_power.impact
            + (modify_attack_power.impact as f32 * modify_attack_power_scaling.impact) as u32,
        magic: base_attack_power.magic
            + (modify_attack_power.magic as f32 * modify_attack_power_scaling.magic) as u32,
        fire: base_attack_power.fire
            + (modify_attack_power.fire as f32 * modify_attack_power_scaling.fire) as u32,
        lightning: base_attack_power.lightning
            + (modify_attack_power.lightning as f32 * modify_attack_power_scaling.lightning) as u32,
        chaos: base_attack_power.chaos
            + (modify_attack_power.chaos as f32 * modify_attack_power_scaling.chaos) as u32,
    }
}

fn calc_attack_power_cut_rate(
    attack_power: &AttackPower,
    guard_cut_rate: &GuardCutRate,
) -> AttackPower {
    AttackPower {
        slash: (attack_power.slash as f32 * guard_cut_rate.slash) as u32,
        strike: (attack_power.strike as f32 * guard_cut_rate.strike) as u32,
        thrust: (attack_power.thrust as f32 * guard_cut_rate.thrust) as u32,
        impact: (attack_power.impact as f32 * guard_cut_rate.impact) as u32,
        magic: (attack_power.magic as f32 * guard_cut_rate.magic) as u32,
        fire: (attack_power.fire as f32 * guard_cut_rate.fire) as u32,
        lightning: (attack_power.lightning as f32 * guard_cut_rate.lightning) as u32,
        chaos: (attack_power.chaos as f32 * guard_cut_rate.chaos) as u32,
    }
}

// ダメージ計算
fn calc_damage(attack_power: &AttackPower, defender: &DefensePower) -> u32 {
    let damage = (attack_power.slash / defender.slash)
        + (attack_power.strike / defender.strike)
        + (attack_power.thrust / defender.thrust)
        + (attack_power.impact / defender.impact)
        + (attack_power.magic / defender.magic)
        + (attack_power.fire / defender.fire)
        + (attack_power.lightning / defender.lightning)
        + (attack_power.chaos / defender.chaos);
    damage
}

fn support_status_effect(
    status_conditions: &Vec<StatusCondition>,
    target: &mut BattleCharacter,
) -> Vec<BattleIncidentStatusCondition> {
    // 支援行動処理
    let mut status_condition_incidents: Vec<BattleIncidentStatusCondition> = Vec::new();
    for status_condition in status_conditions {
        // 状態変化付与処理
        let battle_status_condition = create_battle_status_condition(status_condition);
        // 状態変化付与
        // TODO: 状態変化の重複処理
        target
            .status_conditions
            .push(battle_status_condition.clone());
        status_condition_incidents.push(BattleIncidentStatusCondition {
            status_condition: battle_status_condition,
            status_condition_handling: BattleIncidentStatusConditionHandling::Applied(
                BattleIncidentStatusConditionApplied {},
            ),
        });
    }
    status_condition_incidents
}

fn support_recover(
    recover: &ArtPotencySupportRecover,
    target: &mut BattleCharacter,
) -> Vec<BattleIncidentStats> {
    // 支援回復処理
    let mut stats_change_incidents = Vec::new();
    for potency in &recover.potencies {
        match potency {
            SupportRecoverPotency::Hp(hp_recover) => {
                let hp_rcv = hp_recover.hp_recover;
                let (before_hp, after_hp) = target.hp.recover(hp_rcv);
                // HP回復のインシデント
                stats_change_incidents.push(BattleIncidentStats::RecoverHp(
                    BattleIncidentRecoverHp {
                        recover: hp_rcv,
                        before: before_hp,
                        after: after_hp,
                    },
                ));
            }
            SupportRecoverPotency::Sp(sp_recover) => {
                let sp_rcv = sp_recover.sp_recover;
                let (before_sp, after_sp) = target.sp.recover(sp_rcv);
                // SP回復のインシデント
                stats_change_incidents.push(BattleIncidentStats::RecoverSp(
                    BattleIncidentRecoverSp {
                        recover: sp_rcv,
                        before: before_sp,
                        after: after_sp,
                    },
                ));
            }
            SupportRecoverPotency::Stamina(stamina_recover) => {
                // スタミナ回復処理はプレイヤーキャラクターのみ
                if target.character_type == BattleCharacterType::Player {
                    let stamina_rcv = stamina_recover.stamina_recover;
                    let (before_stamina, after_stamina) = target.stamina.recover(stamina_rcv);
                    // スタミナ回復のインシデント
                    stats_change_incidents.push(BattleIncidentStats::RecoverStamina(
                        BattleIncidentRecoverStamina {
                            recover: stamina_rcv,
                            before: before_stamina,
                            after: after_stamina,
                        },
                    ));
                }
            }
        }
    }
    stats_change_incidents
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
            return Some(BattleIncidentConductOutcomeFailureReason {
                insufficient_stamina: true,
                insufficient_ability: false,
                insufficient_sp: false,
                is_break: false,
            });
        }
    }

    // ブレイク中行動不能
    for se in attacker.status_conditions.iter() {
        if let StatusConditionPotency::Break(_) = &se.potency {
            // ブレイク中
            return Some(BattleIncidentConductOutcomeFailureReason {
                insufficient_stamina: false,
                insufficient_ability: false,
                insufficient_sp: false,
                is_break: true,
            });
        }
    }

    // 必要能力が足りないと不発
    let req = &conduct.art.requirement;
    let abil = &attacker.current_ability();
    if abil.strength < req.strength
        || abil.dexterity < req.dexterity
        || abil.intelligence < req.intelligence
        || abil.faith < req.faith
        || abil.arcane < req.arcane
        || abil.agility < req.agility
    {
        return Some(BattleIncidentConductOutcomeFailureReason {
            insufficient_stamina: false,
            insufficient_ability: true,
            insufficient_sp: false,
            is_break: false,
        });
    }

    // SPが足りないと不発
    let sp_cost = conduct.art.sp_cost;
    if attacker.sp.current_sp < sp_cost {
        return Some(BattleIncidentConductOutcomeFailureReason {
            insufficient_stamina: false,
            insufficient_ability: false,
            insufficient_sp: true,
            is_break: false,
        });
    }

    None
}
