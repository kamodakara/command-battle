use super::super::super::events::*;
use super::super::resources::*;
use crate::fundamental::*;
use bevy::prelude::*;

fn format_art_cost(sp_cost: u32, stamina_cost: u32) -> String {
    match (sp_cost, stamina_cost) {
        (0, 0) => String::new(),
        (0, st) => format!(" (消費: ST{})", st),
        (sp, 0) => format!(" (消費: SP{})", sp),
        (sp, st) => format!(" (消費: SP{} ST{})", sp, st),
    }
}

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

        let actor = if incident.conduct.actor_character_id == player_id {
            "プレイヤー"
        } else {
            "敵"
        };
        let art = &incident.conduct.art;

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
                    "{}の{}は不発（{}）",
                    actor, incident.conduct.art.name, reason
                )));
            }
            BattleIncidentConductOutcome::Success(s) => {
                let mut sp_cost = 0;
                let mut stamina_cost = 0;
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
                                if d.damage > 0 {
                                    sp_cost = d.damage;
                                }
                            }
                            BattleCharacterIncidentConcrete::DamageStamina(d) => {
                                if d.damage > 0 {
                                    stamina_cost = d.damage;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                let cost_str = format_art_cost(sp_cost, stamina_cost);
                log_ev.write(BattleLogEvent(format!(
                    "{}の{}{}",
                    actor, art.name, cost_str
                )));

                for def in s.defenders.iter() {
                    if def.is_evaded {
                        log_ev.write(BattleLogEvent("回避した".to_string()));
                    }
                    // GuardSuccess インシデントから武器名と消費スタミナを取得
                    let guard_info = def
                        .character
                        .incidents
                        .iter()
                        .flat_map(|i| i.concretes.iter())
                        .find_map(|c| {
                            if let BattleCharacterIncidentConcrete::GuardSuccess(g) = c {
                                Some(g)
                            } else {
                                None
                            }
                        });
                    if let Some(g) = guard_info {
                        let cost_str = if g.stamina_consumed > 0 {
                            format!(" (消費: ST{})", g.stamina_consumed)
                        } else {
                            String::new()
                        };
                        let msg = format!("{}で攻撃を防いだ {}", g.weapon_name, cost_str);
                        log_ev.write(BattleLogEvent(msg));
                    };

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
