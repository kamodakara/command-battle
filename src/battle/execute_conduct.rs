use super::*;
use std::rc::Rc;

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

    let before_sp = attacker.current_stats().max_sp - attacker.current_stats().sp_damage;
    let sp_damage = conduct.conduct.sp_cost;
    // SP消費
    attacker.current_stats_mut().sp_damage += sp_damage;
    let after_sp = attacker.current_stats().max_sp - attacker.current_stats().sp_damage;
    // インシデント
    attacker_stats_changes.push(BattleIncidentStats::DamageSp(BattleIncidentDamageSp {
        damage: sp_damage,
        before: before_sp,
        after: after_sp,
    }));

    // スタミナ消費
    if let BattleCharacter::Player(player) = attacker {
        // プレイヤーの場合のみスタミナ消費処理
        let before_stamina =
            player.base.current_stats.max_stamina - player.base.current_stats.stamina_damage;
        let stamina_damage = conduct.conduct.stamina_cost;
        player.base.current_stats.stamina_damage += conduct.conduct.stamina_cost;
        let after_stamina =
            player.base.current_stats.max_stamina - player.base.current_stats.stamina_damage;
        // インシデント
        attacker_stats_changes.push(BattleIncidentStats::DamageStamina(
            BattleIncidentDamageStamina {
                damage: stamina_damage,
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

fn conduct_effect(
    conduct: &BattleConduct,
    target: &mut BattleCharacter,
) -> BattleIncidentConductOutcomeSuccessDefender {
    // 回避判定
    for se in target.status_effects().iter() {
        match &se.potency {
            StatusEffectPotency::Evasion => {
                // 回避効果処理
                return BattleIncidentConductOutcomeSuccessDefender {
                    character_id: target.character_id(),
                    stats_changes: Vec::new(),
                    status_effects: Vec::new(),
                    is_defended: false,
                    is_evaded: true,
                };
            }
            StatusEffectPotency::Airborne => {
                // 空中効果処理
                // 遠距離攻撃でない時は回避
                if !conduct.conduct.perks.contains(&ConductPerk::Ranged) {
                    return BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.character_id(),
                        stats_changes: Vec::new(),
                        status_effects: Vec::new(),
                        is_defended: false,
                        is_evaded: true,
                    };
                }
            }
            StatusEffectPotency::Floating => {
                // 浮遊効果処理
                // 足元攻撃は回避
                if conduct.conduct.perks.contains(&ConductPerk::AtFeet) {
                    return BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.character_id(),
                        stats_changes: Vec::new(),
                        status_effects: Vec::new(),
                        is_defended: false,
                        is_evaded: true,
                    };
                }
            }
            StatusEffectPotency::Ranged => {
                // 遠距離効果処理
                // 近距離の攻撃を回避
                if !conduct.conduct.perks.contains(&ConductPerk::Ranged) {
                    return BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.character_id(),
                        stats_changes: Vec::new(),
                        status_effects: Vec::new(),
                        is_defended: false,
                        is_evaded: true,
                    };
                }
            }
            _ => {
                // その他
            }
        }
    }

    match &conduct.conduct.conduct_type {
        ConductType::Basic(basic) => {
            match basic {
                ConductTypeBasic::Attack(conduct_attack) => {
                    let mut stats_change_incidents = Vec::new();
                    let mut status_effect_incidents = Vec::new();

                    // ダメージ計算
                    let mut attak_power = conduct_attack.attack_power.clone();
                    let mut is_defended = false;
                    for se in target.status_effects().iter() {
                        match &se.potency {
                            StatusEffectPotency::Resistance(resistance) => {
                                // 防御効果処理
                                attak_power =
                                    calc_attack_power_cut_rate(&attak_power, &resistance.cut_rate);
                                is_defended = true;
                            }
                            _ => {
                                // その他
                            }
                        }
                    }

                    let defense_power = &target.defense_power();
                    let damage = calc_damage(&attak_power, defense_power);

                    let current_hp_damage = target.current_stats().hp_damage;
                    let mut next_hp_damage = current_hp_damage + damage;
                    if next_hp_damage > target.current_stats().max_hp {
                        next_hp_damage = target.current_stats().max_hp;
                    }
                    target.current_stats_mut().hp_damage = next_hp_damage;
                    // HPダメージのインシデント
                    stats_change_incidents.push(BattleIncidentStats::DamageHp(
                        BattleIncidentDamageHp {
                            damage,
                            before: current_hp_damage,
                            after: next_hp_damage,
                        },
                    ));

                    // ブレイクダメージ処理
                    if let BattleCharacter::Enemy(enemy) = &target {
                        // ブレイク中でない時
                        let mut is_break = false;
                        for se in enemy.base.status_effects.iter() {
                            if let StatusEffectPotency::Break(_) = &se.potency {
                                is_break = true
                            }
                        }

                        if !is_break {
                            // 敵のブレイクダメージ処理
                            let break_power = conduct_attack.break_power;
                            let current_break_damage = enemy.current_enemy_only_stats.break_damage;
                            let mut next_break_damage = current_break_damage + break_power;
                            if next_break_damage > enemy.current_enemy_only_stats.max_break {
                                next_break_damage = enemy.current_enemy_only_stats.max_break;

                                // ブレイク状態にする
                                // TODO: サポート技用の関数を使用していいか？
                                let new_status_effects = support_status_effect(
                                    &vec![StatusEffect {
                                        potency: StatusEffectPotency::Break(StatusEffectBreak {}),
                                        duration: StatusEffectDuration::Permanent,
                                    }],
                                    target,
                                );
                                status_effect_incidents.extend(new_status_effects);
                            }

                            // ブレイクダメージインシデント追加
                            stats_change_incidents.push(BattleIncidentStats::DamageBreak(
                                BattleIncidentDamageBreak {
                                    damage: break_power,
                                    before: current_break_damage,
                                    after: next_break_damage,
                                },
                            ));
                        }
                    }

                    BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.character_id(),
                        stats_changes: stats_change_incidents,
                        status_effects: status_effect_incidents,
                        is_defended,
                        is_evaded: false,
                    }
                }
                ConductTypeBasic::Support(support) => {
                    // 支援行動処理
                    match &support {
                        ConductTypeBasicSupport::StatusEffect(status_effect) => {
                            let new_incidents =
                                support_status_effect(&status_effect.status_effects, target);

                            BattleIncidentConductOutcomeSuccessDefender {
                                character_id: target.character_id(),
                                stats_changes: Vec::new(),
                                status_effects: new_incidents,
                                is_defended: false,
                                is_evaded: false,
                            }
                        }
                    }
                }
            }
        }
        ConductType::Skill(skill) => match &skill.potency {
            ConductTypeSkillPotency::Attack(skill) => {
                let mut stats_change_incidents = Vec::new();
                let mut status_effect_incidents = Vec::new();

                let weapon_attack_power = if let Some(weapon) = &conduct.weapon {
                    &weapon.attack_power
                } else {
                    &AttackPower::default()
                };
                let skill_attack_power = &skill.attack_power;
                let skill_attack_power_scaling = &skill.attack_power_scaling;
                let mut attack_power = calc_attack_power_modifier(
                    skill_attack_power,
                    weapon_attack_power,
                    skill_attack_power_scaling,
                );

                // 防御効果処理
                let mut is_defended = false;
                for se in target.status_effects().iter() {
                    match &se.potency {
                        StatusEffectPotency::Resistance(resistance) => {
                            // 防御効果処理
                            attack_power =
                                calc_attack_power_cut_rate(&attack_power, &resistance.cut_rate);
                            is_defended = true;
                        }
                        _ => {
                            // その他
                        }
                    }
                }

                let break_power =
                    calc_conduct_attack_modifier(0, skill.break_power, skill.break_power_scaling);

                // ダメージ
                let damage = calc_damage(&attack_power, &target.defense_power());
                let current_hp_damage = target.current_stats().hp_damage;
                let mut next_hp_damage = current_hp_damage + damage;
                if next_hp_damage > target.current_stats().max_hp {
                    next_hp_damage = target.current_stats().max_hp;
                }
                target.current_stats_mut().hp_damage = next_hp_damage;
                // HPダメージのインシデント
                stats_change_incidents.push(BattleIncidentStats::DamageHp(
                    BattleIncidentDamageHp {
                        damage,
                        before: current_hp_damage,
                        after: next_hp_damage,
                    },
                ));

                // ブレイクダメージ処理
                if let BattleCharacter::Enemy(enemy) = &target {
                    // ブレイク中でない時
                    let mut is_break = false;
                    for se in enemy.base.status_effects.iter() {
                        if let StatusEffectPotency::Break(_) = &se.potency {
                            is_break = true
                        }
                    }
                    if !is_break {
                        // 敵のブレイクダメージ処理
                        let current_break_damage = enemy.current_enemy_only_stats.break_damage;
                        let mut next_break_damage = current_break_damage + break_power;
                        if next_break_damage > enemy.current_enemy_only_stats.max_break {
                            next_break_damage = enemy.current_enemy_only_stats.max_break;

                            // ブレイク状態にする
                            let new_status_effects = support_status_effect(
                                &vec![StatusEffect {
                                    potency: StatusEffectPotency::Break(StatusEffectBreak {}),
                                    duration: StatusEffectDuration::Permanent,
                                }],
                                target,
                            );
                            status_effect_incidents.extend(new_status_effects);
                        }
                        // ブレイクダメージインシデント追加
                        stats_change_incidents.push(BattleIncidentStats::DamageBreak(
                            BattleIncidentDamageBreak {
                                damage: break_power,
                                before: current_break_damage,
                                after: next_break_damage,
                            },
                        ));
                    }
                }

                BattleIncidentConductOutcomeSuccessDefender {
                    character_id: target.character_id(),
                    stats_changes: stats_change_incidents,
                    status_effects: Vec::new(),
                    is_defended,
                    is_evaded: false,
                }
            }
            ConductTypeSkillPotency::Support(support) => {
                // 支援行動処理
                match &support {
                    ConductTypeSkillPotencySupport::StatusEffect(status_effect) => {
                        let new_incidents =
                            support_status_effect(&status_effect.status_effects, target);

                        BattleIncidentConductOutcomeSuccessDefender {
                            character_id: target.character_id(),
                            stats_changes: Vec::new(),
                            status_effects: new_incidents,
                            is_defended: false,
                            is_evaded: false,
                        }
                    }
                }
            }
        },
        ConductType::Sorcery(sorcery) => match &sorcery {
            ConductTypeSorcery::Attack(sorcery) => {
                let mut stats_change_incidents = Vec::new();
                let mut status_effect_incidents = Vec::new();

                let mut attack_power = sorcery.attack_power.clone();
                let sorcery_power = if let Some(weapon) = &conduct.weapon {
                    weapon.sorcery_power
                } else {
                    1.0
                };
                attack_power.multiply(sorcery_power);

                // 防御効果処理
                let mut is_defended = false;
                for se in target.status_effects().iter() {
                    match &se.potency {
                        StatusEffectPotency::Resistance(resistance) => {
                            // 防御効果処理
                            attack_power =
                                calc_attack_power_cut_rate(&attack_power, &resistance.cut_rate);
                            is_defended = true;
                        }
                        _ => {
                            // その他
                        }
                    }
                }

                // ダメージ計算
                let damage = calc_damage(&attack_power, &target.defense_power());

                // TODO: ブレイク状態のダメージ補正
                // TODO: 防御側が敵の場合、ブレイクダメージ処理

                let current_hp_damage = target.current_stats().hp_damage;
                let mut next_hp_damage = current_hp_damage + damage;
                if next_hp_damage > target.current_stats().max_hp {
                    next_hp_damage = target.current_stats().max_hp;
                }
                target.current_stats_mut().hp_damage = next_hp_damage;
                // HPダメージのインシデント
                stats_change_incidents.push(BattleIncidentStats::DamageHp(
                    BattleIncidentDamageHp {
                        damage,
                        before: current_hp_damage,
                        after: next_hp_damage,
                    },
                ));

                // ブレイクダメージ処理
                if let BattleCharacter::Enemy(enemy) = &target {
                    // ブレイク中でない時
                    let mut is_break = false;
                    for se in target.status_effects().iter() {
                        if let StatusEffectPotency::Break(_) = &se.potency {
                            is_break = true
                        }
                    }
                    if !is_break {
                        // 敵のブレイクダメージ処理
                        let break_power = sorcery.break_power;
                        let current_break_damage = enemy.current_enemy_only_stats.break_damage;
                        let mut next_break_damage = current_break_damage + break_power;
                        if next_break_damage > enemy.current_enemy_only_stats.max_break {
                            next_break_damage = enemy.current_enemy_only_stats.max_break;
                            // ブレイク状態にする
                            let new_status_effects = support_status_effect(
                                &vec![StatusEffect {
                                    potency: StatusEffectPotency::Break(StatusEffectBreak {}),
                                    duration: StatusEffectDuration::Permanent,
                                }],
                                target,
                            );
                            status_effect_incidents.extend(new_status_effects);
                        }
                        // ブレイクダメージインシデント追加
                        stats_change_incidents.push(BattleIncidentStats::DamageBreak(
                            BattleIncidentDamageBreak {
                                damage: break_power,
                                before: current_break_damage,
                                after: next_break_damage,
                            },
                        ));
                    }
                }

                BattleIncidentConductOutcomeSuccessDefender {
                    character_id: target.character_id(),
                    stats_changes: stats_change_incidents,
                    status_effects: status_effect_incidents,
                    is_defended,
                    is_evaded: false,
                }
            }
            ConductTypeSorcery::Support(support) => match &support {
                ConductTypeSorcerySupport::StatusEffect(status_effect) => {
                    let new_incidents =
                        support_status_effect(&status_effect.status_effects, target);

                    BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.character_id(),
                        stats_changes: Vec::new(),
                        status_effects: new_incidents,
                        is_defended: false,
                        is_evaded: false,
                    }
                }
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use rand::rand_core::le;

    use super::*;

    // ヘルパー: ダミーのプレイヤー原本
    fn dummy_player() -> Rc<Player> {
        Rc::new(Player {
            ability: PlayerAbility {
                vitality: 0,
                spirit: 0,
                endurance: 0,
                agility: 0,
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
            },
            stats: PlayerStats {
                hp: 100,
                sp: 10,
                stamina: 10,
                equip_load: 0,
            },
            equipment: Equipment {
                weapon1: None,
                weapon2: None,
                armor1: None,
                armor2: None,
                armor3: None,
                armor4: None,
                armor5: None,
                armor6: None,
                armor7: None,
                armor8: None,
            },
        })
    }

    // ヘルパー: 最低限の防御力(0除算を避けるため全て1)
    fn min_defense() -> DefensePower {
        DefensePower {
            slash: 1,
            strike: 1,
            thrust: 1,
            impact: 1,
            magic: 1,
            fire: 1,
            lightning: 1,
            chaos: 1,
        }
    }

    // ヘルパー: デフォルト攻撃力(全て0)
    fn zero_attack() -> AttackPower {
        AttackPower::default()
    }

    // conduct_effect: 回避(Evasion)で早期リターンすること
    #[test]
    fn test_conduct_effect_evades_with_evasion() {
        // ターゲット: プレイヤーで回避状態
        let mut player = BattlePlayer {
            character_id: 1,
            original: dummy_player(),
            base: BattleCharacterBase {
                current_ability: BattleAbility {
                    agility: 0,
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                },
                current_stats: BattleStats {
                    max_hp: 100,
                    max_sp: 10,
                    max_stamina: 10,
                    hp_damage: 0,
                    sp_damage: 0,
                    stamina_damage: 0,
                },
                defense_power: min_defense(),
                status_effects: vec![BattleStatusEffect {
                    potency: StatusEffectPotency::Evasion,
                    duration: BattleStatusEffectDuration::Permanent,
                }],
            },
        };
        let mut target = BattleCharacter::Player(&mut player);

        // 行動: 基本攻撃(近接)で十分
        let conduct = BattleConduct {
            actor_character_id: 100,
            target_character_id: 1,
            conduct: Conduct {
                name: "Basic Attack".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Basic(ConductTypeBasic::Attack(
                    ConductTypeBasicAttack {
                        attack_power: zero_attack(),
                        break_power: 0,
                    },
                )),
            },
            weapon: None,
        };

        let result = conduct_effect(&conduct, &mut target);

        assert!(result.is_evaded);
        assert!(!result.is_defended);
        assert!(result.stats_changes.is_empty());
        assert!(result.status_effects.is_empty());
    }

    // conduct_effect: 非回避ルート（基本攻撃・攻撃力0）は回避せず、HPダメージ0の適用結果を返す
    #[test]
    fn test_conduct_effect_basic_attack_zero_damage() {
        // ターゲット: プレイヤー(状態変化なし)
        let mut player = BattlePlayer {
            character_id: 2,
            original: dummy_player(),
            base: BattleCharacterBase {
                current_ability: BattleAbility {
                    agility: 0,
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                },
                current_stats: BattleStats {
                    max_hp: 100,
                    max_sp: 10,
                    max_stamina: 10,
                    hp_damage: 0,
                    sp_damage: 0,
                    stamina_damage: 0,
                },
                defense_power: min_defense(),
                status_effects: vec![],
            },
        };
        let mut target = BattleCharacter::Player(&mut player);

        // 行動: 基本攻撃(近接)
        let conduct = BattleConduct {
            actor_character_id: 100,
            target_character_id: 2,
            conduct: Conduct {
                name: "Basic Attack".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Basic(ConductTypeBasic::Attack(
                    ConductTypeBasicAttack {
                        attack_power: zero_attack(),
                        break_power: 0,
                    },
                )),
            },
            weapon: None,
        };

        let result = conduct_effect(&conduct, &mut target);

        // 回避・防御なし
        assert!(!result.is_evaded);
        assert!(!result.is_defended);
        // 状態変化なし
        assert!(result.status_effects.is_empty());
        // HPダメージ0が記録されていること
        assert_eq!(result.stats_changes.len(), 1);
        match &result.stats_changes[0] {
            BattleIncidentStats::DamageHp(d) => {
                assert_eq!(d.damage, 0);
                assert_eq!(d.before, 0);
                assert_eq!(d.after, 0);
            }
            _ => panic!("expected DamageHp incident"),
        }
        // 実際のターゲットのHPダメージも0のまま
        assert_eq!(target.current_stats().hp_damage, 0);
    }

    // 基本攻撃でHPダメージが適用されること
    #[test]
    fn test_conduct_effect_basic_attack_applies_damage() {
        let mut player = BattlePlayer {
            character_id: 100,
            original: dummy_player(),
            base: BattleCharacterBase {
                current_ability: BattleAbility {
                    agility: 0,
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                },
                current_stats: BattleStats {
                    max_hp: 100,
                    max_sp: 10,
                    max_stamina: 10,
                    hp_damage: 0,
                    sp_damage: 0,
                    stamina_damage: 0,
                },
                defense_power: min_defense(),
                status_effects: vec![],
            },
        };
        let mut target = BattleCharacter::Player(&mut player);

        let mut atk = zero_attack();
        atk.slash = 10; // 期待ダメージ10

        let conduct = BattleConduct {
            actor_character_id: 100,
            target_character_id: 10,
            conduct: Conduct {
                name: "Basic Attack".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Basic(ConductTypeBasic::Attack(
                    ConductTypeBasicAttack {
                        attack_power: atk,
                        break_power: 0,
                    },
                )),
            },
            weapon: None,
        };

        let result = conduct_effect(&conduct, &mut target);

        assert!(!result.is_evaded);
        assert!(!result.is_defended);
        assert!(matches!(
            result.stats_changes.get(0),
            Some(BattleIncidentStats::DamageHp(_))
        ));
        if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
            assert_eq!(d.damage, 10);
            assert_eq!(d.before, 0);
            assert_eq!(d.after, 10);
        } else {
            panic!("expected DamageHp incident");
        }
        assert_eq!(target.current_stats().hp_damage, 10);
    }

    // 技攻撃でHPダメージが適用されること
    #[test]
    fn test_conduct_effect_skill_attack_applies_damage() {
        let mut player = BattlePlayer {
            character_id: 100,
            original: dummy_player(),
            base: BattleCharacterBase {
                current_ability: BattleAbility {
                    agility: 0,
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                },
                current_stats: BattleStats {
                    max_hp: 100,
                    max_sp: 10,
                    max_stamina: 10,
                    hp_damage: 0,
                    sp_damage: 0,
                    stamina_damage: 0,
                },
                defense_power: min_defense(),
                status_effects: vec![],
            },
        };
        let mut target = BattleCharacter::Player(&mut player);

        let mut skill_ap = zero_attack();
        skill_ap.slash = 12; // スキル基礎攻撃力

        // 実装ロジック: 攻撃力 = 武器攻撃 + (スキル攻撃 * スケーリング)
        // 武器を持たせ、スキル側のスケーリングも1.0にして合算を検証
        let mut scaling = AttackPowerScaling::default();
        scaling.slash = 1.0;

        // ダミー武器（攻撃力 5 を付与）
        let weapon = BattleWeapon {
            original: Rc::new(Weapon {
                kind: WeaponKind::StraightSword,
                weight: 1,
                ability_requirement: WeaponAbilityRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                attack_power: WeaponAttackPower {
                    base: AttackPower::default(),
                    ability_scaling: WeaponAttackPowerAbilityScaling {
                        slash: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        strike: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        thrust: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        impact: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        magic: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        fire: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        lightning: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        chaos: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                    },
                },
                sorcery_power: WeaponSorceryPower {
                    base: 1,
                    scaling: AbilityScaling {
                        strength: 0.0,
                        dexterity: 0.0,
                        intelligence: 0.0,
                        faith: 0.0,
                        arcane: 0.0,
                        agility: 0.0,
                    },
                },
                break_power: WeaponBreakPower {
                    base_power: 0,
                    scaling: AbilityScaling {
                        strength: 0.0,
                        dexterity: 0.0,
                        intelligence: 0.0,
                        faith: 0.0,
                        arcane: 0.0,
                        agility: 0.0,
                    },
                },
                guard: WeaponGuard {
                    cut_rate: GuardCutRate {
                        slash: 1.0,
                        strike: 1.0,
                        thrust: 1.0,
                        impact: 1.0,
                        magic: 1.0,
                        fire: 1.0,
                        lightning: 1.0,
                        chaos: 1.0,
                    },
                    guard_strength: 0,
                },
            }),
            attack_power: AttackPower {
                slash: 5,
                strike: 0,
                thrust: 0,
                impact: 0,
                magic: 0,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            sorcery_power: 1.0,
            break_power: 0,
        };

        let conduct = BattleConduct {
            actor_character_id: 100,
            target_character_id: 11,
            conduct: Conduct {
                name: "Skill Attack".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Skill(ConductTypeSkill {
                    usable_weapon_kinds: vec![],
                    potency: ConductTypeSkillPotency::Attack(ConductTypeSkillPotencyAttack {
                        attack_power: skill_ap,
                        attack_power_scaling: scaling,
                        break_power: 0,
                        break_power_scaling: 0.0,
                    }),
                }),
            },
            weapon: Some(weapon),
        };

        let result = conduct_effect(&conduct, &mut target);

        assert!(!result.is_evaded);
        assert!(!result.is_defended);
        if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
            // 期待値: weapon(5) + skill(12*1.0) = 17
            assert_eq!(d.damage, 17);
            assert_eq!(d.before, 0);
            assert_eq!(d.after, 17);
        } else {
            panic!("expected DamageHp incident");
        }
        assert_eq!(target.current_stats().hp_damage, 17);
    }

    // 術攻撃でHPダメージが適用されること
    #[test]
    fn test_conduct_effect_sorcery_attack_applies_damage() {
        let mut player = BattlePlayer {
            character_id: 100,
            original: dummy_player(),
            base: BattleCharacterBase {
                current_ability: BattleAbility {
                    agility: 0,
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                },
                current_stats: BattleStats {
                    max_hp: 100,
                    max_sp: 10,
                    max_stamina: 10,
                    hp_damage: 0,
                    sp_damage: 0,
                    stamina_damage: 0,
                },
                defense_power: min_defense(),
                status_effects: vec![],
            },
        };
        let mut target = BattleCharacter::Player(&mut player);
        let mut sorc_ap = zero_attack();
        sorc_ap.slash = 8; // 期待ダメージ8（weaponなし→術力1.0）

        let conduct = BattleConduct {
            actor_character_id: 100,
            target_character_id: 12,
            conduct: Conduct {
                name: "Sorcery Attack".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Sorcery(ConductTypeSorcery::Attack(
                    ConductTypeSorceryAttack {
                        attack_power: sorc_ap,
                        break_power: 0,
                    },
                )),
            },
            weapon: None, // weaponなし→術力1.0
        };

        let result = conduct_effect(&conduct, &mut target);

        assert!(!result.is_evaded);
        assert!(!result.is_defended);
        if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
            assert_eq!(d.damage, 8);
            assert_eq!(d.before, 0);
            assert_eq!(d.after, 8);
        } else {
            panic!("expected DamageHp incident");
        }
        assert_eq!(target.current_stats().hp_damage, 8);
    }

    // 技: スケーリング0.0なら武器攻撃力のみが寄与すること
    #[test]
    fn test_conduct_effect_skill_attack_with_weapon_zero_scaling() {
        let mut player = BattlePlayer {
            character_id: 100,
            original: dummy_player(),
            base: BattleCharacterBase {
                current_ability: BattleAbility {
                    agility: 0,
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                },
                current_stats: BattleStats {
                    max_hp: 100,
                    max_sp: 10,
                    max_stamina: 10,
                    hp_damage: 0,
                    sp_damage: 0,
                    stamina_damage: 0,
                },
                defense_power: min_defense(),
                status_effects: vec![],
            },
        };
        let mut target = BattleCharacter::Player(&mut player);

        // skill基礎攻撃力（高めに設定するが、スケーリング0.0なので無視される）
        let mut skill_ap = zero_attack();
        skill_ap.slash = 20;

        let scaling = AttackPowerScaling::default(); // 全属性0.0

        // ダミー武器（攻撃力 7 を付与）
        let weapon = BattleWeapon {
            original: Rc::new(Weapon {
                kind: WeaponKind::StraightSword,
                weight: 1,
                ability_requirement: WeaponAbilityRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                attack_power: WeaponAttackPower {
                    base: AttackPower::default(),
                    ability_scaling: WeaponAttackPowerAbilityScaling {
                        slash: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        strike: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        thrust: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        impact: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        magic: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        fire: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        lightning: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        chaos: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                    },
                },
                sorcery_power: WeaponSorceryPower {
                    base: 1,
                    scaling: AbilityScaling {
                        strength: 0.0,
                        dexterity: 0.0,
                        intelligence: 0.0,
                        faith: 0.0,
                        arcane: 0.0,
                        agility: 0.0,
                    },
                },
                break_power: WeaponBreakPower {
                    base_power: 0,
                    scaling: AbilityScaling {
                        strength: 0.0,
                        dexterity: 0.0,
                        intelligence: 0.0,
                        faith: 0.0,
                        arcane: 0.0,
                        agility: 0.0,
                    },
                },
                guard: WeaponGuard {
                    cut_rate: GuardCutRate {
                        slash: 1.0,
                        strike: 1.0,
                        thrust: 1.0,
                        impact: 1.0,
                        magic: 1.0,
                        fire: 1.0,
                        lightning: 1.0,
                        chaos: 1.0,
                    },
                    guard_strength: 0,
                },
            }),
            attack_power: AttackPower {
                slash: 7,
                strike: 0,
                thrust: 0,
                impact: 0,
                magic: 0,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            sorcery_power: 1.0,
            break_power: 0,
        };

        let conduct = BattleConduct {
            actor_character_id: 100,
            target_character_id: 13,
            conduct: Conduct {
                name: "Skill Attack Zero Scaling".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Skill(ConductTypeSkill {
                    usable_weapon_kinds: vec![],
                    potency: ConductTypeSkillPotency::Attack(ConductTypeSkillPotencyAttack {
                        attack_power: skill_ap,
                        attack_power_scaling: scaling, // 0.0
                        break_power: 0,
                        break_power_scaling: 0.0,
                    }),
                }),
            },
            weapon: Some(weapon),
        };

        let result = conduct_effect(&conduct, &mut target);
        if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
            assert_eq!(d.damage, 7); // 武器のみ寄与
            assert_eq!(d.before, 0);
            assert_eq!(d.after, 7);
        } else {
            panic!("expected DamageHp incident");
        }
        assert_eq!(target.current_stats().hp_damage, 7);
    }

    // 技: 複数属性の合算が正しく行われること
    #[test]
    fn test_conduct_effect_skill_attack_multi_attribute_sum() {
        let mut player = BattlePlayer {
            character_id: 100,
            original: dummy_player(),
            base: BattleCharacterBase {
                current_ability: BattleAbility {
                    agility: 0,
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                },
                current_stats: BattleStats {
                    max_hp: 100,
                    max_sp: 10,
                    max_stamina: 10,
                    hp_damage: 0,
                    sp_damage: 0,
                    stamina_damage: 0,
                },
                defense_power: min_defense(),
                status_effects: vec![],
            },
        };
        let mut target = BattleCharacter::Player(&mut player);

        // skill: slash=10, strike=6
        let mut skill_ap = zero_attack();
        skill_ap.slash = 10;
        skill_ap.strike = 6;

        let mut scaling = AttackPowerScaling::default();
        scaling.slash = 1.0;
        scaling.strike = 1.0;

        // weapon: slash=5, strike=4
        let weapon = BattleWeapon {
            original: Rc::new(Weapon {
                kind: WeaponKind::StraightSword,
                weight: 1,
                ability_requirement: WeaponAbilityRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                attack_power: WeaponAttackPower {
                    base: AttackPower::default(),
                    ability_scaling: WeaponAttackPowerAbilityScaling {
                        slash: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        strike: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        thrust: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        impact: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        magic: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        fire: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        lightning: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                        chaos: AbilityScaling {
                            strength: 0.0,
                            dexterity: 0.0,
                            intelligence: 0.0,
                            faith: 0.0,
                            arcane: 0.0,
                            agility: 0.0,
                        },
                    },
                },
                sorcery_power: WeaponSorceryPower {
                    base: 1,
                    scaling: AbilityScaling {
                        strength: 0.0,
                        dexterity: 0.0,
                        intelligence: 0.0,
                        faith: 0.0,
                        arcane: 0.0,
                        agility: 0.0,
                    },
                },
                break_power: WeaponBreakPower {
                    base_power: 0,
                    scaling: AbilityScaling {
                        strength: 0.0,
                        dexterity: 0.0,
                        intelligence: 0.0,
                        faith: 0.0,
                        arcane: 0.0,
                        agility: 0.0,
                    },
                },
                guard: WeaponGuard {
                    cut_rate: GuardCutRate {
                        slash: 1.0,
                        strike: 1.0,
                        thrust: 1.0,
                        impact: 1.0,
                        magic: 1.0,
                        fire: 1.0,
                        lightning: 1.0,
                        chaos: 1.0,
                    },
                    guard_strength: 0,
                },
            }),
            attack_power: AttackPower {
                slash: 5,
                strike: 4,
                thrust: 0,
                impact: 0,
                magic: 0,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            sorcery_power: 1.0,
            break_power: 0,
        };

        let conduct = BattleConduct {
            actor_character_id: 100,
            target_character_id: 14,
            conduct: Conduct {
                name: "Skill Attack Multi Attribute".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Skill(ConductTypeSkill {
                    usable_weapon_kinds: vec![],
                    potency: ConductTypeSkillPotency::Attack(ConductTypeSkillPotencyAttack {
                        attack_power: skill_ap,
                        attack_power_scaling: scaling, // 1.0 on slash & strike
                        break_power: 0,
                        break_power_scaling: 0.0,
                    }),
                }),
            },
            weapon: Some(weapon),
        };

        let result = conduct_effect(&conduct, &mut target);
        if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
            // 期待値: (slash 5 + 10) + (strike 4 + 6) = 25
            assert_eq!(d.damage, 25);
            assert_eq!(d.before, 0);
            assert_eq!(d.after, 25);
        } else {
            panic!("expected DamageHp incident");
        }
        assert_eq!(target.current_stats().hp_damage, 25);
    }
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
            if current_status.max_stamina - current_status.stamina_damage
                < conduct.conduct.stamina_cost
            {
                return Some(BattleIncidentConductOutcomeFailureReason {
                    insufficient_stamina: true,
                    insufficient_ability: false,
                    insufficient_sp: false,
                });
            }
        }
        BattleCharacter::Enemy(_) => {
            // 敵のスタミナ管理は省略
        }
    };

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
        });
    }

    // SPが足りないと不発
    let sp_cost = conduct.conduct.sp_cost;
    let current_sp = attacker.current_stats().max_sp - attacker.current_stats().sp_damage;
    if current_sp < sp_cost {
        return Some(BattleIncidentConductOutcomeFailureReason {
            insufficient_stamina: false,
            insufficient_ability: false,
            insufficient_sp: true,
        });
    }

    None
}
