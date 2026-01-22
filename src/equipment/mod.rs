use crate::types::{
    Ability, AbilityType, Armor, ArmorSlot, AttackPower, Equipment, Weapon, WeaponAttackPower,
    WeaponPerformance, WeaponPerformancePenalty,
};

// 装備可能かチェックする関数
pub fn is_armor_equippable(armor: &Armor, equipment: Equipment) -> bool {
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
