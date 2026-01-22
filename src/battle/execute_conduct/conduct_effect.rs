use super::*;

pub fn conduct_effect(
    battle: &mut Battle,
    conduct: &BattleConduct,
    attcker_character_id: &BattleCharacterId,
    target_charackter_id: &BattleCharacterId,
) -> BattleIncidentConductOutcomeSuccessDefender {
    let attacker = if let Some(c) = battle.character(&conduct.actor_character_id) {
        c
    } else {
        // TODO: エラー処理
        panic!("Attacker character not found");
    };

    // 能力補正済みの武器性能取得
    let attacker_weapon_performance = if let Some(battle_weapon_id) = &conduct.battle_weapon_id {
        if let Some(performance) = attacker.weapon_performance(&battle_weapon_id) {
            performance
        } else {
            // TODO: エラー処理
            panic!("Attacker weapon performance not found");
        }
    } else {
        // 武器なし
        // TODO: 素手攻撃の性能を返すようにする
        panic!("not implemented");
        // BattleWeaponPerformance::default()
    };

    let target = if let Some(c) = battle.character_mut(target_charackter_id) {
        c
    } else {
        // TODO: エラー処理
        panic!("Target character not found");
    };

    // 回避判定
    for se in target.status_conditions.iter() {
        match &se.potency {
            StatusConditionPotency::Evasion => {
                // 回避効果処理
                return BattleIncidentConductOutcomeSuccessDefender {
                    character_id: target.character_id,
                    stats_changes: Vec::new(),
                    status_conditions: Vec::new(),
                    is_defended: false,
                    is_evaded: true,
                };
            }
            StatusConditionPotency::Airborne => {
                // 空中効果処理
                // 遠距離攻撃でない時は回避
                if !conduct.art.perks.contains(&ArtPerk::Ranged) {
                    return BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.character_id,
                        stats_changes: Vec::new(),
                        status_conditions: Vec::new(),
                        is_defended: false,
                        is_evaded: true,
                    };
                }
            }
            StatusConditionPotency::Floating => {
                // 浮遊効果処理
                // 足元攻撃は回避
                if conduct.art.perks.contains(&ArtPerk::AtFeet) {
                    return BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.character_id,
                        stats_changes: Vec::new(),
                        status_conditions: Vec::new(),
                        is_defended: false,
                        is_evaded: true,
                    };
                }
            }
            StatusConditionPotency::Ranged => {
                // 遠距離効果処理
                // 近距離の攻撃を回避
                if !conduct.art.perks.contains(&ArtPerk::Ranged) {
                    return BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.character_id,
                        stats_changes: Vec::new(),
                        status_conditions: Vec::new(),
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

    let sorcery_power = attacker_weapon_performance.final_sorcery_power();
    // 効果ランク判定
    // 術力のみの想定なので術力でランク判定
    let rank = if let Some(rank3) = &conduct.art.rank3
        && sorcery_power >= rank3.threshold
    {
        // ランク3適用
        rank3
    } else if let Some(rank2) = &conduct.art.rank2
        && sorcery_power >= rank2.threshold
    {
        // ランク2適用
        rank2
    } else {
        // ランク1適用
        &conduct.art.rank1
    };

    match &rank.potency {
        ArtPotency::Attack(art_attack) => {
            let mut stats_change_incidents = Vec::new();
            let mut status_condition_incidents = Vec::new();

            let weapon_attack_power = &attacker_weapon_performance.attack_power;
            let weapon_break_power = attacker_weapon_performance.break_power;
            let art_attack_power = &art_attack.attack_power;
            let weapon_attack_power_scaling = &art_attack.weapon_attack_power_scaling;
            let mut attack_power = calc_attack_power_modifier(
                art_attack_power,
                weapon_attack_power,
                weapon_attack_power_scaling,
            );

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
                        attack_power =
                            calc_attack_power_cut_rate(&attack_power, &resistance.cut_rate);
                        is_defended = true;
                    }
                    _ => {
                        // その他
                    }
                }
            }

            // ブレイク攻撃力算出
            let break_power = art_attack.break_power
                + (weapon_break_power as f32 * art_attack.weapon_break_power_scaling) as u32;

            // ダメージ
            let damage = calc_damage(&attack_power, &target.defense_power());
            let (before_hp_damage, after_hp_damage) = target.hp.damage(damage);
            // HPダメージのインシデント
            stats_change_incidents.push(BattleIncidentStats::DamageHp(BattleIncidentDamageHp {
                damage,
                before: before_hp_damage,
                after: after_hp_damage,
            }));

            // 防御時のスタミナダメージ
            if is_defended {
                let stamina_damage = damage / 4; // TODO: 固定値ではなく防御力依存にする
                let (before_stamina, after_stamina) = target.stamina.damage(stamina_damage);

                // スタミナダメージのインシデント
                stats_change_incidents.push(BattleIncidentStats::DamageStamina(
                    BattleIncidentDamageStamina {
                        damage: stamina_damage,
                        before: before_stamina,
                        after: after_stamina,
                    },
                ));
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
                    let (before_break, after_break) = target.break_resistance.damage(break_power);

                    if after_break == 0 {
                        // ブレイク状態にする
                        let new_status_conditions = support_status_effect(
                            &vec![StatusCondition {
                                potency: StatusConditionPotency::Break(StatusConditionBreak {}),
                                duration: StatusConditionDuration::Permanent,
                            }],
                            target,
                        );
                        status_condition_incidents.extend(new_status_conditions);
                    }

                    // ブレイクダメージインシデント追加
                    stats_change_incidents.push(BattleIncidentStats::DamageBreak(
                        BattleIncidentDamageBreak {
                            damage: break_power,
                            before: before_break,
                            after: after_break,
                        },
                    ));
                }
            }

            BattleIncidentConductOutcomeSuccessDefender {
                character_id: target.character_id,
                stats_changes: stats_change_incidents,
                status_conditions: Vec::new(),
                is_defended,
                is_evaded: false,
            }
        }
        ArtPotency::Support(support) => {
            // 支援処理
            match &support {
                ArtPotencySupport::StatusCondition(status_condition) => {
                    let new_incidents =
                        support_status_effect(&status_condition.status_conditions, target);

                    BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.character_id,
                        stats_changes: Vec::new(),
                        status_conditions: new_incidents,
                        is_defended: false,
                        is_evaded: false,
                    }
                }
                ArtPotencySupport::Recover(recover) => {
                    let stats_change_incidents = support_recover(recover, target);

                    BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.character_id,
                        stats_changes: stats_change_incidents,
                        status_conditions: Vec::new(),
                        is_defended: false,
                        is_evaded: false,
                    }
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // ヘルパー: ダミーのプレイヤー原本
//     fn dummy_player() -> Arc<Player> {
//         Arc::new(Player {
//             ability: PlayerAbility {
//                 vitality: 0,
//                 spirit: 0,
//                 endurance: 0,
//                 agility: 0,
//                 strength: 0,
//                 dexterity: 0,
//                 intelligence: 0,
//                 faith: 0,
//                 arcane: 0,
//             },
//             stats: PlayerStats {
//                 hp: 100,
//                 sp: 10,
//                 stamina: 10,
//                 stamina_recovery: 1,
//                 equip_load: 0,
//             },
//             base_defense_power: DefensePower {
//                 slash: 0,
//                 strike: 0,
//                 thrust: 0,
//                 impact: 0,
//                 magic: 0,
//                 fire: 0,
//                 lightning: 0,
//                 chaos: 0,
//             },
//             equipment: Equipment {
//                 weapon1: None,
//                 weapon2: None,
//                 armor1: None,
//                 armor2: None,
//                 armor3: None,
//                 armor4: None,
//                 armor5: None,
//                 armor6: None,
//                 armor7: None,
//                 armor8: None,
//             },
//             arts: vec![],
//         })
//     }

//     // ヘルパー: 最低限の防御力(0除算を避けるため全て1)
//     fn min_defense() -> DefensePower {
//         DefensePower {
//             slash: 1,
//             strike: 1,
//             thrust: 1,
//             impact: 1,
//             magic: 1,
//             fire: 1,
//             lightning: 1,
//             chaos: 1,
//         }
//     }

//     // ヘルパー: デフォルト攻撃力(全て0)
//     fn zero_attack() -> AttackPower {
//         AttackPower::default()
//     }

//     // conduct_effect: 回避(Evasion)で早期リターンすること
//     #[test]
//     fn test_conduct_effect_evades_with_evasion() {
//         // ターゲット: プレイヤーで回避状態
//         let mut player = BattlePlayer {
//             character_id: 1,
//             original: dummy_player(),
//             base: BattleCharacterBase {
//                 current_ability: BattleAbility {
//                     agility: 0,
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                 },
//                 current_stats: BattleStats {
//                     max_hp: 100,
//                     max_sp: 10,
//                     max_stamina: 10,
//                     stamina_recovery: 1,
//                     current_hp: 100,
//                     current_sp: 10,
//                     current_stamina: 10,
//                 },
//                 defense_power: min_defense(),
//                 status_conditions: vec![BattleStatusCondition {
//                     potency: StatusConditionPotency::Evasion,
//                     duration: BattleStatusConditionDuration::Permanent,
//                 }],

