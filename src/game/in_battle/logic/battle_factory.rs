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
            stamina_cost: 0,
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
                                    battle_weapon_id: BattleWeaponId(0), // TODO: ここどうするか、とりあえずあとから選択した武器のIDで上書きしている
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
                        slash: 0.7,
                        strike: 0.7,
                        thrust: 0.7,
                        impact: 0.7,
                        magic: 0.7,
                        fire: 0.7,
                        lightning: 0.7,
                        chaos: 0.7,
                    },
                    break_power: 10,
                    weapon_break_power_scaling: 0.5,
                    additional_effects: vec![],
                }),
            },
            rank2: None,
            rank3: None,
        }),
        // 強攻撃
        Arc::new(Art {
            name: "強攻撃".to_string(),
            sp_cost: 0,
            stamina_cost: 20,
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
                        slash: 1.3,
                        strike: 1.3,
                        thrust: 1.3,
                        impact: 1.3,
                        magic: 1.3,
                        fire: 1.3,
                        lightning: 1.3,
                        chaos: 1.3,
                    },
                    break_power: 50,
                    weapon_break_power_scaling: 1.5,
                    additional_effects: vec![],
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
        stats: EnemyStats { hp: 1700, sp: 30 },
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

    // 敵コマンド
    let enemy_command_wait = create_enemy_command(
        "待機",
        ArtType::Basic,
        0,
        ArtRank {
            threshold: 0,
            target: ArtTarget::Single,
            potency: ArtPotency::Support(ArtPotencySupport::None),
        },
        Some(BattleWeaponId(0)),
    );
    // 通常ひっかき
    let enemy_command_scratch = create_enemy_command(
        "ひっかき",
        ArtType::Basic,
        0,
        ArtRank {
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
                additional_effects: vec![],
            }),
        },
        Some(BattleWeaponId(0)),
    );
    // 弱ひっかき
    let enemy_command_scratch2 = create_enemy_command(
        "ひっかき",
        ArtType::Basic,
        0,
        ArtRank {
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
                    slash: 0.7,
                    strike: 0.0,
                    thrust: 0.0,
                    impact: 0.0,
                    magic: 0.0,
                    fire: 0.0,
                    lightning: 0.0,
                    chaos: 0.0,
                },
                break_power: 20,
                weapon_break_power_scaling: 0.5,
                additional_effects: vec![],
            }),
        },
        Some(BattleWeaponId(0)),
    );
    let enemy_command_crush = create_enemy_command(
        "両手押しつぶし",
        ArtType::Basic,
        -1,
        ArtRank {
            threshold: 0,
            target: ArtTarget::Single,
            potency: ArtPotency::Attack(ArtPotencyAttack {
                attack_power: AttackPower {
                    slash: 0,
                    strike: 0,
                    thrust: 0,
                    impact: 500,
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
                break_power: 100,
                weapon_break_power_scaling: 3.0,
                additional_effects: vec![],
            }),
        },
        Some(BattleWeaponId(0)),
    );
    let enemy_command_bite = create_enemy_command(
        "噛みつき",
        ArtType::Basic,
        0,
        ArtRank {
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
                break_power: 150,
                weapon_break_power_scaling: 3.0,
                additional_effects: vec![AdditionalEffect {
                    target: AdditionalEffectTarget::AttackTarget,
                    content: ArtPotencySupport::StatusAilment(ArtPotencySupportStatusAilment {
                        kind: StatusAilment::Poison,
                        accumulation: 600,
                    }),
                }],
            }),
        },
        Some(BattleWeaponId(0)),
    );
    let enemy_command_tail = create_enemy_command(
        "尾撃",
        ArtType::Basic,
        -1,
        ArtRank {
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
                break_power: 150,
                weapon_break_power_scaling: 3.0,
                additional_effects: vec![],
            }),
        },
        Some(BattleWeaponId(1)),
    );
    // 息を吸い込む
    let enemy_command_breath = create_enemy_command(
        "息を吸い込む",
        ArtType::Sorcery,
        0,
        ArtRank {
            threshold: 0,
            target: ArtTarget::Single,
            potency: ArtPotency::Support(ArtPotencySupport::None),
        },
        None,
    );
    // ファイアブレス
    let enemy_command_fire_breath = create_enemy_command(
        "ファイアブレス",
        ArtType::Sorcery,
        0,
        ArtRank {
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
                additional_effects: vec![],
            }),
        },
        None,
    );

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
                    KarmaDeckCard { card_id: KarmaCardId(1) }, // ラッキーパンチ
                    KarmaDeckCard { card_id: KarmaCardId(2) }, // 好調
                    KarmaDeckCard { card_id: KarmaCardId(2) },
                    KarmaDeckCard { card_id: KarmaCardId(2) },
                    KarmaDeckCard { card_id: KarmaCardId(3) }, // 堅実
                    KarmaDeckCard { card_id: KarmaCardId(3) },
                    KarmaDeckCard { card_id: KarmaCardId(3) },
                    KarmaDeckCard { card_id: KarmaCardId(3) },
                    KarmaDeckCard { card_id: KarmaCardId(4) }, // 追い風
                    KarmaDeckCard { card_id: KarmaCardId(4) },
                    KarmaDeckCard { card_id: KarmaCardId(4) },
                    KarmaDeckCard { card_id: KarmaCardId(4) },
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
                                slash: 700,
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
                                strike: 800,
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
                                strength: 5.0,
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
        enemy_ai_state: EnemyAiState::default(),
        enemy_behavior_tree: EnemyBehaviorTree {
            phases: vec![
                // フェーズ0: 通常段階（HP60%超）
                EnemyPhase {
                    enter_condition: None,
                    entry_action: None,
                    root: BehaviorNode::WeightedRandom(vec![
                        WeightedChoice {
                            weight: 20,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "ひっかき(単)1",
                                    [
                                        enemy_command_scratch.id,
                                        enemy_command_wait.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("右前脚を振り上げる"),
                            ),
                        },
                        WeightedChoice {
                            weight: 20,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "ひっかき(単)2",
                                    [
                                        enemy_command_wait.id,
                                        enemy_command_scratch.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("右前脚を振り上げる"),
                            ),
                        },
                        WeightedChoice {
                            weight: 5,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "ひっかき（2連）",
                                    [
                                        enemy_command_scratch2.id,
                                        enemy_command_scratch.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("左前脚を振り上げる"),
                            ),
                        },
                        WeightedChoice {
                            weight: 15,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "噛みつき1",
                                    [
                                        enemy_command_bite.id,
                                        enemy_command_wait.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("こちらを見ながら近づいてくる"),
                            ),
                        },
                        WeightedChoice {
                            weight: 15,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "噛みつき2",
                                    [
                                        enemy_command_wait.id,
                                        enemy_command_bite.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("こちらを見ながら近づいてくる"),
                            ),
                        },
                        WeightedChoice {
                            weight: 10,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "尾撃（単）",
                                    [
                                        enemy_command_tail.id,
                                        enemy_command_wait.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("体をねじっている"),
                            ),
                        },
                        WeightedChoice {
                            weight: 5,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "尾撃（2連）",
                                    [
                                        enemy_command_tail.id,
                                        enemy_command_tail.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("体をねじっている"),
                            ),
                        },
                    ]),
                },
                // フェーズ1: 強化段階（HP60%以下）
                EnemyPhase {
                    enter_condition: Some(PhaseCondition::HpBelow {
                        threshold_percent: 0.6,
                    }),
                    entry_action: None,
                    root: BehaviorNode::WeightedRandom(vec![
                        WeightedChoice {
                            weight: 10,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "ひっかき（2連）",
                                    [
                                        enemy_command_wait.id,
                                        enemy_command_scratch2.id,
                                        enemy_command_scratch.id,
                                    ],
                                )
                                .with_hint("左前脚を振り上げる"),
                            ),
                        },
                        WeightedChoice {
                            weight: 10,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "ひっかき（2連）",
                                    [
                                        enemy_command_scratch2.id,
                                        enemy_command_scratch.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("左前脚を振り上げる"),
                            ),
                        },
                        WeightedChoice {
                            weight: 10,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "ひっかきコンボ",
                                    [
                                        enemy_command_scratch2.id,
                                        enemy_command_scratch.id,
                                        enemy_command_crush.id,
                                    ],
                                )
                                .with_hint("左前脚を振り上げる"),
                            ),
                        },
                        WeightedChoice {
                            weight: 20,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "連続噛みつき",
                                    [
                                        enemy_command_bite.id,
                                        enemy_command_bite.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("こちらを見ながら近づいてくる"),
                            ),
                        },
                        WeightedChoice {
                            weight: 5,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "尾撃（単）1",
                                    [
                                        enemy_command_tail.id,
                                        enemy_command_wait.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("体をねじっている"),
                            ),
                        },
                        WeightedChoice {
                            weight: 5,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "尾撃（単）2",
                                    [
                                        enemy_command_wait.id,
                                        enemy_command_tail.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("体をねじっている"),
                            ),
                        },
                        WeightedChoice {
                            weight: 20,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "尾撃（2連）",
                                    [
                                        enemy_command_tail.id,
                                        enemy_command_tail.id,
                                        enemy_command_wait.id,
                                    ],
                                )
                                .with_hint("体をひねってる"),
                            ),
                        },
                        WeightedChoice {
                            weight: 20,
                            node: BehaviorNode::Fixed(
                                ActionSet::new(
                                    "ファイアブレス",
                                    [
                                        enemy_command_fire_breath.id,
                                        enemy_command_fire_breath.id,
                                        enemy_command_fire_breath.id,
                                    ],
                                )
                                .with_hint("大きく息を吸い込んでいる"),
                            ),
                        },
                    ]),
                },
            ],
        },
        // 敵のアーツ
        enemy_commands: vec![
            enemy_command_wait,
            enemy_command_scratch,
            enemy_command_scratch2,
            enemy_command_crush,
            enemy_command_bite,
            enemy_command_tail,
            enemy_command_breath,
            enemy_command_fire_breath,
        ],
    }
}

static ENEMY_COMMAND_ID_COUNTER: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(100);
fn create_enemy_command(
    name: &str,
    art_type: ArtType,
    priority: i32,
    rank1: ArtRank,
    battle_weapon_id: Option<BattleWeaponId>,
) -> EnemyCommand {
    EnemyCommand {
        id: EnemyCommandId(
            ENEMY_COMMAND_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        ),
        art: Arc::new(Art {
            name: name.to_string(),
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
            art_type,
            usable_weapon: ArtUsableWeapon::All,
            always_hits: false,
            priority,
            rank1,
            rank2: None,
            rank3: None,
        }),
        battle_weapon_id,
    }
}
