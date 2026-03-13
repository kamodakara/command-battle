use super::resources::{EquippedWeaponWithArts, PlayerEquippedWeapons};
use crate::fundamental::*;
use crate::game::{ArtsDatabase, EquipmentDatabase, PreparationState};
use std::sync::Arc;

pub fn player_standard_arts() -> Vec<Arc<Art>> {
    vec![
        // 待機
        Arc::new(Art {
            name: "待機".to_string(),
            sp_cost: 0,
            stamina_cost: 0,
            perks: vec![],
            requirement: ArtRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            art_type: ArtType::Basic,
            usable_weapon: ArtUsableWeapon::All,
            always_hits: false,
            priority: 0,
            rank1: ArtRank {
                threshold: 0,
                target: ArtTarget::Single,
                potency: ArtPotency::Support(ArtPotencySupport::Recover(
                    ArtPotencySupportRecover {
                        potencies: vec![SupportRecoverPotency::Stamina(
                            SupportRecoverPotencyStamina {
                                stamina_recover: 60,
                            },
                        )],
                    },
                )),
            },
            rank2: None,
            rank3: None,
        }),
        // 回復
        Arc::new(Art {
            name: "回復".to_string(),
            sp_cost: 0,
            stamina_cost: 10,
            perks: vec![],
            requirement: ArtRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            art_type: ArtType::Basic,
            usable_weapon: ArtUsableWeapon::All,
            always_hits: true,
            priority: 0,
            rank1: ArtRank {
                threshold: 0,
                target: ArtTarget::Single,
                potency: ArtPotency::Support(ArtPotencySupport::Recover(
                    ArtPotencySupportRecover {
                        potencies: vec![SupportRecoverPotency::Hp(SupportRecoverPotencyHp {
                            hp_recover: 50,
                        })],
                    },
                )),
            },
            rank2: None,
            rank3: None,
        }),
        // 防御
        Arc::new(Art {
            name: "防御".to_string(),
            sp_cost: 0,
            stamina_cost: 5,
            perks: vec![ArtPerk::Guard],
            requirement: ArtRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            art_type: ArtType::Skill,
            usable_weapon: ArtUsableWeapon::All,
            always_hits: false,
            priority: 2,
            rank1: ArtRank {
                threshold: 0,
                target: ArtTarget::Single,
                potency: ArtPotency::Support(ArtPotencySupport::StatusCondition(
                    ArtPotencySupportStatusCondition {
                        status_conditions: vec![StatusCondition {
                            potency: StatusConditionPotency::Resistance(
                                StatusConditionResistance {
                                    battle_weapon_id: BattleWeaponId(0),
                                },
                            ),
                            duration: StatusConditionDuration::Turn(StatusConditionDurationTurn {
                                turns: 1,
                            }),
                        }],
                    },
                )),
            },
            rank2: None,
            rank3: None,
        }),
        // 通常攻撃
        Arc::new(Art {
            name: "攻撃".to_string(),
            sp_cost: 0,
            stamina_cost: 5,
            perks: vec![ArtPerk::Melee],
            requirement: ArtRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            art_type: ArtType::Skill,
            usable_weapon: ArtUsableWeapon::All,
            always_hits: false,
            priority: 0,
            rank1: ArtRank {
                threshold: 0,
                target: ArtTarget::Single,
                potency: ArtPotency::Attack(ArtPotencyAttack {
                    attack_power: AttackPower {
                        slash: 0,
                        strike: 0,
                        thrust: 0,
                        impact: 0,
                        magic: 0,
                        fire: 0,
                        lightning: 0,
                        chaos: 0,
                    },
                    weapon_attack_power_scaling: AttackPowerScaling {
                        slash: 1.0,
                        strike: 1.0,
                        thrust: 1.0,
                        impact: 1.0,
                        magic: 1.0,
                        fire: 1.0,
                        lightning: 1.0,
                        chaos: 1.0,
                    },
                    break_power: 10,
                    weapon_break_power_scaling: 0.0,
                }),
            },
            rank2: None,
            rank3: None,
        }),
    ]
}

