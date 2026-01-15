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
        BattleCharacter::Player(player)
    } else if let Some(enemy) = battle
        .enemies
        .iter_mut()
        .find(|e| e.character_id == conduct.actor_character_id)
    {
        BattleCharacter::Enemy(enemy)
    } else {
        panic!("Attacker not found");
    };
    let attacker_id = attacker.character_id();

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
    let sp_cost = conduct.conduct.sp_cost;
    let (before_sp, after_sp) = attacker.current_stats_mut().sp_subtract(sp_cost);
    // インシデント
    attacker_stats_changes.push(BattleIncidentStats::DamageSp(BattleIncidentDamageSp {
        damage: sp_cost,
        before: before_sp,
        after: after_sp,
    }));

    // スタミナ消費
    if let BattleCharacter::Player(player) = attacker {
        // プレイヤーの場合のみスタミナ消費処理
        let stamina_cost = conduct.conduct.stamina_cost;
        let (before_stamina, after_stamina) =
            player.base.current_stats.stamina_subtract(stamina_cost);
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
        BattleCharacter::Player(player)
    } else if let Some(enemy) = battle
        .enemies
        .iter_mut()
        .find(|e| e.character_id == conduct.target_character_id)
    {
        BattleCharacter::Enemy(enemy)
    } else {
        panic!("Defender not found");
    };
    // TODO: 複数ターゲットが存在した時のターゲットごとに効果処理
    let defender_incident = conduct_effect(&conduct, &mut target);

    BattleIncident::Conduct(BattleIncidentConduct {
        attacker_id,
        defender_id: target.character_id(),
        conduct,
        outcome: BattleIncidentConductOutcome::Success(BattleIncidentConductOutcomeSuccess {
            attacker: attacker_incident,
            defenders: vec![defender_incident],
        }),
    })
}

// 行動攻撃補正
fn calc_conduct_attack_modifier(
    base_attack_power: u32,
    conduct_attack_power: u32,
    conduct_attack_power_scaling: f32,
) -> u32 {
    conduct_attack_power + (base_attack_power as f32 * conduct_attack_power_scaling) as u32
}

// 攻撃力補正
fn calc_attack_power_modifier(
    base_attack_power: &AttackPower,
    modify_attack_power: &AttackPower,
    modify_attack_power_scaling: &AttackPowerScaling,
) -> AttackPower {
    AttackPower {
        slash: modify_attack_power.slash
            + (base_attack_power.slash as f32 * modify_attack_power_scaling.slash) as u32,
        strike: modify_attack_power.strike
            + (base_attack_power.strike as f32 * modify_attack_power_scaling.strike) as u32,
        thrust: modify_attack_power.thrust
            + (base_attack_power.thrust as f32 * modify_attack_power_scaling.thrust) as u32,
        impact: modify_attack_power.impact
            + (base_attack_power.impact as f32 * modify_attack_power_scaling.impact) as u32,
        magic: modify_attack_power.magic
            + (base_attack_power.magic as f32 * modify_attack_power_scaling.magic) as u32,
        fire: modify_attack_power.fire
            + (base_attack_power.fire as f32 * modify_attack_power_scaling.fire) as u32,
        lightning: modify_attack_power.lightning
            + (base_attack_power.lightning as f32 * modify_attack_power_scaling.lightning) as u32,
        chaos: modify_attack_power.chaos
            + (base_attack_power.chaos as f32 * modify_attack_power_scaling.chaos) as u32,
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
    status_effects: &Vec<StatusEffect>,
    target: &mut BattleCharacter,
) -> Vec<BattleIncidentStatusEffect> {
    // 支援行動処理
    let mut status_effect_incidents: Vec<BattleIncidentStatusEffect> = Vec::new();
    for status_effect in status_effects {
        // 状態変化付与処理
        let battle_status_effect = create_battle_status_effect(status_effect);
        // 状態変化付与
        // TODO: 状態変化の重複処理
        target
            .status_effects_mut()
            .push(battle_status_effect.clone());
        status_effect_incidents.push(BattleIncidentStatusEffect {
            status_effect: battle_status_effect,
            status_effect_handling: BattleIncidentStatusEffectHandling::Applied(
                BattleIncidentStatusEffectApplied {},
            ),
        });
    }
    status_effect_incidents
}

fn support_recover(
    recover: &SupportRecover,
    target: &mut BattleCharacter,
) -> Vec<BattleIncidentStats> {
    // 支援回復処理
    let mut stats_change_incidents = Vec::new();
    for potency in &recover.potencies {
        match potency {
            SupportRecoverPotency::Hp(hp_recover) => {
                let hp_rcv = hp_recover.hp_recover;
                let (before_hp, after_hp) = target.current_stats_mut().hp_add(hp_rcv);
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
                let (before_sp, after_sp) = target.current_stats_mut().sp_add(sp_rcv);
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
                if let BattleCharacter::Player(player) = target {
                    let stamina_rcv = stamina_recover.stamina_recover;
                    let (before_stamina, after_stamina) =
                        player.base.current_stats.stamina_add(stamina_rcv);
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

fn create_battle_status_effect(status_effect: &StatusEffect) -> BattleStatusEffect {
    let duration = match &status_effect.duration {
        StatusEffectDuration::Permanent => BattleStatusEffectDuration::Permanent,
        StatusEffectDuration::Turn(d) => {
            BattleStatusEffectDuration::Turn(BattleStatusEffectDurationTurn {
                turns: d.turns,
                elapsed_turns: 0,
            })
        }
        StatusEffectDuration::Count(d) => {
            BattleStatusEffectDuration::Count(BattleStatusEffectDurationCount {
                count: d.count,
                elapsed_count: 0,
            })
        }
        StatusEffectDuration::UntilNextAction => BattleStatusEffectDuration::UntilNextAction,
    };
    BattleStatusEffect {
        potency: status_effect.potency.clone(),
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
    match &attacker {
        BattleCharacter::Player(player) => {
            let current_status = &player.base.current_stats;
            // スタミナが足りないと不発
            if current_status.current_stamina < conduct.conduct.stamina_cost {
                return Some(BattleIncidentConductOutcomeFailureReason {
                    insufficient_stamina: true,
                    insufficient_ability: false,
                    insufficient_sp: false,
                    is_break: false,
                });
            }
        }
        BattleCharacter::Enemy(_) => {
            // 敵のスタミナ管理は省略
        }
    };

    // ブレイク中行動不能
    for se in attacker.status_effects().iter() {
        if let StatusEffectPotency::Break(_) = &se.potency {
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
    let req = &conduct.conduct.requirement;
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
    let sp_cost = conduct.conduct.sp_cost;
    if attacker.current_stats().current_sp < sp_cost {
        return Some(BattleIncidentConductOutcomeFailureReason {
            insufficient_stamina: false,
            insufficient_ability: false,
            insufficient_sp: true,
            is_break: false,
        });
    }

    None
}
