use bevy::prelude::*;
use super::super::resources::*;
use super::super::super::events::*;
use crate::fundamental::*;

pub fn handle_combination_resolved(
    mut events: EventReader<BattleCombinationEvent>,
    mut log_ev: EventWriter<BattleLogEvent>,
) {
    for event in events.read() {
        log_ev.write(BattleLogEvent("コンビネーション発動！".to_string()));

        for character_incident in event.incident.incidents.iter() {
            for concrete in character_incident.concretes.iter() {
                match concrete {
                    BattleCharacterIncidentConcrete::TranceIncrease(t) => {
                        log_ev.write(BattleLogEvent(format!(
                            "トランス値 +{} ({} → {})",
                            t.increase, t.before, t.after
                        )));
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn handle_conduct_resolved(
    mut events: EventReader<BattleConductResolvedEvent>,
    mut log_ev: EventWriter<BattleLogEvent>,
    mut popup: ResMut<EnemyDamagePopup>,
) {
    for event in events.read() {
        let player_id = event.player_character_id;
        let enemy_id = event.enemy_character_id;
        let incident = &event.incident;

        match &incident.outcome {
            BattleIncidentConductOutcome::Failure(f) => {
                let reason = match f.reason {
                    BattleIncidentConductOutcomeFailureReason::InsufficientStamina => "スタミナ不足",
                    BattleIncidentConductOutcomeFailureReason::InsufficientSp => "SP不足",
                    BattleIncidentConductOutcomeFailureReason::InsufficientAbility => "能力不足",
                    BattleIncidentConductOutcomeFailureReason::IsBreak => "ブレイク状態",
                };
                log_ev.write(BattleLogEvent(format!(
                    "{}は不発（{}）",
                    incident.conduct.art.name, reason
                )));
            }
            BattleIncidentConductOutcome::Success(s) => {
                // 攻撃側のインシデント
                for character_incident in s.attacker.incidents.iter() {
                    for concrete in character_incident.concretes.iter() {
                        match concrete {
                            BattleCharacterIncidentConcrete::CombinationSkillActivated(c) => {
                                log_ev.write(BattleLogEvent(format!(
                                    "コンビネーション技 {} 発動！",
                                    c.combination_skill_name
                                )));
                            }
                            BattleCharacterIncidentConcrete::DamageSp(d) => {
                                log_ev.write(BattleLogEvent(format!(
                                    "SP -{} ({} → {})",
                                    d.damage, d.before, d.after
                                )));
                            }
                            BattleCharacterIncidentConcrete::DamageStamina(d) => {
                                log_ev.write(BattleLogEvent(format!(
                                    "Stamina -{} ({} → {})",
                                    d.damage, d.before, d.after
                                )));
                            }
                            _ => {}
                        }
                    }
                }

                // 防御側のインシデント
                for def in s.defenders.iter() {
                    if def.is_evaded {
                        log_ev.write(BattleLogEvent("回避した".to_string()));
                    }
                    if def.is_defended {
                        log_ev.write(BattleLogEvent("防御した".to_string()));
                    }

                    for character_incident in def.character.incidents.iter() {
                        for concrete in character_incident.concretes.iter() {
                            match concrete {
                                BattleCharacterIncidentConcrete::DamageHp(d) => {
                                    let who = if def.character.character_id == enemy_id {
                                        "敵"
                                    } else {
                                        "プレイヤー"
                                    };
                                    log_ev.write(BattleLogEvent(format!(
                                        "{} に{}ダメージ (HP {} → {})",
                                        who, d.damage, d.before, d.after
                                    )));
                                    if def.character.character_id == enemy_id {
                                        popup.amount = d.damage as i32;
                                        popup.timer = 1.0;
                                    }
                                }
                                BattleCharacterIncidentConcrete::RecoverHp(r) => {
                                    let who = if def.character.character_id == player_id {
                                        "プレイヤー"
                                    } else {
                                        "敵"
                                    };
                                    log_ev.write(BattleLogEvent(format!(
                                        "{} のHPを{}回復 ({} → {})",
                                        who, r.recover, r.before, r.after
                                    )));
                                }
                                BattleCharacterIncidentConcrete::RecoverStamina(r) => {
                                    let who = if def.character.character_id == player_id {
                                        "プレイヤー"
                                    } else {
                                        "敵"
                                    };
                                    log_ev.write(BattleLogEvent(format!(
                                        "{} のスタミナを{}回復 ({} → {})",
                                        who, r.recover, r.before, r.after
                                    )));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}
