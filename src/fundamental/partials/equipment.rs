use super::*;

// 装備可能かチェックする関数
pub fn is_armor_equippable(armor: &Armor, equipment: &Equipment) -> bool {
    let mut equipment_slots: Vec<&ArmorSlot> = vec![];
    if let Some(a) = &equipment.armor1 {
        equipment_slots.extend(&a.slots);
    }
    if let Some(a) = &equipment.armor2 {
        equipment_slots.extend(&a.slots);
    }
    if let Some(a) = &equipment.armor3 {
        equipment_slots.extend(&a.slots);
    }
    if let Some(a) = &equipment.armor4 {
        equipment_slots.extend(&a.slots);
    }
    if let Some(a) = &equipment.armor5 {
        equipment_slots.extend(&a.slots);
    }
    if let Some(a) = &equipment.armor6 {
        equipment_slots.extend(&a.slots);
    }
    if let Some(a) = &equipment.armor7 {
        equipment_slots.extend(&a.slots);
    }
    if let Some(a) = &equipment.armor8 {
        equipment_slots.extend(&a.slots);
    }

    for slot in &armor.slots {
        if equipment_slots.contains(&slot) {
            // 装備中装備にすでに存在する装備箇所がある場合、装備不可
            return false;
        }
    }
    // 装備可能
    true
}

impl Equipment {
    pub fn is_equippable(&self, armor: &Armor) -> bool {
        is_armor_equippable(armor, self)
    }

    // 装備総重量
    pub fn total_weight(&self) -> u32 {
        let mut weight = 0;
        if let Some(armor) = &self.armor1 {
            weight += armor.weight;
        }
        if let Some(armor) = &self.armor2 {
            weight += armor.weight;
        }
        if let Some(armor) = &self.armor3 {
            weight += armor.weight;
        }
        if let Some(armor) = &self.armor4 {
            weight += armor.weight;
        }
        if let Some(armor) = &self.armor5 {
            weight += armor.weight;
        }
        if let Some(armor) = &self.armor6 {
            weight += armor.weight;
        }
        if let Some(armor) = &self.armor7 {
            weight += armor.weight;
        }
        if let Some(armor) = &self.armor8 {
            weight += armor.weight;
        }
        weight
    }

    pub fn load_performance(&self, max_equipment_weight: u32) -> EquipmentLoadPerformance {
        let total_weight = self.total_weight();
        let status = if total_weight <= max_equipment_weight {
            EquipmentLoadPerformanceStatus::Light
        } else if total_weight <= max_equipment_weight * 2 {
            EquipmentLoadPerformanceStatus::Medium
        } else if total_weight <= max_equipment_weight * 3 {
            EquipmentLoadPerformanceStatus::Heavy
        } else {
            EquipmentLoadPerformanceStatus::SuperHeavy
        };
        let agility_multiplier = match status {
            EquipmentLoadPerformanceStatus::Light => 1.5,
            EquipmentLoadPerformanceStatus::Medium => 1.0,
            EquipmentLoadPerformanceStatus::Heavy => 0.5,
            EquipmentLoadPerformanceStatus::SuperHeavy => 0.1,
        };
        let stamina_recovery_multiplier = match status {
            EquipmentLoadPerformanceStatus::Light => 1.5,
            EquipmentLoadPerformanceStatus::Medium => 1.0,
            EquipmentLoadPerformanceStatus::Heavy => 0.5,
            EquipmentLoadPerformanceStatus::SuperHeavy => 0.1,
        };
        EquipmentLoadPerformance {
            max_equipment_weight,
            total_weight,
            status,
            agility_multiplier,
            stamina_recovery_multiplier,
        }
    }
}

impl Weapon {
    // 不足している能力一覧を取得する
    pub fn not_enough_abilities(&self, ability: &Ability) -> Vec<AbilityType> {
        let mut not_enough = vec![];
        if ability.strength < self.ability_requirement.strength {
            not_enough.push(AbilityType::Strength);
        }
        if ability.dexterity < self.ability_requirement.dexterity {
            not_enough.push(AbilityType::Dexterity);
        }
        if ability.intelligence < self.ability_requirement.intelligence {
            not_enough.push(AbilityType::Intelligence);
        }
        if ability.faith < self.ability_requirement.faith {
            not_enough.push(AbilityType::Faith);
        }
        if ability.arcane < self.ability_requirement.arcane {
            not_enough.push(AbilityType::Arcane);
        }
        if ability.agility < self.ability_requirement.agility {
            not_enough.push(AbilityType::Agility);
        }
        not_enough
    }

