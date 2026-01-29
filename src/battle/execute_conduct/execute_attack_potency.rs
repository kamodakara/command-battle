use super::*;

pub fn execute_attack_potency(
    art_attack: &ArtPotencyAttack,
    attacker_data: &AttackerData,
    target_data: TargetData,
) -> Vec<BattleCharacterIncidentConcrete> {
    let target = target_data.target;
    let target_effects = target_data.effects;

    let mut incidents = vec![];

    // 武器性能取得
    let weapon_attack_power = attacker_data.weapon_performance.final_attack_power();
    let weapon_break_power = attacker_data.weapon_performance.final_break_power();

    // アーツ攻撃力算出
    let mut attack_power = art_attack.final_attack_power(&weapon_attack_power);

    // 術力の攻撃力補正
    let sorcery_power = attacker_data.weapon_performance.sorcery_power;
    if attacker_data.conduct.art.art_type == ArtType::Sorcery {
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
                attack_power = resistance.cut_rate.apply_guard_cut(&attack_power);
                is_defended = true;
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

    // ブレイク力算出
    let break_power = art_attack.final_break_power(weapon_break_power);

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
    let (before_hp_damage, after_hp_damage) = target.hp.damage(damage);

    // HPダメージのインシデント
    incidents.push(BattleCharacterIncidentConcrete::DamageHp(
        BattleIncidentDamageHp::new(damage, before_hp_damage, after_hp_damage),
    ));

    // 防御時のスタミナダメージ
    if is_defended {
        let sta_damage = break_power / 4; // TODO: 固定値ではなくガード強度
        let (before_sta, after_sta) = target.stamina.damage(sta_damage);

        // スタミナダメージのインシデント
        incidents.push(BattleCharacterIncidentConcrete::DamageStamina(
            BattleIncidentDamageStamina::new(sta_damage, before_sta, after_sta),
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
                // TODO:
                // ブレイク状態にする
                // support_status_effect(
                //     &vec![StatusCondition {
                //         potency: StatusConditionPotency::Break(StatusConditionBreak {}),
                //         duration: StatusConditionDuration::Permanent,
                //     }],
                //     target,
                //     &mut target_character_incident,
                // );
            }

            // ブレイクダメージインシデント追加
            incidents.push(BattleCharacterIncidentConcrete::DamageBreak(
                BattleIncidentDamageBreak::new(break_power, before_break, after_break),
            ));
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