//                 is_dead: false,
//             },
//         };
//         let mut target = BattleCharacter::Player(&mut player);

//         // 行動: 基本攻撃(近接)で十分
//         let conduct = BattleConduct {
//             actor_character_id: 100,
//             target_character_id: 1,
//             art: Arc::new(Art {
//                 name: "Basic Attack".to_string(),
//                 sp_cost: 0,
//                 stamina_cost: 0,
//                 perks: vec![ArtPerk::Melee],
//                 requirement: ArtRequirement {
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                     agility: 0,
//                 },
//                 art_type: ArtType::Basic,
//                 usable_weapon: ArtUsableWeapon::All,
//                 rank1: ArtRank {
//                     threshold: 0,
//                     target: ArtTarget::Single,
//                     potency: ArtPotency::Attack(ArtPotencyAttack {
//                         attack_power: zero_attack(),
//                         break_power: 0,
//                         weapon_attack_power_scaling: AttackPowerScaling::default(),
//                         weapon_break_power_scaling: 0.0,
//                     }),
//                 },
//                 rank2: None,
//                 rank3: None,
//             }),
//             weapon: None,
//         };

//         let result = conduct_effect(&conduct, &mut target);

//         assert!(result.is_evaded);
//         assert!(!result.is_defended);
//         assert!(result.stats_changes.is_empty());
//         assert!(result.status_conditions.is_empty());
//     }