    // 武器の最終的な性能を取得する
    // 武器性能、能力、装備から計算する
    // 必要能力が足りていない場合はペナルティを与える
    // ペナルティがかかった事を返す
    pub fn performance(&self, ability: &Ability) -> WeaponPerformance {
        let not_enough_abilities = self.not_enough_abilities(&ability);
        if not_enough_abilities.is_empty() {
            let base_attack_power = self.attack_power.base.clone();
            let ability_attack_power = self.attack_power.ability_attack_power(&ability);
            let base_sorcery_power = self.sorcery_power.base;
            let ability_sorcery_power = self.sorcery_power.scaling.scale_value(ability);
            let base_break_power = self.break_power.base_power;
            let ability_break_power = self.break_power.scaling.scale_value(ability);

            // 必要能力を満たしている場合、通常の性能を返す
            WeaponPerformance {
                attack_power: base_attack_power,
                ability_attack_power,
                sorcery_power: base_sorcery_power,
                ability_sorcery_power,
                break_power: base_break_power,
                ability_break_power,
                guard_strength: self.guard.guard_strength,
                penalty: None,
            }
        } else {
            // 必要能力を満たしていない場合、ペナルティを与えた性能を返す
            let base_attack_power = self.attack_power.base.clone();
            let ability_attack_power = AttackPower::default();
            let base_sorcery_power = self.sorcery_power.base;
            let ability_sorcery_power = 0;
            let base_break_power = self.break_power.base_power;
            let ability_break_power = 0;

            // ペナルティ割合
            let penalty_rate = 0.3;
            let mut penalty_attack_power = base_attack_power.clone();
            penalty_attack_power.multiply(penalty_rate);
            let penalty_sorcery_power = (base_sorcery_power as f32 * penalty_rate) as u32;
            let penalty_break_power = (base_break_power as f32 * penalty_rate) as u32;
            let penalty_guard_strength = (self.guard.guard_strength as f32 * penalty_rate) as u32;

            WeaponPerformance {
                attack_power: base_attack_power,
                ability_attack_power,
                sorcery_power: base_sorcery_power,
                ability_sorcery_power,
                break_power: base_break_power,
                ability_break_power,
                guard_strength: self.guard.guard_strength,
                penalty: Some(WeaponPerformancePenalty {
                    not_enough_abilities,
                    penalty_attack_power,
                    penalty_sorcery_power,
                    penalty_break_power,
                    penalty_guard_strength,
                }),
            }
        }
    }
}

impl WeaponAttackPower {
    pub fn ability_attack_power(&self, ability: &Ability) -> AttackPower {
        AttackPower {
            slash: self.ability_scaling.slash.scale_value(ability),
            strike: self.ability_scaling.strike.scale_value(ability),
            thrust: self.ability_scaling.thrust.scale_value(ability),
            impact: self.ability_scaling.impact.scale_value(ability),
            magic: self.ability_scaling.magic.scale_value(ability),
            fire: self.ability_scaling.fire.scale_value(ability),
            lightning: self.ability_scaling.lightning.scale_value(ability),
            chaos: self.ability_scaling.chaos.scale_value(ability),
        }
    }
}

impl WeaponPerformance {
    // 素手の攻撃性能取得
    pub fn unarmed_weapon_performance() -> WeaponPerformance {
        // TODO: 仮
        WeaponPerformance {
            attack_power: AttackPower {
                slash: 0,
                strike: 10,
                thrust: 0,
                impact: 0,
                magic: 0,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            ability_attack_power: AttackPower::default(),
            sorcery_power: 0,
            ability_sorcery_power: 0,
            break_power: 0,
            ability_break_power: 0,
            guard_strength: 10,
            penalty: None,
        }
    }

    // 最終的な攻撃性能を取得する
    pub fn final_attack_power(&self) -> AttackPower {
        let mut attack_power = self.attack_power.clone();

        // 能力補正分を加算
        attack_power.slash += self.ability_attack_power.slash;
        attack_power.strike += self.ability_attack_power.strike;
        attack_power.thrust += self.ability_attack_power.thrust;
        attack_power.impact += self.ability_attack_power.impact;
        attack_power.magic += self.ability_attack_power.magic;
        attack_power.fire += self.ability_attack_power.fire;
        attack_power.lightning += self.ability_attack_power.lightning;
        attack_power.chaos += self.ability_attack_power.chaos;

        // ペナルティ分を減算
        if let Some(penalty) = &self.penalty {
            attack_power.slash = attack_power
                .slash
                .saturating_sub(penalty.penalty_attack_power.slash);
            attack_power.strike = attack_power
                .strike
                .saturating_sub(penalty.penalty_attack_power.strike);
            attack_power.thrust = attack_power
                .thrust
                .saturating_sub(penalty.penalty_attack_power.thrust);
            attack_power.impact = attack_power
                .impact
                .saturating_sub(penalty.penalty_attack_power.impact);
            attack_power.magic = attack_power
                .magic
                .saturating_sub(penalty.penalty_attack_power.magic);
            attack_power.fire = attack_power
                .fire
                .saturating_sub(penalty.penalty_attack_power.fire);
            attack_power.lightning = attack_power
                .lightning
                .saturating_sub(penalty.penalty_attack_power.lightning);
            attack_power.chaos = attack_power
                .chaos
                .saturating_sub(penalty.penalty_attack_power.chaos);
        }

        attack_power
    }

    pub fn final_sorcery_power(&self) -> u32 {
        let mut sorcery_power = self.sorcery_power;

        // 能力補正分を加算
        sorcery_power += self.ability_sorcery_power;

        // ペナルティ分を減算
        if let Some(penalty) = &self.penalty {
            sorcery_power = sorcery_power.saturating_sub(penalty.penalty_sorcery_power);
        }

        sorcery_power
    }

    pub fn final_break_power(&self) -> u32 {
        let mut break_power = self.break_power;

        // 能力補正分を加算
        break_power += self.ability_break_power;

        // ペナルティ分を減算
        if let Some(penalty) = &self.penalty {
            break_power = break_power.saturating_sub(penalty.penalty_break_power);
        }

        break_power
    }

    pub fn final_guard_strength(&self) -> u32 {
        let mut guard_strength = self.guard_strength;

        // ペナルティ分を減算
        if let Some(penalty) = &self.penalty {
            guard_strength = guard_strength.saturating_sub(penalty.penalty_guard_strength);
        }

        guard_strength
    }
}
