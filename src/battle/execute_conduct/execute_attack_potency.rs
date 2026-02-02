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
                    let sta_damage = break_power / performance.guard_strength.max(1);
                    let (before_sta, after_sta) = target.stamina.damage(sta_damage);

                    // ガード成功時のコンビネーションログ
                    if let Some(combination_skill) = &mut target.combination_skill {
                        combination_skill
                            .add_current_conduct_result(CombinationConductResult::GuardSuccess);
                    }

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
    incidents.extend(accumulate_status_ailment(
        target,
        &StatusAilment::Breaking,
        break_power,
    ));

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

// 状態異常蓄積
fn accumulate_status_ailment(
    target: &mut BattleCharacter,
    ailment: &StatusAilment,
    ailment_accumulation: u32, // 蓄積量
) -> Vec<BattleCharacterIncidentConcrete> {
    let mut incidents = vec![];

    if ailment == &StatusAilment::Breaking && target.status_ailment.breaking.is_ailment {
        // ブレイク状態の場合は蓄積しない
        return incidents;
    }

    let status = match ailment {
        StatusAilment::Poison => &mut target.status_ailment.poison,
        StatusAilment::Sleep => &mut target.status_ailment.sleep,
        StatusAilment::Chill => &mut target.status_ailment.chill,
        StatusAilment::Bleed => &mut target.status_ailment.bleed,
        StatusAilment::Burn => &mut target.status_ailment.burn,
        StatusAilment::Paralysis => &mut target.status_ailment.paralysis,
        StatusAilment::Fear => &mut target.status_ailment.fear,
        StatusAilment::Rage => &mut target.status_ailment.rage,
        StatusAilment::Breaking => &mut target.status_ailment.breaking,
    };

    let (before, after) = status.add_accumulation(ailment_accumulation);
    // 蓄積インシデント
    incidents.push(BattleCharacterIncidentConcrete::StatusAilmentAccumulation(
        BattleIncidentStatusAilmentAccumulation {
            status_ailment: ailment.clone(),
            accumulation: ailment_accumulation,
            before_accumulation: before,
            after_accumulation: after,
        },
    ));
    // 蓄積ターンリセット
    status.no_accumulation_turns = 0;

    if !status.is_ailment && after == status.max_accumulation {
        // 状態異常でない場合、蓄積
        status.is_ailment = true;

        // 効果発動
        let effects = ailment.on_ailment_effects();
        for effect in effects.iter() {
            match effect {
                BattleStatusAilmentOnAilmentEffect::HpPercentageDamage(effect) => {
                    // HP割合ダメージ
                    let damage = (target.hp.max_hp as f32 * effect.percentage) as u32;
                    let (before_hp, after_hp, is_dead) = target.hp.damage(damage);

                    // HPダメージインシデント
                    incidents.push(BattleCharacterIncidentConcrete::DamageHp(
                        BattleIncidentDamageHp::new(damage, before_hp, after_hp),
                    ));
                    if is_dead {
                        // 死亡インシデント
                        incidents.push(BattleCharacterIncidentConcrete::Death(
                            BattleIncidentDeath {},
                        ));
                    }
                }
                BattleStatusAilmentOnAilmentEffect::SpPercentageDamage(effect) => {
                    // SP割合ダメージ
                    let damage = (target.sp.max_sp as f32 * effect.percentage) as u32;
                    let (before_sp, after_sp) = target.sp.damage(damage);

                    // SPダメージインシデント
                    incidents.push(BattleCharacterIncidentConcrete::DamageSp(
                        BattleIncidentDamageSp::new(damage, before_sp, after_sp),
                    ));
                }
                _ => {
                    // その他
                }
            }
        }

        // 状態異常付与インシデント
        incidents.push(BattleCharacterIncidentConcrete::StatusAilmentApplied(
            BattleIncidentStatusAilmentApplied {
                status_ailment: ailment.clone(),
            },
        ));
    }

    incidents
}