//     // conduct_effect: 非回避ルート（基本攻撃・攻撃力0）は回避せず、HPダメージ0の適用結果を返す
//     #[test]
//     fn test_conduct_effect_basic_attack_zero_damage() {
//         // ターゲット: プレイヤー(状態変化なし)
//         let mut player = BattlePlayer {
//             character_id: 2,
//             original: dummy_player(),
//             base: BattleCharacterBase {
//                 current_ability: BattleAbility {
//                     agility: 0,
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                 },
//                 current_stats: BattleStats {
//                     max_hp: 100,
//                     max_sp: 10,
//                     max_stamina: 10,
//                     stamina_recovery: 1,
//                     current_hp: 100,
//                     current_sp: 10,
//                     current_stamina: 10,
//                 },
//                 defense_power: min_defense(),
//                 status_conditions: vec![],

//                 is_dead: false,
//             },
//         };
//         let mut target = BattleCharacter::Player(&mut player);

//         // 行動: 基本攻撃(近接)
//         let conduct = BattleConduct {
//             actor_character_id: 100,
//             target_character_id: 2,
//             art: Arc::new(Art {
//                 name: "Basic Attack".to_string(),
//                 sp_cost: 0,
//                 stamina_cost: 0,
//                 perks: vec![ArtPerk::Melee],
//                 requirement: ArtRequirement {
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                     agility: 0,
//                 },
//                 art_type: ArtType::Basic,
//                 usable_weapon: ArtUsableWeapon::All,
//                 rank1: ArtRank {
//                     threshold: 0,
//                     target: ArtTarget::Single,
//                     potency: ArtPotency::Attack(ArtPotencyAttack {
//                         attack_power: zero_attack(),
//                         break_power: 0,
//                         weapon_attack_power_scaling: AttackPowerScaling::default(),
//                         weapon_break_power_scaling: 0.0,
//                     }),
//                 },
//                 rank2: None,
//                 rank3: None,
//             }),
//             weapon: None,
//         };