/// 準備画面の設定から装備武器とアーツを作成する
pub fn create_equipped_weapons_from_preparation(
    prep_state: &PreparationState,
    equipment_db: &EquipmentDatabase,
    arts_db: &ArtsDatabase,
) -> (Vec<Arc<Art>>, PlayerEquippedWeapons, Vec<BattleWeapon>) {
    // 選択された技術を取得
    let mut selected_arts: Vec<Arc<Art>> = prep_state
        .selected_arts
        .iter()
        .filter_map(|&id| arts_db.arts.iter().find(|a| a.id == id))
        .map(|a| Arc::new(a.art.clone()))
        .collect();
    // プレイヤー標準アーツを追加
    selected_arts.extend(player_standard_arts());

    // 基本アーツ（待機など）を取得
    let basic_arts: Vec<Arc<Art>> = selected_arts
        .iter()
        .filter(|art| art.art_type == ArtType::Basic || art.art_type == ArtType::Sorcery)
        .cloned()
        .collect();

    // 装備武器を取得
    let mut weapons: Vec<EquippedWeaponWithArts> = Vec::new();
    let mut battle_weapons: Vec<BattleWeapon> = Vec::new();

    // 武器1
    if let Some(weapon1_id) = prep_state.equipped_weapon1 {
        if let Some(weapon_data) = equipment_db.weapons.iter().find(|w| w.id == weapon1_id) {
            let battle_weapon_id = BattleWeaponId(0);
            let skills: Vec<Arc<Art>> = selected_arts
                .iter()
                .filter(|art| {
                    art.art_type == ArtType::Skill
                        && (art.usable_weapon == ArtUsableWeapon::All
                            || matches!(
                                &art.usable_weapon,
                                ArtUsableWeapon::Specific(kinds) if kinds.contains(&weapon_data.weapon.kind)
                            ))
                })
                .cloned()
                .collect();
            let sorceries: Vec<Arc<Art>> = selected_arts
                .iter()
                .filter(|art| art.art_type == ArtType::Sorcery)
                .cloned()
                .collect();
            battle_weapons.push(BattleWeapon {
                id: BattleWeaponId(0),
                weapon: weapon_data.weapon.clone(),
            });
            weapons.push(EquippedWeaponWithArts {
                weapon: weapon_data.weapon.clone(),
                skills,
                sorceries,
                battle_weapon_id,
            });
        }
    }

    // 武器2
    if let Some(weapon2_id) = prep_state.equipped_weapon2 {
        if let Some(weapon_data) = equipment_db.weapons.iter().find(|w| w.id == weapon2_id) {
            let battle_weapon_id = BattleWeaponId(1);
            let skills: Vec<Arc<Art>> = selected_arts
                .iter()
                .filter(|art| {
                    art.art_type == ArtType::Skill
                        && (art.usable_weapon == ArtUsableWeapon::All
                            || matches!(
                                &art.usable_weapon,
                                ArtUsableWeapon::Specific(kinds) if kinds.contains(&weapon_data.weapon.kind)
                            ))
                })
                .cloned()
                .collect();
            let sorceries: Vec<Arc<Art>> = selected_arts
                .iter()
                .filter(|art| art.art_type == ArtType::Sorcery)
                .cloned()
                .collect();
            battle_weapons.push(BattleWeapon {
                id: BattleWeaponId(1),
                weapon: weapon_data.weapon.clone(),
            });
            weapons.push(EquippedWeaponWithArts {
                weapon: weapon_data.weapon.clone(),
                skills,
                sorceries,
                battle_weapon_id,
            });
        }
    }

    (
        basic_arts,
        PlayerEquippedWeapons { weapons },
        battle_weapons,
    )
}

