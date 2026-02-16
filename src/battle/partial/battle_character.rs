use super::*;

impl BattleCharacter {
    // 効果適応済み能力
    pub fn ability_with_effects(&self, effects: &Vec<Effect>) -> Ability {
        let mut ability = self.raw_ability.clone();

        for effect in effects {
            match effect {
                Effect::AbilityIncrease(e) => match e.ability_type {
                    AbilityType::Strength => ability.strength += e.amount,
                    AbilityType::Dexterity => ability.dexterity += e.amount,
                    AbilityType::Intelligence => ability.intelligence += e.amount,
                    AbilityType::Faith => ability.faith += e.amount,
                    AbilityType::Vitality => ability.vitality += e.amount,
                    AbilityType::Spirit => ability.spirit += e.amount,
                    AbilityType::Endurance => ability.endurance += e.amount,
                    AbilityType::Agility => ability.agility += e.amount,
                    AbilityType::Arcane => ability.arcane += e.amount,
                },
                _ => { /* 無視 */ }
            }
        }

        for effect in effects {
            match effect {
                Effect::AbilityModifier(e) => match e.ability_type {
                    AbilityType::Strength => {
                        ability.strength = (ability.strength as f32 * e.modifier) as u32
                    }
                    AbilityType::Dexterity => {
                        ability.dexterity = (ability.dexterity as f32 * e.modifier) as u32
                    }
                    AbilityType::Intelligence => {
                        ability.intelligence = (ability.intelligence as f32 * e.modifier) as u32
                    }
                    AbilityType::Faith => {
                        ability.faith = (ability.faith as f32 * e.modifier) as u32
                    }
                    AbilityType::Vitality => {
                        ability.vitality = (ability.vitality as f32 * e.modifier) as u32
                    }
                    AbilityType::Spirit => {
                        ability.spirit = (ability.spirit as f32 * e.modifier) as u32
                    }
                    AbilityType::Endurance => {
                        ability.endurance = (ability.endurance as f32 * e.modifier) as u32
                    }
                    AbilityType::Agility => {
                        ability.agility = (ability.agility as f32 * e.modifier) as u32
                    }
                    AbilityType::Arcane => {
                        ability.arcane = (ability.arcane as f32 * e.modifier) as u32
                    }
                },
                _ => { /* 無視 */ }
            }
        }

        ability
    }

    pub fn armor_defense_power(&self) -> DefensePower {
        self.raw_equipment.armor_defense_power()
    }

    // 効果適応済み防御力取得
    pub fn defense_power_with_effects(&self, effects: &Vec<Effect>) -> DefensePower {
        // 能力の防御力
        let ability = self.ability_with_effects(effects);
        let mut defense_power = ability.base_defense_power();

        // 装備の防御力を加算
        defense_power.add(&self.armor_defense_power());

        defense_power
    }

    pub fn weapon(&self, weapon_id: &BattleWeaponId) -> Option<&BattleWeapon> {
        self.weapons.iter().find(|w| &w.id == weapon_id)
    }

    // 最大カルマ値を取得する
    // TODO: 能力変化の影響を受けないので値で持っていてもいいかも
    pub fn max_karma(&self) -> u32 {
        // 最大カルマ値は能力変化の影響を受けない
        let ability = &self.raw_ability;
        (ability.vitality as f32
            + ability.spirit as f32
            + (ability.intelligence as f32 * 1.5)
            + (ability.faith as f32 * 1.5)) as u32
    }