//         let result = conduct_effect(&conduct, &mut target);

//         // 回避・防御なし
//         assert!(!result.is_evaded);
//         assert!(!result.is_defended);
//         // 状態変化なし
//         assert!(result.status_conditions.is_empty());
//         // HPダメージ0が記録されていること
//         assert_eq!(result.stats_changes.len(), 1);
//         match &result.stats_changes[0] {
//             BattleIncidentStats::DamageHp(d) => {
//                 assert_eq!(d.damage, 0);
//                 assert_eq!(d.before, 100);
//                 assert_eq!(d.after, 100);
//             }
//             _ => panic!("expected DamageHp incident"),
//         }
//         // 実際のターゲットのHPダメージも0のまま
//         assert_eq!(target.current_stats().current_hp, 100);
//     }

//     // 基本攻撃でHPダメージが適用されること
//     #[test]
//     fn test_conduct_effect_basic_attack_applies_damage() {
//         let mut player = BattlePlayer {
//             character_id: 100,
//             original: dummy_player(),
//             base: BattleCharacterBase {
//                 current_ability: BattleAbility {
//                     agility: 0,
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                 },
//                 current_stats: BattleStats {
//                     max_hp: 100,
//                     max_sp: 10,
//                     max_stamina: 10,
//                     stamina_recovery: 1,
//                     current_hp: 100,
//                     current_sp: 10,
//                     current_stamina: 10,
//                 },
//                 defense_power: min_defense(),
//                 status_conditions: vec![],

//                 is_dead: false,
//             },
//         };
//         let mut target = BattleCharacter::Player(&mut player);

//         let mut atk = zero_attack();
//         atk.slash = 10; // 期待ダメージ10

//         let conduct = BattleConduct {
//             actor_character_id: 100,
//             target_character_id: 10,
//             art: Arc::new(Art {
//                 name: "Basic Attack".to_string(),
//                 sp_cost: 0,
//                 stamina_cost: 0,
//                 perks: vec![ArtPerk::Melee],
//                 requirement: ArtRequirement {
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                     agility: 0,
//                 },
//                 art_type: ArtType::Basic,
//                 usable_weapon: ArtUsableWeapon::All,
//                 rank1: ArtRank {
//                     threshold: 0,
//                     target: ArtTarget::Single,
//                     potency: ArtPotency::Attack(ArtPotencyAttack {
//                         attack_power: atk.clone(),
//                         break_power: 0,
//                         weapon_attack_power_scaling: AttackPowerScaling::default(),
//                         weapon_break_power_scaling: 0.0,
//                     }),
//                 },
//                 rank2: None,
//                 rank3: None,
//             }),
//             weapon: None,
//         };

//         let result = conduct_effect(&conduct, &mut target);

//         assert!(!result.is_evaded);
//         assert!(!result.is_defended);
//         assert!(matches!(
//             result.stats_changes.get(0),
//             Some(BattleIncidentStats::DamageHp(_))
//         ));
//         if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
//             assert_eq!(d.damage, 10);
//             assert_eq!(d.before, 100);
//             assert_eq!(d.after, 90);
//         } else {
//             panic!("expected DamageHp incident");
//         }
//         assert_eq!(target.current_stats().current_hp, 90);
//     }

