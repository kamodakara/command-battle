use crate::types::{Armor, ArmorSlot, Equipment};

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

// 武器使用可能かチェックする関数
pub fn is_weapon_usable(
    weapon: &crate::types::Weapon,
    player_ability: &crate::types::PlayerAbility,
) -> bool {
    if player_ability.strength < weapon.ability_requirement.strength {
        return false;
    }
    if player_ability.dexterity < weapon.ability_requirement.dexterity {
        return false;
    }
    if player_ability.intelligence < weapon.ability_requirement.intelligence {
        return false;
    }
    if player_ability.faith < weapon.ability_requirement.faith {
        return false;
    }
    if player_ability.arcane < weapon.ability_requirement.arcane {
        return false;
    }
    if player_ability.agility < weapon.ability_requirement.agility {
        return false;
    }
    true
}
