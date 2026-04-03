use super::*;

pub fn execute_attack_potency(
    art_attack: &ArtPotencyAttack,
    attacker_data: &AttackerData,
    target_data: TargetData,
) -> Vec<BattleCharacterIncidentConcrete> {
    let target = target_data.target;
    let target_effects = target_data.effects;

    let target_ability = target.ability_with_effects(&target_effects);

    let mut incidents = vec![];

    // 武器性能取得
    let weapon_attack_power = attacker_data.weapon_performance.final_attack_power();
    // アーツ攻撃力算出
    let mut attack_power = art_attack.final_attack_power(&weapon_attack_power);

    // 術力の攻撃力補正
    let sorcery_power = attacker_data.weapon_performance.sorcery_power;
    if attacker_data.conduct.art.art_type == ArtType::Sorcery {
        // 魔法タイプの場合、術力補正をかける
        let sorcery_attack_power_rate = 1.0 + (sorcery_power as f32 / 100.0);
        attack_power.multiply(sorcery_attack_power_rate);
    }

    // ブレイク力算出
    let weapon_break_power = attacker_data.weapon_performance.final_break_power();
    let break_power = art_attack.final_break_power(weapon_break_power);

    // 防御時の攻撃力カット処理
    for se in target.status_conditions.iter() {
        match &se.potency {
            StatusConditionPotency::Resistance(resistance) => {
                let battle_weapon_id = &resistance.battle_weapon_id;
                if let Some(weapon) = target.weapon(battle_weapon_id) {
                    let performance = weapon.weapon.performance(&target_ability);

                    // 防御時の攻撃力カット処理
                    attack_power = weapon.weapon.guard.cut_rate.apply_guard_cut(&attack_power);

                    // 防御時のスタミナダメージ
                    // ガードで使用する武器の現状の武器性能でガード強度を取得する
                    let sta_damage = 5 + (break_power / performance.guard_strength.max(1));
                    // 武器名を借用解放前にクローン
                    let guard_weapon_name = weapon.weapon.name.clone();
                    // weapon の借用ここで終了

                    let (before_sta, after_sta) = target.stamina.damage(sta_damage);

                    // ガード成功時のコンビネーションログ
                    if let Some(combination_skill) = &mut target.combination_skill {
                        combination_skill
                            .add_current_conduct_result(CombinationConductResult::GuardSuccess);
                    }

                    // ガード成功インシデント（武器名・消費スタミナを保持）
                    incidents.push(BattleCharacterIncidentConcrete::GuardSuccess(
                        BattleIncidentGuardSuccess {
                            weapon_name: guard_weapon_name,
                            stamina_consumed: sta_damage,
                        },
                    ));

                    // スタミナダメージのインシデント
                    incidents.push(BattleCharacterIncidentConcrete::DamageStamina(
                        BattleIncidentDamageStamina::new(sta_damage, before_sta, after_sta),
                    ));
                } else {
                    // TODO: エラー処理
                    panic!(
                        "防御に使用する武器が見つかりませんでした: {:?}",
                        battle_weapon_id
                    );
                };
            }
            _ => {
                // その他
            }
        }
    }

    // 攻撃側の効果から攻撃力補正を反映する
    attacker_data.effects.iter().for_each(|effect| {
        match effect {
            Effect::PhysicalAttackModifier(modifier) => {
                // 物理攻撃力補正
                attack_power.multiply_attribute(&Attribute::Slash, modifier.modifier);
                attack_power.multiply_attribute(&Attribute::Strike, modifier.modifier);
                attack_power.multiply_attribute(&Attribute::Thrust, modifier.modifier);
                attack_power.multiply_attribute(&Attribute::Impact, modifier.modifier);
            }
            Effect::MagicalAttackModifier(modifier) => {
                // 魔法攻撃力補正
                attack_power.multiply_attribute(&Attribute::Magic, modifier.modifier);
                attack_power.multiply_attribute(&Attribute::Fire, modifier.modifier);
                attack_power.multiply_attribute(&Attribute::Lightning, modifier.modifier);
                attack_power.multiply_attribute(&Attribute::Chaos, modifier.modifier);
            }
            _ => {
                // その他
            }
        }
    });

    // 防御力取得
    // TODO: 能力補正は外だしするか考える
    let mut target_defense_power = target.defense_power_with_effects(&target_effects);

    // 防御側の効果から防御力補正を反映する
    target_effects.iter().for_each(|effect| {
        match effect {
            Effect::PhysicalDefenseModifier(modifier) => {
                // 物理防御力補正
                target_defense_power.multiply_attribute(&Attribute::Slash, modifier.modifier);
                target_defense_power.multiply_attribute(&Attribute::Strike, modifier.modifier);
                target_defense_power.multiply_attribute(&Attribute::Thrust, modifier.modifier);
                target_defense_power.multiply_attribute(&Attribute::Impact, modifier.modifier);
            }
            Effect::MagicalDefenseModifier(modifier) => {
                // 魔法防御力補正
                target_defense_power.multiply_attribute(&Attribute::Magic, modifier.modifier);
                target_defense_power.multiply_attribute(&Attribute::Fire, modifier.modifier);
                target_defense_power.multiply_attribute(&Attribute::Lightning, modifier.modifier);
                target_defense_power.multiply_attribute(&Attribute::Chaos, modifier.modifier);
            }
            _ => {
                // その他
            }
        }
    });

    // 調整のための補正
    // TODO: 後で整理する
    attack_power.multiply(8.0);

    println!("攻撃力: {attack_power:?}");
    // ダメージ
    let mut damage = calc_damage(&attack_power, &target_defense_power);
    // 攻撃側の効果からダメージ補正を反映する
    for effect in attacker_data.effects.iter() {
        match effect {
            Effect::AttackDamageModifier(rate) => {
                // ダメージ率増加効果
                damage = (damage as f32 * rate.modifier) as u32;
            }
            _ => {
                // その他
            }
        }
    }
    //  防御側の効果からダメージ補正を反映する
    for effect in target_effects.iter() {
        match effect {
            Effect::ReceiveDamageModifier(rate) => {
                // 受けるダメージ率減少効果
                damage = (damage as f32 * rate.modifier) as u32;
            }
            _ => {
                // その他
            }
        }
    }
    let (before_hp_damage, after_hp_damage, is_dead) = target.hp.damage(damage);

    // HPダメージのインシデント
    incidents.push(BattleCharacterIncidentConcrete::DamageHp(
        BattleIncidentDamageHp::new(damage, before_hp_damage, after_hp_damage),
    ));
    if is_dead {
        // 死亡インシデント
        incidents.push(BattleCharacterIncidentConcrete::Death(
            BattleIncidentDeath {},
        ));
    }

    // ブレイクダメージ処理
    if target.character_type == BattleCharacterType::Enemy {
        // 敵の場合のみブレイクダメージを与える
        incidents.extend(super::accumulate_status_ailment(
            target,
            &StatusAilment::Breaking,
            break_power,
        ));
    }

    // AttackTarget 追加効果
    for additional_effect in &art_attack.additional_effects {
        if additional_effect.target == AdditionalEffectTarget::AttackTarget {
            match &additional_effect.content {
                ArtPotencySupport::None => {}
                ArtPotencySupport::StatusCondition(status_condition) => {
                    for sc in &status_condition.status_conditions {
                        let battle_sc = super::create_battle_status_condition(sc);
                        target.status_conditions.push(battle_sc.clone());
                        incidents.push(BattleCharacterIncidentConcrete::StatusConditionApplied(
                            BattleIncidentStatusConditionApplied {
                                status_condition: battle_sc,
                            },
                        ));
                    }
                }
                ArtPotencySupport::Recover(recover) => {
                    for potency in &recover.potencies {
                        match potency {
                            SupportRecoverPotency::Hp(hp) => {
                                let (before, after) = target.hp.recover(hp.hp_recover);
                                incidents.push(BattleCharacterIncidentConcrete::RecoverHp(
                                    BattleIncidentRecoverHp::new(hp.hp_recover, before, after),
                                ));
                            }
                            SupportRecoverPotency::Sp(sp) => {
                                let (before, after) = target.sp.recover(sp.sp_recover);
                                incidents.push(BattleCharacterIncidentConcrete::RecoverSp(
                                    BattleIncidentRecoverSp::new(sp.sp_recover, before, after),
                                ));
                            }
                            SupportRecoverPotency::Stamina(sta) => {
                                if target.character_type == BattleCharacterType::Player {
                                    let (before, after) =
                                        target.stamina.recover(sta.stamina_recover);
                                    incidents.push(
                                        BattleCharacterIncidentConcrete::RecoverStamina(
                                            BattleIncidentRecoverStamina::new(
                                                sta.stamina_recover,
                                                before,
                                                after,
                                            ),
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }
                ArtPotencySupport::StatusAilment(status_ailment) => {
                    incidents.extend(super::accumulate_status_ailment(
                        target,
                        &status_ailment.kind,
                        status_ailment.accumulation,
                    ));
                }
                ArtPotencySupport::AddKarmaToDeck(add_karma) => {
                    if let Some(karma) = target.karma.as_mut() {
                        for _ in 0..add_karma.count {
                            karma.draw_pile.push(KarmaDeckCard { card_id: add_karma.karma_card_id });
                        }
                        incidents.push(BattleCharacterIncidentConcrete::KarmaAddedToDeck(
                            BattleIncidentKarmaAddedToDeck {
                                karma_card_id: add_karma.karma_card_id, // KarmaCardId 型
                                count: add_karma.count,
                            },
                        ));
                    }
                }
            }
        }
    }

    incidents
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