//     // 技攻撃でHPダメージが適用されること
//     #[test]
//     fn test_conduct_effect_skill_attack_applies_damage() {
//         let mut player = BattlePlayer {
//             character_id: 100,
//             original: dummy_player(),
//             base: BattleCharacterBase {
//                 current_ability: BattleAbility {
//                     agility: 0,
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                 },
//                 current_stats: BattleStats {
//                     max_hp: 100,
//                     max_sp: 10,
//                     max_stamina: 10,
//                     stamina_recovery: 1,
//                     current_hp: 100,
//                     current_sp: 10,
//                     current_stamina: 10,
//                 },
//                 defense_power: min_defense(),
//                 status_conditions: vec![],

//                 is_dead: false,
//             },
//         };
//         let mut target = BattleCharacter::Player(&mut player);

//         let mut skill_ap = zero_attack();
//         skill_ap.slash = 12; // スキル基礎攻撃力

//         // 実装ロジック: 攻撃力 = 武器攻撃 + (スキル攻撃 * スケーリング)
//         // 武器を持たせ、スキル側のスケーリングも1.0にして合算を検証
//         let mut scaling = AttackPowerScaling::default();
//         scaling.slash = 1.0;

//         // ダミー武器（攻撃力 5 を付与）
//         let weapon = BattleWeapon {
//             original: Arc::new(Weapon {
//                 kind: WeaponKind::StraightSword,
//                 weight: 1,
//                 ability_requirement: WeaponAbilityRequirement {
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                     agility: 0,
//                 },
//                 attack_power: WeaponAttackPower {
//                     base: AttackPower::default(),
//                     ability_scaling: WeaponAttackPowerAbilityScaling {
//                         slash: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         strike: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         thrust: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         impact: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         magic: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         fire: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         lightning: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         chaos: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                     },
//                 },
//                 sorcery_power: WeaponSorceryPower {
//                     base: 1,
//                     scaling: AbilityScaling {
//                         strength: 0.0,
//                         dexterity: 0.0,
//                         intelligence: 0.0,
//                         faith: 0.0,
//                         arcane: 0.0,
//                         agility: 0.0,
//                     },
//                 },
//                 break_power: WeaponBreakPower {
//                     base_power: 0,
//                     scaling: AbilityScaling {
//                         strength: 0.0,
//                         dexterity: 0.0,
//                         intelligence: 0.0,
//                         faith: 0.0,
//                         arcane: 0.0,
//                         agility: 0.0,
//                     },
//                 },
//                 guard: WeaponGuard {
//                     cut_rate: GuardCutRate {
//                         slash: 1.0,
//                         strike: 1.0,
//                         thrust: 1.0,
//                         impact: 1.0,
//                         magic: 1.0,
//                         fire: 1.0,
//                         lightning: 1.0,
//                         chaos: 1.0,
//                     },
//                     guard_strength: 0,
//                 },
//             }),
//             attack_power: AttackPower {
//                 slash: 5,
//                 strike: 0,
//                 thrust: 0,
//                 impact: 0,
//                 magic: 0,
//                 fire: 0,
//                 lightning: 0,
//                 chaos: 0,
//             },
//             sorcery_power: 1.0,
//             break_power: 0,
//         };

//         let conduct = BattleConduct {
//             actor_character_id: 100,
//             target_character_id: 11,
//             art: Arc::new(Art {
//                 name: "Skill Attack".to_string(),
//                 sp_cost: 0,
//                 stamina_cost: 0,
//                 perks: vec![ArtPerk::Melee],
//                 requirement: ArtRequirement {
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                     agility: 0,
//                 },
//                 art_type: ArtType::Skill,
//                 usable_weapon: ArtUsableWeapon::All,
//                 rank1: ArtRank {
//                     threshold: 0,
//                     target: ArtTarget::Single,
//                     potency: ArtPotency::Attack(ArtPotencyAttack {
//                         attack_power: skill_ap,
//                         weapon_attack_power_scaling: scaling,
//                         break_power: 0,
//                         weapon_break_power_scaling: 0.0,
//                     }),
//                 },
//                 rank2: None,
//                 rank3: None,
//             }),
//             weapon: Some(weapon),
//         };

//         let result = conduct_effect(&conduct, &mut target);

