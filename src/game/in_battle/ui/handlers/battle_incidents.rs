use super::super::super::events::*;
use super::super::resources::*;
use crate::fundamental::*;
use bevy::prelude::*;

pub fn handle_combination_resolved(
    mut events: MessageReader<BattleCombinationEvent>,
    mut log_ev: MessageWriter<BattleLogEvent>,
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
    mut events: MessageReader<BattleConductResolvedEvent>,
    mut log_ev: MessageWriter<BattleLogEvent>,
    mut popup: ResMut<EnemyDamagePopup>,
    mut board: ResMut<super::super::resources::TurnActionBoard>,
) {
    for event in events.read() {
        let player_id = event.player_character_id;
        let enemy_id = event.enemy_character_id;
        let incident = &event.incident;

        // 行動ボード更新：敵が行動者なら行動名を公開、スロットを実行済みマーク
        let enemy_art_name = if incident.conduct.actor_character_id == enemy_id {
            Some(incident.conduct.art.name.as_str())
        } else {
            None
        };
        board.on_conduct(event.action_index as usize, enemy_art_name);

        match &incident.outcome {
            BattleIncidentConductOutcome::Failure(f) => {
                let reason = match f.reason {
                    BattleIncidentConductOutcomeFailureReason::InsufficientStamina => {
                        "スタミナ不足"
                    }
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