    // キャラクター関係の現在有効な効果取得
    // 状態異常、カルマ、トランスなどで発動している効果をすべて取得する
    // 効果打消し等あればここで処理する
    // 最終的に有効な効果一覧を返す
    pub fn current_effects(&self) -> Vec<Effect> {
        let mut effects = vec![];

        // 装備重量
        if self.character_type == BattleCharacterType::Player {
            // TODO: 現状、毎回算出する必要はない
            let equipment_load_performance = self
                .raw_equipment
                .load_performance(self.max_equipment_weight);

            // 装備重量による敏捷性補正
            if equipment_load_performance.agility_multiplier != 1.0 {
                effects.push(Effect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Agility,
                    modifier: equipment_load_performance.agility_multiplier,
                }));
            }
            // 装備重量によるスタミナ回復量補正
            if equipment_load_performance.stamina_recovery_multiplier != 1.0 {
                effects.push(Effect::StaminaRecoveryModifier(
                    EffectStaminaRecoveryModifier {
                        modifier: equipment_load_performance.stamina_recovery_multiplier,
                    },
                ));
            }
        }

        // 状態異常の継続効果
        let status_ailment_ongoing_effects = self.status_ailment.current_ongoing_effects();
        for ongoing_effect in status_ailment_ongoing_effects.into_iter() {
            match ongoing_effect {
                BattleStatusAilmentOngoingEffect::HpPercentageDamage(e) => {
                    effects.push(Effect::HpPercentageDamage(e))
                }
                BattleStatusAilmentOngoingEffect::SpPercentageDamage(e) => {
                    effects.push(Effect::SpPercentageDamage(e))
                }
                BattleStatusAilmentOngoingEffect::AttackDamageModifier(e) => {
                    effects.push(Effect::AttackDamageModifier(e))
                }
                BattleStatusAilmentOngoingEffect::ReceiveDamageModifier(e) => {
                    effects.push(Effect::ReceiveDamageModifier(e))
                }
                BattleStatusAilmentOngoingEffect::AbilityModifier(e) => {
                    effects.push(Effect::AbilityModifier(e))
                }
                BattleStatusAilmentOngoingEffect::RemoveStatusAilment(e) => {
                    effects.push(Effect::RemoveStatusAilment(e))
                }
                BattleStatusAilmentOngoingEffect::UnableToAct => {
                    effects.push(Effect::UnableToAct)
                }
            }
        }

        // カルマの場効果
        if let Some(karma) = &self.karma {
            let karma_field_effects = karma.field_effects();

            for karma_effect in karma_field_effects.into_iter() {
                match karma_effect {
                    KarmaEffect::AttackDamageModifier(e) => {
                        effects.push(Effect::AttackDamageModifier(e))
                    }
                    KarmaEffect::ReceiveDamageModifier(e) => {
                        effects.push(Effect::ReceiveDamageModifier(e))
                    }
                    KarmaEffect::AbilityIncrease(e) => effects.push(Effect::AbilityIncrease(e)),
                }
            }
        }

        // トランスハート効果
        if let Some(trance) = &self.trance {
            let heart_effects = trance.current_heart_effects();

            for heart_effect in heart_effects.into_iter() {
                match heart_effect {
                    HeartEffect::PhysicalDefenseModifier(e) => {
                        effects.push(Effect::PhysicalDefenseModifier(e))
                    }
                    HeartEffect::MagicalDefenseModifier(e) => {
                        effects.push(Effect::MagicalDefenseModifier(e))
                    }
                    HeartEffect::PhysicalAttackModifier(e) => {
                        effects.push(Effect::PhysicalAttackModifier(e))
                    }
                    HeartEffect::MagicalAttackModifier(e) => {
                        effects.push(Effect::MagicalAttackModifier(e))
                    }
                    HeartEffect::StaminaRecoveryModifier(e) => {
                        effects.push(Effect::StaminaRecoveryModifier(e))
                    }
                    HeartEffect::AbilityIncrease(e) => effects.push(Effect::AbilityIncrease(e)),
                }
            }
        }

        effects
    }
}

impl BattleCharacterHP {
    // ダメージ処理
    // ダメージを与えた後のHP値と、死亡状態を返す
    pub fn damage(&mut self, amount: u32) -> (u32, u32, bool) {
        let before = self.current_hp;
        self.current_hp = self.current_hp.saturating_sub(amount);
        let after = self.current_hp;
        self.is_dead = self.current_hp == 0;
        (before, after, self.is_dead)
    }
    pub fn recover(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_hp;
        self.current_hp = std::cmp::min(self.current_hp + amount, self.max_hp);
        let after = self.current_hp;
        (before, after)
    }
}

impl BattleCharacterSP {
    pub fn damage(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_sp;
        self.current_sp = self.current_sp.saturating_sub(amount);
        let after = self.current_sp;
        (before, after)
    }
    pub fn recover(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_sp;
        self.current_sp = std::cmp::min(self.current_sp + amount, self.max_sp);
        let after = self.current_sp;
        (before, after)
    }
}

impl BattleCharacterStamina {
    pub fn damage(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_stamina;
        self.current_stamina = self.current_stamina.saturating_sub(amount);
        let after = self.current_stamina;
        (before, after)
    }
    pub fn recover(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_stamina;
        self.current_stamina = std::cmp::min(self.current_stamina + amount, self.max_stamina);
        let after = self.current_stamina;
        (before, after)
    }
}