//         assert!(!result.is_evaded);
//         assert!(!result.is_defended);
//         if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
//             // 期待値: weapon(5) + skill(12*1.0) = 17
//             assert_eq!(d.damage, 17);
//             assert_eq!(d.before, 100);
//             assert_eq!(d.after, 83);
//         } else {
//             panic!("expected DamageHp incident");
//         }
//         assert_eq!(target.current_stats().current_hp, 83);
//     }

//     // 術攻撃でHPダメージが適用されること
//     #[test]
//     fn test_conduct_effect_sorcery_attack_applies_damage() {
//         let mut player = BattlePlayer {
//             character_id: 100,
//             original: dummy_player(),
//             base: BattleCharacterBase {
//                 current_ability: BattleAbility {
//                     agility: 0,
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                 },
//                 current_stats: BattleStats {
//                     max_hp: 100,
//                     max_sp: 10,
//                     max_stamina: 10,
//                     stamina_recovery: 1,
//                     current_hp: 100,
//                     current_sp: 10,
//                     current_stamina: 10,
//                 },
//                 defense_power: min_defense(),
//                 status_conditions: vec![],
//                 is_dead: false,
//             },
//         };
//         let mut target = BattleCharacter::Player(&mut player);
//         let mut sorc_ap = zero_attack();
//         sorc_ap.slash = 8; // 期待ダメージ8（weaponなし→術力1.0）

//         let conduct = BattleConduct {
//             actor_character_id: 100,
//             target_character_id: 12,
//             art: Arc::new(Art {
//                 name: "Sorcery Attack".to_string(),
//                 sp_cost: 0,
//                 stamina_cost: 0,
//                 perks: vec![ArtPerk::Melee],
//                 requirement: ArtRequirement {
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                     agility: 0,
//                 },
//                 art_type: ArtType::Sorcery,
//                 usable_weapon: ArtUsableWeapon::All,
//                 rank1: ArtRank {
//                     threshold: 0,
//                     target: ArtTarget::Single,
//                     potency: ArtPotency::Attack(ArtPotencyAttack {
//                         attack_power: sorc_ap,
//                         weapon_attack_power_scaling: AttackPowerScaling::default(),
//                         break_power: 0,
//                         weapon_break_power_scaling: 0.0,
//                     }),
//                 },
//                 rank2: None,
//                 rank3: None,
//             }),
//             weapon: None, // weaponなし→術力1.0
//         };

//         let result = conduct_effect(&conduct, &mut target);

//         assert!(!result.is_evaded);
//         assert!(!result.is_defended);
//         if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
//             assert_eq!(d.damage, 8);
//             assert_eq!(d.before, 100);
//             assert_eq!(d.after, 92);
//         } else {
//             panic!("expected DamageHp incident");
//         }
//         assert_eq!(target.current_stats().current_hp, 92);
//     }

//     // 技: スケーリング0.0なら武器攻撃力のみが寄与すること
//     #[test]
//     fn test_conduct_effect_skill_attack_with_weapon_zero_scaling() {
//         let mut player = BattlePlayer {
//             character_id: 100,
//             original: dummy_player(),
//             base: BattleCharacterBase {
//                 current_ability: BattleAbility {
//                     agility: 0,
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                 },
//                 current_stats: BattleStats {
//                     max_hp: 100,
//                     max_sp: 10,
//                     max_stamina: 10,
//                     stamina_recovery: 1,
//                     current_hp: 100,
//                     current_sp: 10,
//                     current_stamina: 10,
//                 },
//                 defense_power: min_defense(),
//                 status_conditions: vec![],
//                 is_dead: false,
//             },
//         };
//         let mut target = BattleCharacter::Player(&mut player);

//         // skill基礎攻撃力（高めに設定するが、スケーリング0.0なので無視される）
//         let mut skill_ap = zero_attack();
//         skill_ap.slash = 20;

//         let scaling = AttackPowerScaling::default(); // 全属性0.0