/// 準備画面の設定からBattleデータを作成する
pub fn create_battle_from_preparation(
    prep_state: &PreparationState,
    equipment_db: &EquipmentDatabase,
    battle_weapons: Vec<BattleWeapon>,
) -> Battle {
    // 共通防御力（0除算防止のため全て1）
    let def = DefensePower {
        slash: 1,
        strike: 1,
        thrust: 1,
        impact: 1,
        magic: 1,
        fire: 1,
        lightning: 1,
        chaos: 1,
    };

    // 準備画面のステータス設定を使用
    let ability = Ability {
        vitality: prep_state.temp_vitality,
        spirit: prep_state.temp_spirit,
        endurance: prep_state.temp_endurance,
        agility: prep_state.temp_agility,
        strength: prep_state.temp_strength,
        dexterity: prep_state.temp_dexterity,
        intelligence: prep_state.temp_intelligence,
        faith: prep_state.temp_faith,
        arcane: prep_state.temp_arcane,
    };

    // プレイヤーステータスを算出する
    let stats = ability.player_stats();

    // 装備を設定
    let mut equipment = Equipment {
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
    };

    // 武器を設定
    if let Some(weapon1_id) = prep_state.equipped_weapon1 {
        if let Some(weapon_data) = equipment_db.weapons.iter().find(|w| w.id == weapon1_id) {
            equipment.weapon1 = Some(weapon_data.weapon.clone());
        }
    }
    if let Some(weapon2_id) = prep_state.equipped_weapon2 {
        if let Some(weapon_data) = equipment_db.weapons.iter().find(|w| w.id == weapon2_id) {
            equipment.weapon2 = Some(weapon_data.weapon.clone());
        }
    }

    // 防具を設定
    if let Some(armor1_id) = prep_state.equipped_armor1 {
        if let Some(armor_data) = equipment_db.armors.iter().find(|a| a.id == armor1_id) {
            equipment.armor1 = Some(armor_data.armor.clone());
        }
    }
    if let Some(armor2_id) = prep_state.equipped_armor2 {
        if let Some(armor_data) = equipment_db.armors.iter().find(|a| a.id == armor2_id) {
            equipment.armor2 = Some(armor_data.armor.clone());
        }
    }
    if let Some(armor3_id) = prep_state.equipped_armor3 {
        if let Some(armor_data) = equipment_db.armors.iter().find(|a| a.id == armor3_id) {
            equipment.armor3 = Some(armor_data.armor.clone());
        }
    }
    if let Some(armor4_id) = prep_state.equipped_armor4 {
        if let Some(armor_data) = equipment_db.armors.iter().find(|a| a.id == armor4_id) {
            equipment.armor4 = Some(armor_data.armor.clone());
        }
    }
    if let Some(armor5_id) = prep_state.equipped_armor5 {
        if let Some(armor_data) = equipment_db.armors.iter().find(|a| a.id == armor5_id) {
            equipment.armor5 = Some(armor_data.armor.clone());
        }
    }
    if let Some(armor6_id) = prep_state.equipped_armor6 {
        if let Some(armor_data) = equipment_db.armors.iter().find(|a| a.id == armor6_id) {
            equipment.armor6 = Some(armor_data.armor.clone());
        }
    }
    if let Some(armor7_id) = prep_state.equipped_armor7 {
        if let Some(armor_data) = equipment_db.armors.iter().find(|a| a.id == armor7_id) {
            equipment.armor7 = Some(armor_data.armor.clone());
        }
    }
    if let Some(armor8_id) = prep_state.equipped_armor8 {
        if let Some(armor_data) = equipment_db.armors.iter().find(|a| a.id == armor8_id) {
            equipment.armor8 = Some(armor_data.armor.clone());
        }
    }

    // 防具から防御力を計算
    let mut total_defense = DefensePower {
        slash: 5,
        strike: 5,
        thrust: 5,
        impact: 5,
        magic: 5,
        fire: 5,
        lightning: 5,
        chaos: 5,
    };
    let armor_list = [
        &equipment.armor1,
        &equipment.armor2,
        &equipment.armor3,
        &equipment.armor4,
        &equipment.armor5,
        &equipment.armor6,
        &equipment.armor7,
        &equipment.armor8,
    ];
    for armor_opt in armor_list.iter() {
        if let Some(armor) = armor_opt {
            total_defense.slash += armor.defense.slash;
            total_defense.strike += armor.defense.strike;
            total_defense.thrust += armor.defense.thrust;
            total_defense.impact += armor.defense.impact;
            total_defense.magic += armor.defense.magic;
            total_defense.fire += armor.defense.fire;
            total_defense.lightning += armor.defense.lightning;
            total_defense.chaos += armor.defense.chaos;
        }
    }

    // 敵原本（仮）
    let enemy_original = Arc::new(Enemy {
        ability: Ability {
            vitality: 50,
            spirit: 50,
            endurance: 50,
            agility: 15,
            strength: 15,
            dexterity: 15,
            intelligence: 15,
            faith: 15,
            arcane: 15,
        },
        stats: EnemyStats { hp: 1000, sp: 30 },
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
    });

    Battle {
        player: BattleCharacter {
            character_id: 1,
            raw_ability: ability.clone(),
            raw_base_defense_power: total_defense.clone(),
            raw_equipment: equipment.clone(),
            character_type: BattleCharacterType::Player,
            hp: BattleCharacterHP {
                max_hp: stats.hp,
                current_hp: stats.hp,
                is_dead: false,
            },
            sp: BattleCharacterSP {
                max_sp: stats.sp,
                current_sp: stats.sp,
            },
            stamina: BattleCharacterStamina {
                max_stamina: stats.stamina,
                current_stamina: stats.stamina,
                stamina_recovery: stats.stamina_recovery,
            },
            max_equipment_weight: stats.max_equipment_weight,
            weapons: battle_weapons,
            status_conditions: vec![],
            status_ailment: BattleStatusAilment {
                poison: BattleStatusAilmentStatus::new_poison(),
                sleep: BattleStatusAilmentStatus::new_sleep(),
                chill: BattleStatusAilmentStatus::new_chill(),
                bleed: BattleStatusAilmentStatus::new_bleed(),
                burn: BattleStatusAilmentStatus::new_burn(),
                paralysis: BattleStatusAilmentStatus::new_paralysis(),
                fear: BattleStatusAilmentStatus::new_fear(),
                rage: BattleStatusAilmentStatus::new_rage(),
                breaking: BattleStatusAilmentStatus::new_breaking(0),
            },
            karma: Some(BattleKarma {
                draw_pile: vec![],
                discard_pile: vec![
                    KarmaCard {
                        name: "ラッキーパンチ".to_string(),
                        cost: 0,
                        max_turn: 1,
                        effects: vec![KarmaEffect::AttackDamageModifier(
                            EffectAttackDamageModifier { modifier: 1.5 },
                        )],
                    },
                    KarmaCard {
                        name: "好調".to_string(),
                        cost: 0,
                        max_turn: 3,
                        effects: vec![KarmaEffect::AttackDamageModifier(
                            EffectAttackDamageModifier { modifier: 1.05 },
                        )],
                    },
                    KarmaCard {
                        name: "好調".to_string(),
                        cost: 0,
                        max_turn: 3,
                        effects: vec![KarmaEffect::AttackDamageModifier(
                            EffectAttackDamageModifier { modifier: 1.05 },
                        )],
                    },
                    KarmaCard {
                        name: "好調".to_string(),
                        cost: 0,
                        max_turn: 3,
                        effects: vec![KarmaEffect::AttackDamageModifier(
                            EffectAttackDamageModifier { modifier: 1.05 },
                        )],
                    },
                    KarmaCard {
                        name: "堅実".to_string(),
                        cost: 0,
                        max_turn: 3,
                        effects: vec![KarmaEffect::ReceiveDamageModifier(
                            EffectReceiveDamageModifier { modifier: 0.95 },
                        )],
                    },
                    KarmaCard {
                        name: "堅実".to_string(),
                        cost: 0,
                        max_turn: 3,
                        effects: vec![KarmaEffect::ReceiveDamageModifier(
                            EffectReceiveDamageModifier { modifier: 0.95 },
                        )],
                    },
                    KarmaCard {
                        name: "堅実".to_string(),
                        cost: 0,
                        max_turn: 3,
                        effects: vec![KarmaEffect::ReceiveDamageModifier(
                            EffectReceiveDamageModifier { modifier: 0.95 },
                        )],
                    },
                    KarmaCard {
                        name: "堅実".to_string(),
                        cost: 0,
                        max_turn: 3,
                        effects: vec![KarmaEffect::ReceiveDamageModifier(
                            EffectReceiveDamageModifier { modifier: 0.95 },
                        )],
                    },
                    KarmaCard {
                        name: "追い風".to_string(),
                        cost: 0,
                        max_turn: 2,
                        effects: vec![KarmaEffect::AbilityIncrease(EffectAbilityIncrease {
                            ability_type: AbilityType::Agility,
                            amount: 1,
                        })],
                    },
                    KarmaCard {
                        name: "追い風".to_string(),
                        cost: 0,
                        max_turn: 2,
                        effects: vec![KarmaEffect::AbilityIncrease(EffectAbilityIncrease {
                            ability_type: AbilityType::Agility,
                            amount: 1,
                        })],
                    },
                    KarmaCard {
                        name: "追い風".to_string(),
                        cost: 0,
                        max_turn: 2,
                        effects: vec![KarmaEffect::AbilityIncrease(EffectAbilityIncrease {
                            ability_type: AbilityType::Agility,
                            amount: 1,
                        })],
                    },
                    KarmaCard {
                        name: "追い風".to_string(),
                        cost: 0,
                        max_turn: 2,
                        effects: vec![KarmaEffect::AbilityIncrease(EffectAbilityIncrease {
                            ability_type: AbilityType::Agility,
                            amount: 1,
                        })],
                    },
                ],
                field_cards: vec![],
            }),
            trance: Some(BattleTrance {
                max_trance: 1000,
                heart: Heart {
                    name: "聖騎士のハート".to_string(),
                    level1_effects: vec![
                        HeartEffect::AbilityIncrease(EffectAbilityIncrease {
                            ability_type: AbilityType::Strength,
                            amount: 5,
                        }),
                        HeartEffect::AbilityIncrease(EffectAbilityIncrease {
                            ability_type: AbilityType::Intelligence,
                            amount: 5,
                        }),
                        HeartEffect::AbilityIncrease(EffectAbilityIncrease {
                            ability_type: AbilityType::Agility,
                            amount: 5,
                        }),
                    ],
                    level2_effects: vec![HeartEffect::AbilityIncrease(EffectAbilityIncrease {
                        ability_type: AbilityType::Faith,
                        amount: 10,
                    })],
                    level3_effects: vec![
                        HeartEffect::AbilityIncrease(EffectAbilityIncrease {
                            ability_type: AbilityType::Strength,
                            amount: 10,
                        }),
                        HeartEffect::AbilityIncrease(EffectAbilityIncrease {
                            ability_type: AbilityType::Intelligence,
                            amount: 10,
                        }),
                    ],
                    combination: None,
                },
                current_trance: 0,
            }),
            combination_skill: Some(BattleCombinationSkill {
                combination_skills: vec![
                    CombinationSkill {
                        name: "連撃".to_string(),
                        effects: vec![
                            HeartCombinationEffect::AttackDamageModifier(
                                EffectAttackDamageModifier { modifier: 1.2 },
                            ),
                            HeartCombinationEffect::AttackBreakDamageModifier(
                                EffectAttackBreakDamageModifier { modifier: 1.2 },
                            ),
                        ],
                        condition: CombinationSkillCondition {
                            current_requirements: CombinationSkillConditionRequirements {
                                categories: vec![CombinationConductCategory::Attack],
                                results: vec![],
                            },
                            previous_requirements: Some(CombinationSkillConditionRequirements {
                                categories: vec![CombinationConductCategory::Attack],
                                results: vec![CombinationConductResult::Success],
                            }),
                            two_steps_before_requirements: None,
                        },
                    },
                    CombinationSkill {
                        name: "ガードカウンター".to_string(),
                        effects: vec![
                            HeartCombinationEffect::AttackDamageModifier(
                                EffectAttackDamageModifier { modifier: 1.5 },
                            ),
                            HeartCombinationEffect::AttackBreakDamageModifier(
                                EffectAttackBreakDamageModifier { modifier: 2.0 },
                            ),
                        ],
                        condition: CombinationSkillCondition {
                            current_requirements: CombinationSkillConditionRequirements {
                                categories: vec![CombinationConductCategory::Attack],
                                results: vec![],
                            },
                            previous_requirements: Some(CombinationSkillConditionRequirements {
                                categories: vec![CombinationConductCategory::Guard],
                                results: vec![CombinationConductResult::GuardSuccess],
                            }),
                            two_steps_before_requirements: None,
                        },
                    },
                ],
                current_combination_conduct_log: None,
                combination_logs: vec![],
            }),
        },
        enemies: vec![BattleCharacter {
            character_id: 2,
            raw_ability: enemy_original.ability.clone(),
            raw_base_defense_power: def.clone(),
            raw_equipment: enemy_original.equipment.clone(),
            character_type: BattleCharacterType::Enemy,
            hp: BattleCharacterHP {
                max_hp: enemy_original.stats.hp,
                current_hp: enemy_original.stats.hp,
                is_dead: false,
            },
            sp: BattleCharacterSP {
                max_sp: enemy_original.stats.sp,
                current_sp: enemy_original.stats.sp,
            },
            stamina: BattleCharacterStamina {
                max_stamina: 0,
                current_stamina: 0,
                stamina_recovery: 0,
            },
            max_equipment_weight: 0,
            weapons: vec![
                BattleWeapon {
                    id: BattleWeaponId(0),
                    weapon: Weapon {
                        name: "爪".to_string(),
                        kind: WeaponKind::StraightSword,
                        weight: 0,
                        ability_requirement: WeaponAbilityRequirement {
                            strength: 0,
                            dexterity: 0,
                            intelligence: 0,
                            faith: 0,
                            arcane: 0,
                            agility: 0,
                        },
                        attack_power: WeaponAttackPower {
                            base: AttackPower {
                                slash: 500,
                                strike: 0,
                                thrust: 0,
                                impact: 0,
                                magic: 0,
                                fire: 0,
                                lightning: 0,
                                chaos: 0,
                            },
                            ability_scaling: WeaponAttackPowerAbilityScaling {
                                slash: AbilityScaling::default(),
                                strike: AbilityScaling::default(),
                                thrust: AbilityScaling::default(),
                                impact: AbilityScaling::default(),
                                magic: AbilityScaling::default(),
                                fire: AbilityScaling::default(),
                                lightning: AbilityScaling::default(),
                                chaos: AbilityScaling::default(),
                            },
                        },
                        sorcery_power: WeaponSorceryPower {
                            base: 0,
                            scaling: AbilityScaling::default(),
                        },
                        break_power: WeaponBreakPower {
                            base_power: 50,
                            scaling: AbilityScaling {
                                strength: 1.0,
                                dexterity: 0.0,
                                intelligence: 0.0,
                                faith: 0.0,
                                arcane: 0.0,
                                agility: 0.0,
                            },
                        },
                        guard: WeaponGuard {
                            cut_rate: GuardCutRate {
                                slash: 0.5,
                                strike: 0.5,
                                thrust: 0.5,
                                impact: 0.5,
                                magic: 0.5,
                                fire: 0.5,
                                lightning: 0.5,
                                chaos: 0.5,
                            },
                            guard_strength: 20,
                        },
                    },
                },
                BattleWeapon {
                    id: BattleWeaponId(1),
                    weapon: Weapon {
                        name: "尻尾".to_string(),
                        kind: WeaponKind::StraightSword,
                        weight: 0,
                        ability_requirement: WeaponAbilityRequirement {
                            strength: 0,
                            dexterity: 0,
                            intelligence: 0,
                            faith: 0,
                            arcane: 0,
                            agility: 0,
                        },
                        attack_power: WeaponAttackPower {
                            base: AttackPower {
                                slash: 0,
                                strike: 400,
                                thrust: 0,
                                impact: 0,
                                magic: 0,
                                fire: 0,
                                lightning: 0,
                                chaos: 0,
                            },
                            ability_scaling: WeaponAttackPowerAbilityScaling {
                                slash: AbilityScaling::default(),
                                strike: AbilityScaling::default(),
                                thrust: AbilityScaling::default(),
                                impact: AbilityScaling::default(),
                                magic: AbilityScaling::default(),
                                fire: AbilityScaling::default(),
                                lightning: AbilityScaling::default(),
                                chaos: AbilityScaling::default(),
                            },
                        },
                        sorcery_power: WeaponSorceryPower {
                            base: 0,
                            scaling: AbilityScaling::default(),
                        },
                        break_power: WeaponBreakPower {
                            base_power: 30,
                            scaling: AbilityScaling {
                                strength: 3.0,
                                dexterity: 0.0,
                                intelligence: 0.0,
                                faith: 0.0,
                                arcane: 0.0,
                                agility: 0.0,
                            },
                        },
                        guard: WeaponGuard {
                            cut_rate: GuardCutRate {
                                slash: 0.5,
                                strike: 0.5,
                                thrust: 0.5,
                                impact: 0.5,
                                magic: 0.5,
                                fire: 0.5,
                                lightning: 0.5,
                                chaos: 0.5,
                            },
                            guard_strength: 20,
                        },
                    },
                },
            ],
            status_conditions: vec![],
            status_ailment: BattleStatusAilment {
                poison: BattleStatusAilmentStatus::new_poison(),
                sleep: BattleStatusAilmentStatus::new_sleep(),
                chill: BattleStatusAilmentStatus::new_chill(),
                bleed: BattleStatusAilmentStatus::new_bleed(),
                burn: BattleStatusAilmentStatus::new_burn(),
                paralysis: BattleStatusAilmentStatus::new_paralysis(),
                fear: BattleStatusAilmentStatus::new_fear(),
                rage: BattleStatusAilmentStatus::new_rage(),
                breaking: BattleStatusAilmentStatus::new_breaking(50),
            },
            karma: None,
            trance: None,
            combination_skill: None,
        }],

        enemy_action_progress: None,
        enemy_second_stage: false,
        enemy_action_lots: vec![
            EnemyActionLot {
                action: EnemyAction {
                    name: "ひっかき（単）".to_string(),
                    commands: vec![EnemyCommandId(1)],
                },
                weight: 40,
            },
            EnemyActionLot {
                action: EnemyAction {
                    name: "ひっかき（2連）".to_string(),
                    commands: vec![EnemyCommandId(4), EnemyCommandId(5)],
                },
                weight: 10,
            },
            EnemyActionLot {
                action: EnemyAction {
                    name: "噛みつき".to_string(),
                    commands: vec![EnemyCommandId(2)],
                },
                weight: 20,
            },
            EnemyActionLot {
                action: EnemyAction {
                    name: "尾撃（単）".to_string(),
                    commands: vec![EnemyCommandId(3)],
                },
                weight: 20,
            },
            EnemyActionLot {
                action: EnemyAction {
                    name: "尾撃（2連）".to_string(),
                    commands: vec![EnemyCommandId(3), EnemyCommandId(3)],
                },
                weight: 10,
            },
        ],
        enemy_second_stage_action_lots: vec![
            EnemyActionLot {
                action: EnemyAction {
                    name: "ひっかき（単）".to_string(),
                    commands: vec![EnemyCommandId(1)],
                },
                weight: 10,
            },
            EnemyActionLot {
                action: EnemyAction {
                    name: "ひっかき（2連）".to_string(),
                    commands: vec![EnemyCommandId(4), EnemyCommandId(5)],
                },
                weight: 20,
            },
            EnemyActionLot {
                action: EnemyAction {
                    name: "ひっかき（3連）".to_string(),
                    commands: vec![EnemyCommandId(4), EnemyCommandId(5), EnemyCommandId(6)],
                },
                weight: 20,
            },
            EnemyActionLot {
                action: EnemyAction {
                    name: "噛みつき".to_string(),
                    commands: vec![EnemyCommandId(2)],
                },
                weight: 30,
            },
            EnemyActionLot {
                action: EnemyAction {
                    name: "尾撃（単）".to_string(),
                    commands: vec![EnemyCommandId(3)],
                },
                weight: 10,
            },
            EnemyActionLot {
                action: EnemyAction {
                    name: "尾撃（2連）".to_string(),
                    commands: vec![EnemyCommandId(3), EnemyCommandId(3)],
                },
                weight: 20,
            },
            EnemyActionLot {
                action: EnemyAction {
                    name: "ファイアブレス".to_string(),
                    commands: vec![
                        EnemyCommandId(7),
                        EnemyCommandId(8),
                        EnemyCommandId(8),
                        EnemyCommandId(8),
                    ],
                },
                weight: 20,
            },
        ],
        // 敵のアーツ
        enemy_commands: vec![
            EnemyCommand {
                id: EnemyCommandId(0),
                art: Arc::new(Art {
                    name: "待機".to_string(),
                    sp_cost: 0,
                    stamina_cost: 0,
                    perks: vec![],
                    requirement: ArtRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    art_type: ArtType::Basic,
                    usable_weapon: ArtUsableWeapon::All,
                    always_hits: false,
                    priority: 0,
                    rank1: ArtRank {
                        threshold: 0,
                        target: ArtTarget::Single,
                        potency: ArtPotency::Support(ArtPotencySupport::None),
                    },
                    rank2: None,
                    rank3: None,
                }),
                battle_weapon_id: Some(BattleWeaponId(0)),
            },
            EnemyCommand {
                id: EnemyCommandId(1),
                art: Arc::new(Art {
                    name: "ひっかき".to_string(),
                    sp_cost: 0,
                    stamina_cost: 0,
                    perks: vec![ArtPerk::Melee],
                    requirement: ArtRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    art_type: ArtType::Basic,
                    usable_weapon: ArtUsableWeapon::All,
                    always_hits: false,
                    priority: 0,
                    rank1: ArtRank {
                        threshold: 0,
                        target: ArtTarget::Single,
                        potency: ArtPotency::Attack(ArtPotencyAttack {
                            attack_power: AttackPower {
                                slash: 0,
                                strike: 0,
                                thrust: 0,
                                impact: 0,
                                magic: 0,
                                fire: 0,
                                lightning: 0,
                                chaos: 0,
                            },
                            weapon_attack_power_scaling: AttackPowerScaling {
                                slash: 1.0,
                                strike: 0.0,
                                thrust: 0.0,
                                impact: 0.0,
                                magic: 0.0,
                                fire: 0.0,
                                lightning: 0.0,
                                chaos: 0.0,
                            },
                            break_power: 30,
                            weapon_break_power_scaling: 1.0,
                        }),
                    },
                    rank2: None,
                    rank3: None,
                }),
                battle_weapon_id: Some(BattleWeaponId(0)),
            },
            EnemyCommand {
                id: EnemyCommandId(2),
                art: Arc::new(Art {
                    name: "噛みつき".to_string(),
                    sp_cost: 0,
                    stamina_cost: 0,
                    perks: vec![ArtPerk::Melee],
                    requirement: ArtRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    art_type: ArtType::Basic,
                    usable_weapon: ArtUsableWeapon::All,
                    always_hits: false,
                    priority: 0,
                    rank1: ArtRank {
                        threshold: 0,
                        target: ArtTarget::Single,
                        potency: ArtPotency::Attack(ArtPotencyAttack {
                            attack_power: AttackPower {
                                slash: 0,
                                strike: 0,
                                thrust: 0,
                                impact: 0,
                                magic: 0,
                                fire: 0,
                                lightning: 0,
                                chaos: 0,
                            },
                            weapon_attack_power_scaling: AttackPowerScaling {
                                slash: 4.0,
                                strike: 0.0,
                                thrust: 0.0,
                                impact: 0.0,
                                magic: 0.0,
                                fire: 0.0,
                                lightning: 0.0,
                                chaos: 0.0,
                            },
                            break_power: 50,
                            weapon_break_power_scaling: 3.0,
                        }),
                    },
                    rank2: None,
                    rank3: None,
                }),
                battle_weapon_id: Some(BattleWeaponId(0)),
            },
            EnemyCommand {
                id: EnemyCommandId(3),
                art: Arc::new(Art {
                    name: "尾撃".to_string(),
                    sp_cost: 0,
                    stamina_cost: 0,
                    perks: vec![ArtPerk::AtFeet],
                    requirement: ArtRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    art_type: ArtType::Basic,
                    usable_weapon: ArtUsableWeapon::All,
                    always_hits: false,
                    priority: -1,
                    rank1: ArtRank {
                        threshold: 0,
                        target: ArtTarget::Single,
                        potency: ArtPotency::Attack(ArtPotencyAttack {
                            attack_power: AttackPower {
                                slash: 0,
                                strike: 0,
                                thrust: 0,
                                impact: 0,
                                magic: 0,
                                fire: 0,
                                lightning: 0,
                                chaos: 0,
                            },
                            weapon_attack_power_scaling: AttackPowerScaling {
                                slash: 0.0,
                                strike: 5.0,
                                thrust: 0.0,
                                impact: 0.0,
                                magic: 0.0,
                                fire: 0.0,
                                lightning: 0.0,
                                chaos: 0.0,
                            },
                            break_power: 50,
                            weapon_break_power_scaling: 3.0,
                        }),
                    },
                    rank2: None,
                    rank3: None,
                }),
                battle_weapon_id: Some(BattleWeaponId(1)),
            },
            EnemyCommand {
                id: EnemyCommandId(4),
                art: Arc::new(Art {
                    name: "連続ひっかき(1)".to_string(),
                    sp_cost: 0,
                    stamina_cost: 0,
                    perks: vec![ArtPerk::Melee],
                    requirement: ArtRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    art_type: ArtType::Basic,
                    usable_weapon: ArtUsableWeapon::All,
                    always_hits: false,
                    priority: 0,
                    rank1: ArtRank {
                        threshold: 0,
                        target: ArtTarget::Single,
                        potency: ArtPotency::Attack(ArtPotencyAttack {
                            attack_power: AttackPower {
                                slash: 0,
                                strike: 0,
                                thrust: 0,
                                impact: 0,
                                magic: 0,
                                fire: 0,
                                lightning: 0,
                                chaos: 0,
                            },
                            weapon_attack_power_scaling: AttackPowerScaling {
                                slash: 0.9,
                                strike: 0.0,
                                thrust: 0.0,
                                impact: 0.0,
                                magic: 0.0,
                                fire: 0.0,
                                lightning: 0.0,
                                chaos: 0.0,
                            },
                            break_power: 30,
                            weapon_break_power_scaling: 1.0,
                        }),
                    },
                    rank2: None,
                    rank3: None,
                }),
                battle_weapon_id: Some(BattleWeaponId(0)),
            },
            EnemyCommand {
                id: EnemyCommandId(5),
                art: Arc::new(Art {
                    name: "連続ひっかき(2)".to_string(),
                    sp_cost: 0,
                    stamina_cost: 0,
                    perks: vec![ArtPerk::Melee],
                    requirement: ArtRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    art_type: ArtType::Basic,
                    usable_weapon: ArtUsableWeapon::All,
                    always_hits: false,
                    priority: 0,
                    rank1: ArtRank {
                        threshold: 0,
                        target: ArtTarget::Single,
                        potency: ArtPotency::Attack(ArtPotencyAttack {
                            attack_power: AttackPower {
                                slash: 0,
                                strike: 0,
                                thrust: 0,
                                impact: 0,
                                magic: 0,
                                fire: 0,
                                lightning: 0,
                                chaos: 0,
                            },
                            weapon_attack_power_scaling: AttackPowerScaling {
                                slash: 1.1,
                                strike: 0.0,
                                thrust: 0.0,
                                impact: 0.0,
                                magic: 0.0,
                                fire: 0.0,
                                lightning: 0.0,
                                chaos: 0.0,
                            },
                            break_power: 30,
                            weapon_break_power_scaling: 1.0,
                        }),
                    },
                    rank2: None,
                    rank3: None,
                }),
                battle_weapon_id: Some(BattleWeaponId(0)),
            },
            EnemyCommand {
                id: EnemyCommandId(6),
                art: Arc::new(Art {
                    name: "連続ひっかき(3)".to_string(),
                    sp_cost: 0,
                    stamina_cost: 0,
                    perks: vec![ArtPerk::Melee],
                    requirement: ArtRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    art_type: ArtType::Basic,
                    usable_weapon: ArtUsableWeapon::All,
                    always_hits: false,
                    priority: 0,
                    rank1: ArtRank {
                        threshold: 0,
                        target: ArtTarget::Single,
                        potency: ArtPotency::Attack(ArtPotencyAttack {
                            attack_power: AttackPower {
                                slash: 0,
                                strike: 0,
                                thrust: 0,
                                impact: 0,
                                magic: 0,
                                fire: 0,
                                lightning: 0,
                                chaos: 0,
                            },
                            weapon_attack_power_scaling: AttackPowerScaling {
                                slash: 1.5,
                                strike: 0.0,
                                thrust: 0.0,
                                impact: 0.0,
                                magic: 0.0,
                                fire: 0.0,
                                lightning: 0.0,
                                chaos: 0.0,
                            },
                            break_power: 60,
                            weapon_break_power_scaling: 2.0,
                        }),
                    },
                    rank2: None,
                    rank3: None,
                }),
                battle_weapon_id: Some(BattleWeaponId(0)),
            },
            EnemyCommand {
                id: EnemyCommandId(7),
                art: Arc::new(Art {
                    name: "息を吸い込む".to_string(),
                    sp_cost: 0,
                    stamina_cost: 0,
                    perks: vec![],
                    requirement: ArtRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    art_type: ArtType::Sorcery,
                    usable_weapon: ArtUsableWeapon::All,
                    always_hits: false,
                    priority: 0,
                    rank1: ArtRank {
                        threshold: 0,
                        target: ArtTarget::Single,
                        potency: ArtPotency::Support(ArtPotencySupport::None),
                    },
                    rank2: None,
                    rank3: None,
                }),
                battle_weapon_id: None,
            },
            EnemyCommand {
                id: EnemyCommandId(8),
                art: Arc::new(Art {
                    name: "ファイアブレス".to_string(),
                    sp_cost: 0,
                    stamina_cost: 0,
                    perks: vec![],
                    requirement: ArtRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    art_type: ArtType::Sorcery,
                    usable_weapon: ArtUsableWeapon::All,
                    always_hits: false,
                    priority: 0,
                    rank1: ArtRank {
                        threshold: 0,
                        target: ArtTarget::Single,
                        potency: ArtPotency::Attack(ArtPotencyAttack {
                            attack_power: AttackPower {
                                slash: 0,
                                strike: 0,
                                thrust: 0,
                                impact: 0,
                                magic: 0,
                                fire: 1500,
                                lightning: 0,
                                chaos: 0,
                            },
                            weapon_attack_power_scaling: AttackPowerScaling {
                                slash: 0.0,
                                strike: 0.0,
                                thrust: 0.0,
                                impact: 0.0,
                                magic: 0.0,
                                fire: 0.0,
                                lightning: 0.0,
                                chaos: 0.0,
                            },
                            break_power: 100,
                            weapon_break_power_scaling: 1.0,
                        }),
                    },
                    rank2: None,
                    rank3: None,
                }),
                battle_weapon_id: None,
            },
        ],
    }
}
