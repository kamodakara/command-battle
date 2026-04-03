use super::super::super::events::*;
use super::super::resources::*;
use crate::data::DataManager;
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
    data_manager: Res<DataManager>,
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
                            BattleCharacterIncidentConcrete::StatusAilmentAccumulation(s) => {
                                let ailment_name = status_ailment_name(&s.status_ailment);
                                log_ev.write(BattleLogEvent(format!(
                                    "{}に{}が蓄積した ({} → {})",
                                    actor,
                                    ailment_name,
                                    s.before_accumulation,
                                    s.after_accumulation
                                )));
                            }
                            BattleCharacterIncidentConcrete::StatusAilmentApplied(s) => {
                                let ailment_name = status_ailment_name(&s.status_ailment);
                                log_ev.write(BattleLogEvent(format!(
                                    "{}に{}が発症した！",
                                    actor, ailment_name
                                )));
                            }
                            BattleCharacterIncidentConcrete::KarmaAddedToDeck(k) => {
                                let card_name = data_manager
                                    .karma_card
                                    .find_by_id(k.karma_card_id.0)
                                    .map(|r| r.data.name.as_str())
                                    .unwrap_or("不明なカード");
                                log_ev.write(BattleLogEvent(format!(
                                    "{}の山札に「{}」が{}枚追加された",
                                    actor, card_name, k.count
                                )));
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
                                BattleCharacterIncidentConcrete::StatusAilmentAccumulation(s) => {
                                    let who = if def.character.character_id == enemy_id {
                                        "敵"
                                    } else {
                                        "プレイヤー"
                                    };
                                    let ailment_name = status_ailment_name(&s.status_ailment);
                                    log_ev.write(BattleLogEvent(format!(
                                        "{}に{}が蓄積した ({} → {})",
                                        who,
                                        ailment_name,
                                        s.before_accumulation,
                                        s.after_accumulation
                                    )));
                                }
                                BattleCharacterIncidentConcrete::StatusAilmentApplied(s) => {
                                    let who = if def.character.character_id == enemy_id {
                                        "敵"
                                    } else {
                                        "プレイヤー"
                                    };
                                    let ailment_name = status_ailment_name(&s.status_ailment);
                                    log_ev.write(BattleLogEvent(format!(
                                        "{}に{}が発症した！",
                                        who, ailment_name
                                    )));
                                }
                                BattleCharacterIncidentConcrete::KarmaAddedToDeck(k) => {
                                    let who = if def.character.character_id == enemy_id {
                                        "敵"
                                    } else {
                                        "プレイヤー"
                                    };
                                    let card_name = data_manager
                                        .karma_card
                                        .find_by_id(k.karma_card_id.0)
                                        .map(|r| r.data.name.as_str())
                                        .unwrap_or("不明なカード");
                                    log_ev.write(BattleLogEvent(format!(
                                        "{}の山札に「{}」が{}枚追加された",
                                        who, card_name, k.count
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

pub fn handle_turn_end_incidents(
    mut events: MessageReader<BattleTurnEndEvent>,
    mut log_ev: MessageWriter<BattleLogEvent>,
) {
    for event in events.read() {
        let player_id = event.player_character_id;
        for character_incident in &event.incidents {
            let who = if character_incident.character_id == player_id {
                "プレイヤー"
            } else {
                "敵"
            };
            for incident in &character_incident.incidents {
                for concrete in &incident.concretes {
                    match concrete {
                        BattleCharacterIncidentConcrete::DamageHp(d) => {
                            if d.damage > 0 {
                                log_ev.write(BattleLogEvent(format!(
                                    "{}に{}ダメージ (HP {} → {})",
                                    who, d.damage, d.before, d.after
                                )));
                            }
                        }
                        BattleCharacterIncidentConcrete::DamageSp(d) => {
                            if d.damage > 0 {
                                log_ev.write(BattleLogEvent(format!(
                                    "{}のSPが{}減少 (SP {} → {})",
                                    who, d.damage, d.before, d.after
                                )));
                            }
                        }
                        BattleCharacterIncidentConcrete::RecoverStamina(r) => {
                            if r.recover > 0 {
                                log_ev.write(BattleLogEvent(format!(
                                    "{}のスタミナが{}回復 ({} → {})",
                                    who, r.recover, r.before, r.after
                                )));
                            }
                        }
                        BattleCharacterIncidentConcrete::StatusConditionRemoved(s) => {
                            let condition_name = match &s.status_condition.potency {
                                StatusConditionPotency::Resistance(_) => "防御",
                                StatusConditionPotency::Evasion => "回避",
                                StatusConditionPotency::Airborne => "空中",
                                StatusConditionPotency::Floating => "浮遊",
                                StatusConditionPotency::Melee => "近距離",
                                StatusConditionPotency::Ranged => "遠距離",
                            };
                            log_ev.write(BattleLogEvent(format!(
                                "{}の{}状態が解除された",
                                who, condition_name
                            )));
                        }
                        BattleCharacterIncidentConcrete::StatusAilmentRecovery(s) => {
                            let ailment_name = status_ailment_name(&s.status_ailment);
                            log_ev.write(BattleLogEvent(format!(
                                "{}の{}蓄積値が回復 ({} → {})",
                                who, ailment_name, s.before_accumulation, s.after_accumulation
                            )));
                        }
                        BattleCharacterIncidentConcrete::StatusAilmentRemoved(s) => {
                            let ailment_name = status_ailment_name(&s.status_ailment);
                            log_ev.write(BattleLogEvent(format!(
                                "{}の{}が回復した",
                                who, ailment_name
                            )));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn status_ailment_name(ailment: &StatusAilment) -> &'static str {
    match ailment {
        StatusAilment::Poison => "毒",
        StatusAilment::Sleep => "眠気",
        StatusAilment::Chill => "寒気",
        StatusAilment::Bleed => "出血",
        StatusAilment::Burn => "火傷",
        StatusAilment::Paralysis => "麻痺",
        StatusAilment::Fear => "恐怖",
        StatusAilment::Rage => "激昂",
        StatusAilment::Breaking => "ブレイク",
    }
}