//         // ダミー武器（攻撃力 7 を付与）
//         let weapon = BattleWeapon {
//             original: Arc::new(Weapon {
//                 kind: WeaponKind::StraightSword,
//                 weight: 1,
//                 ability_requirement: WeaponAbilityRequirement {
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                     agility: 0,
//                 },
//                 attack_power: WeaponAttackPower {
//                     base: AttackPower::default(),
//                     ability_scaling: WeaponAttackPowerAbilityScaling {
//                         slash: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         strike: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         thrust: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         impact: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         magic: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         fire: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         lightning: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         chaos: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                     },
//                 },
//                 sorcery_power: WeaponSorceryPower {
//                     base: 1,
//                     scaling: AbilityScaling {
//                         strength: 0.0,
//                         dexterity: 0.0,
//                         intelligence: 0.0,
//                         faith: 0.0,
//                         arcane: 0.0,
//                         agility: 0.0,
//                     },
//                 },
//                 break_power: WeaponBreakPower {
//                     base_power: 0,
//                     scaling: AbilityScaling {
//                         strength: 0.0,
//                         dexterity: 0.0,
//                         intelligence: 0.0,
//                         faith: 0.0,
//                         arcane: 0.0,
//                         agility: 0.0,
//                     },
//                 },
//                 guard: WeaponGuard {
//                     cut_rate: GuardCutRate {
//                         slash: 1.0,
//                         strike: 1.0,
//                         thrust: 1.0,
//                         impact: 1.0,
//                         magic: 1.0,
//                         fire: 1.0,
//                         lightning: 1.0,
//                         chaos: 1.0,
//                     },
//                     guard_strength: 0,
//                 },
//             }),
//             attack_power: AttackPower {
//                 slash: 7,
//                 strike: 0,
//                 thrust: 0,
//                 impact: 0,
//                 magic: 0,
//                 fire: 0,
//                 lightning: 0,
//                 chaos: 0,
//             },
//             sorcery_power: 1.0,
//             break_power: 0,
//         };

//         let conduct = BattleConduct {
//             actor_character_id: 100,
//             target_character_id: 13,
//             art: Arc::new(Art {
//                 name: "Skill Attack Zero Scaling".to_string(),
//                 sp_cost: 0,
//                 stamina_cost: 0,
//                 perks: vec![ArtPerk::Melee],
//                 requirement: ArtRequirement {
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                     agility: 0,
//                 },
//                 art_type: ArtType::Skill,
//                 usable_weapon: ArtUsableWeapon::All,
//                 rank1: ArtRank {
//                     threshold: 0,
//                     target: ArtTarget::Single,
//                     potency: ArtPotency::Attack(ArtPotencyAttack {
//                         attack_power: skill_ap,
//                         weapon_attack_power_scaling: scaling, // 0.0
//                         break_power: 0,
//                         weapon_break_power_scaling: 0.0,
//                     }),
//                 },
//                 rank2: None,
//                 rank3: None,
//             }),
//             weapon: Some(weapon),
//         };

//         let result = conduct_effect(&conduct, &mut target);
//         if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
//             assert_eq!(d.damage, 20); // 武器のみ寄与
//             assert_eq!(d.before, 100);
//             assert_eq!(d.after, 80);
//         } else {
//             panic!("expected DamageHp incident");
//         }
//         assert_eq!(target.current_stats().current_hp, 80);
//     }

//     // 技: 複数属性の合算が正しく行われること
//     #[test]
//     fn test_conduct_effect_skill_attack_multi_attribute_sum() {
//         let mut player = BattlePlayer {
//             character_id: 100,
//             original: dummy_player(),
//             base: BattleCharacterBase {
//                 current_ability: BattleAbility {
//                     agility: 0,
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                 },
//                 current_stats: BattleStats {
//                     max_hp: 100,
//                     max_sp: 10,
//                     max_stamina: 10,
//                     stamina_recovery: 1,
//                     current_hp: 100,
//                     current_sp: 10,
//                     current_stamina: 10,
//                 },
//                 defense_power: min_defense(),
//                 status_conditions: vec![],
//                 is_dead: false,
//             },
//         };
//         let mut target = BattleCharacter::Player(&mut player);

//         // skill: slash=10, strike=6
//         let mut skill_ap = zero_attack();
//         skill_ap.slash = 10;
//         skill_ap.strike = 6;

//         let mut scaling = AttackPowerScaling::default();
//         scaling.slash = 1.0;
//         scaling.strike = 1.0;

//         // weapon: slash=5, strike=4
//         let weapon = BattleWeapon {
//             original: Arc::new(Weapon {
//                 kind: WeaponKind::StraightSword,
//                 weight: 1,
//                 ability_requirement: WeaponAbilityRequirement {
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                     agility: 0,
//                 },
//                 attack_power: WeaponAttackPower {
//                     base: AttackPower::default(),
//                     ability_scaling: WeaponAttackPowerAbilityScaling {
//                         slash: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         strike: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         thrust: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         impact: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         magic: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         fire: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         lightning: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                         chaos: AbilityScaling {
//                             strength: 0.0,
//                             dexterity: 0.0,
//                             intelligence: 0.0,
//                             faith: 0.0,
//                             arcane: 0.0,
//                             agility: 0.0,
//                         },
//                     },
//                 },
//                 sorcery_power: WeaponSorceryPower {
//                     base: 1,
//                     scaling: AbilityScaling {
//                         strength: 0.0,
//                         dexterity: 0.0,
//                         intelligence: 0.0,
//                         faith: 0.0,
//                         arcane: 0.0,
//                         agility: 0.0,
//                     },
//                 },
//                 break_power: WeaponBreakPower {
//                     base_power: 0,
//                     scaling: AbilityScaling {
//                         strength: 0.0,
//                         dexterity: 0.0,
//                         intelligence: 0.0,
//                         faith: 0.0,
//                         arcane: 0.0,
//                         agility: 0.0,
//                     },
//                 },
//                 guard: WeaponGuard {
//                     cut_rate: GuardCutRate {
//                         slash: 1.0,
//                         strike: 1.0,
//                         thrust: 1.0,
//                         impact: 1.0,
//                         magic: 1.0,
//                         fire: 1.0,
//                         lightning: 1.0,
//                         chaos: 1.0,
//                     },
//                     guard_strength: 0,
//                 },
//             }),
//             attack_power: AttackPower {
//                 slash: 5,
//                 strike: 4,
//                 thrust: 0,
//                 impact: 0,
//                 magic: 0,
//                 fire: 0,
//                 lightning: 0,
//                 chaos: 0,
//             },
//             sorcery_power: 1.0,
//             break_power: 0,
//         };

//         let conduct = BattleConduct {
//             actor_character_id: 100,
//             target_character_id: 14,
//             art: Arc::new(Art {
//                 name: "Skill Attack Multi Attribute".to_string(),
//                 sp_cost: 0,
//                 stamina_cost: 0,
//                 perks: vec![ArtPerk::Melee],
//                 requirement: ArtRequirement {
//                     strength: 0,
//                     dexterity: 0,
//                     intelligence: 0,
//                     faith: 0,
//                     arcane: 0,
//                     agility: 0,
//                 },
//                 art_type: ArtType::Skill,
//                 usable_weapon: ArtUsableWeapon::All,
//                 rank1: ArtRank {
//                     threshold: 0,
//                     target: ArtTarget::Single,
//                     potency: ArtPotency::Attack(ArtPotencyAttack {
//                         attack_power: skill_ap,
//                         weapon_attack_power_scaling: scaling, // 1.0 on slash & strike
//                         break_power: 0,
//                         weapon_break_power_scaling: 0.0,
//                     }),
//                 },
//                 rank2: None,
//                 rank3: None,
//             }),
//             weapon: Some(weapon),
//         };

//         let result = conduct_effect(&conduct, &mut target);
//         if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
//             // 期待値: (slash 5 + 10) + (strike 4 + 6) = 25
//             assert_eq!(d.damage, 25);
//             assert_eq!(d.before, 100);
//             assert_eq!(d.after, 75);
//         } else {
//             panic!("expected DamageHp incident");
//         }
//         assert_eq!(target.current_stats().current_hp, 75);
//     }
// }
