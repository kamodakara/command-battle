use crate::battle::{
    BattleCharacterController, BattleController, BattleDecideOrderRequest,
    BattleExecuteConductRequest, BattleTranceController, DecideEnemyConductRequest,
};
use crate::fundamental::*;

use super::*;

use bevy::{log, prelude::*};
use rand::Rng;
use std::sync::Arc;

use super::{ArtsDatabase, EquipmentDatabase, PreparationState};

// ================== Plugin ==================
pub struct InBattlePlugin;

impl Plugin for InBattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Battle), setup_battle_screen)
            .add_systems(OnExit(GameState::Battle), cleanup_battle_screen)
            .add_systems(
                Update,
                player_input_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                back_button_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                action_menu_click_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                action_menu_update_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                battle_end_check_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(Update, ui_update_system.run_if(in_state(GameState::Battle)))
            .add_systems(
                Update,
                ui_update_enemy_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                ui_update_enemy_damage_popup_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                ui_update_player_status_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                ui_update_command_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                ui_update_message_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                ui_update_skill_effect_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                ui_update_karma_cards_system.run_if(in_state(GameState::Battle)),
            )
            .add_systems(
                Update,
                boss_slain_banner_system.run_if(in_state(GameState::Battle)),
            );
    }
}

#[derive(Resource, PartialEq, Eq)]
enum BattlePhase {
    DecideEnemyConduct, // 敵行動決定
    AwaitCommand,       // プレイヤーコマンド入力待ち
    ConfirmQueued,      // 連続コマンドの次コマンドを実行するか確認するフェーズ
    InBattle,
    TurnEnd,  // ターン終了処理
    Finished, // 戦闘終了
}
#[derive(Resource)]
struct Turn(u32);

#[derive(Resource)]
struct CombatLog(Vec<String>);

// 敵ダメージポップアップ用リソース（タイマー制御）
#[derive(Resource, Default)]
struct EnemyDamagePopup {
    amount: i32,
    timer: f32, // 秒。0以下で非表示
}

// 連続コマンド実行バッチの総件数（選択確定時に設定）
#[derive(Resource, Default)]
struct ConsecutiveBatch {
    total: usize,    // このバッチの総選択数
    executed: usize, // このバッチで既に実行した数
}

// カルマカードUIの初期描画フラグ
#[derive(Resource, Default)]
struct KarmaCardsNeedsRedraw(bool);

fn player_standard_arts() -> Vec<Arc<Art>> {
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

fn player_arts() -> Vec<Arc<Art>> {
    vec![
        // 基本アーツ
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
        // 技
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
                        slash: 25,
                        strike: 0,
                        thrust: 0,
                        impact: 0,
                        magic: 0,
                        fire: 0,
                        lightning: 0,
                        chaos: 0,
                    },
                    weapon_attack_power_scaling: AttackPowerScaling::default(),
                    break_power: 10,
                    weapon_break_power_scaling: 0.0,
                }),
            },
            rank2: None,
            rank3: None,
        }),
        Arc::new(Art {
            name: "強攻撃".to_string(),
            sp_cost: 0,
            stamina_cost: 25,
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
                        slash: 40,
                        strike: 0,
                        thrust: 0,
                        impact: 0,
                        magic: 0,
                        fire: 0,
                        lightning: 0,
                        chaos: 0,
                    },
                    weapon_attack_power_scaling: AttackPowerScaling::default(),
                    break_power: 20,
                    weapon_break_power_scaling: 0.0,
                }),
            },
            rank2: None,
            rank3: None,
        }),
        Arc::new(Art {
            name: "回復".to_string(),
            sp_cost: 0,
            stamina_cost: 25,
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
            priority: 0,
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
        Arc::new(Art {
            name: "横斬り".to_string(),
            sp_cost: 0,
            stamina_cost: 15,
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
            usable_weapon: ArtUsableWeapon::Specific(vec![WeaponKind::StraightSword]),
            always_hits: false,
            priority: 0,
            rank1: ArtRank {
                threshold: 0,
                target: ArtTarget::Single,
                potency: ArtPotency::Attack(ArtPotencyAttack {
                    attack_power: AttackPower {
                        slash: 40,
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
                    break_power: 15,
                    weapon_break_power_scaling: 1.0,
                }),
            },
            rank2: None,
            rank3: None,
        }),
        Arc::new(Art {
            name: "突き".to_string(),
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
            usable_weapon: ArtUsableWeapon::Specific(vec![WeaponKind::StraightSword]),
            always_hits: false,
            priority: 0,
            rank1: ArtRank {
                threshold: 0,
                target: ArtTarget::Single,
                potency: ArtPotency::Attack(ArtPotencyAttack {
                    attack_power: AttackPower {
                        slash: 0,
                        strike: 0,
                        thrust: 50,
                        impact: 0,
                        magic: 0,
                        fire: 0,
                        lightning: 0,
                        chaos: 0,
                    },
                    weapon_attack_power_scaling: AttackPowerScaling {
                        slash: 0.0,
                        strike: 0.0,
                        thrust: 1.2,
                        impact: 0.0,
                        magic: 0.0,
                        fire: 0.0,
                        lightning: 0.0,
                        chaos: 0.0,
                    },
                    break_power: 20,
                    weapon_break_power_scaling: 1.0,
                }),
            },
            rank2: None,
            rank3: None,
        }),
        // 術
        Arc::new(Art {
            name: "ファイアボール".to_string(),
            sp_cost: 10,
            stamina_cost: 10,
            perks: vec![ArtPerk::Ranged],
            requirement: ArtRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 10,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            art_type: ArtType::Sorcery,
            usable_weapon: ArtUsableWeapon::Specific(vec![WeaponKind::Staff]),
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
                        fire: 60,
                        lightning: 0,
                        chaos: 0,
                    },
                    weapon_attack_power_scaling: AttackPowerScaling::default(),
                    break_power: 25,
                    weapon_break_power_scaling: 0.0,
                }),
            },
            rank2: None,
            rank3: None,
        }),
        Arc::new(Art {
            name: "ヒール".to_string(),
            sp_cost: 15,
            stamina_cost: 5,
            perks: vec![],
            requirement: ArtRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 5,
                faith: 5,
                arcane: 0,
                agility: 0,
            },
            art_type: ArtType::Sorcery,
            usable_weapon: ArtUsableWeapon::Specific(vec![WeaponKind::Staff]),
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
    ]
}

fn create_basic_arts() -> Vec<Arc<Art>> {
    player_arts()
        .into_iter()
        .filter(|art| art.art_type == ArtType::Basic || art.art_type == ArtType::Sorcery)
        .map(|art| Arc::clone(&art))
        .collect()
}

// サンプル武器と対応するスキル・術を作成
fn create_equipped_weapons_with_arts() -> PlayerEquippedWeapons {
    let arts = player_arts();

    // 直剣
    let straight_sword = Weapon {
        name: "ロングソード".to_string(),
        kind: WeaponKind::StraightSword,
        weight: 4,
        ability_requirement: WeaponAbilityRequirement {
            strength: 10,
            dexterity: 10,
            intelligence: 0,
            faith: 0,
            arcane: 0,
            agility: 0,
        },
        attack_power: WeaponAttackPower {
            base: AttackPower {
                slash: 30,
                strike: 0,
                thrust: 5,
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
            base_power: 15,
            scaling: AbilityScaling::default(),
        },
        guard: WeaponGuard {
            cut_rate: GuardCutRate {
                slash: 0.5,
                strike: 0.5,
                thrust: 0.5,
                impact: 0.5,
                magic: 0.2,
                fire: 0.2,
                lightning: 0.2,
                chaos: 0.2,
            },
            guard_strength: 30,
        },
    };
    let straight_sword_skills = arts
        .iter()
        .filter(|art| {
            art.art_type == ArtType::Skill
                && (
            art.usable_weapon == ArtUsableWeapon::All
                || matches!(
                    &art.usable_weapon,
                    ArtUsableWeapon::Specific(kinds) if kinds.contains(&WeaponKind::StraightSword)
                ))
        })
        .map(|art| Arc::clone(art))
        .collect::<Vec<Arc<Art>>>();
    let straight_sword_sorceries: Vec<Arc<Art>> = arts
        .iter()
        .filter(|art| art.art_type == ArtType::Sorcery)
        .map(|art| Arc::clone(art))
        .collect::<Vec<Arc<Art>>>();

    // 杖
    let staff = Weapon {
        name: "賢者の杖".to_string(),
        kind: WeaponKind::Staff,
        weight: 2,
        ability_requirement: WeaponAbilityRequirement {
            strength: 0,
            dexterity: 0,
            intelligence: 15,
            faith: 0,
            arcane: 0,
            agility: 0,
        },
        attack_power: WeaponAttackPower {
            base: AttackPower {
                slash: 0,
                strike: 10,
                thrust: 0,
                impact: 0,
                magic: 20,
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
            base: 50,
            scaling: AbilityScaling::default(),
        },
        break_power: WeaponBreakPower {
            base_power: 5,
            scaling: AbilityScaling::default(),
        },
        guard: WeaponGuard {
            cut_rate: GuardCutRate {
                slash: 0.2,
                strike: 0.2,
                thrust: 0.2,
                impact: 0.2,
                magic: 0.5,
                fire: 0.3,
                lightning: 0.3,
                chaos: 0.3,
            },
            guard_strength: 10,
        },
    };
    let staff_skills: Vec<Arc<Art>> = arts
        .iter()
        .filter(|art| {
            art.art_type == ArtType::Skill
                && (art.usable_weapon == ArtUsableWeapon::All
                    || matches!(
                        &art.usable_weapon,
                        ArtUsableWeapon::Specific(kinds) if kinds.contains(&WeaponKind::Staff)
                    ))
        })
        .map(|art| Arc::clone(art))
        .collect::<Vec<Arc<Art>>>();
    let staff_sorceries = arts
        .iter()
        .filter(|art| art.art_type == ArtType::Sorcery)
        .map(|art| Arc::clone(art))
        .collect::<Vec<Arc<Art>>>();

    PlayerEquippedWeapons {
        weapons: vec![
            EquippedWeaponWithArts {
                weapon: straight_sword,
                skills: straight_sword_skills,
                sorceries: straight_sword_sorceries,
                battle_weapon_id: BattleWeaponId(0),
            },
            EquippedWeaponWithArts {
                weapon: staff,
                skills: staff_skills,
                sorceries: staff_sorceries,
                battle_weapon_id: BattleWeaponId(1),
            },
        ],
    }
}

// BattleCharacter用の武器リストを作成
fn create_battle_weapons() -> Vec<BattleWeapon> {
    let equipped = create_equipped_weapons_with_arts();
    equipped
        .weapons
        .into_iter()
        .map(|w| BattleWeapon {
            id: w.battle_weapon_id,
            weapon: w.weapon,
        })
        .collect()
}

#[derive(Clone)]
struct ActionProcess {
    action: Arc<Action>,
    next_step_index: usize,
}
impl ActionProcess {
    fn from(action: &Arc<Action>) -> Self {
        ActionProcess {
            action: Arc::clone(action),
            next_step_index: 0,
        }
    }

    fn is_finished(&self) -> bool {
        self.next_step_index >= self.action.steps.len()
    }

    fn current_step(&self) -> Option<&ActionStep> {
        if self.is_finished() {
            None
        } else {
            Some(&self.action.steps[self.next_step_index])
        }
    }

    fn next(&mut self) -> Option<&ActionStep> {
        self.next_step_index += 1;
        if self.is_finished() {
            None
        } else {
            let step = &self.action.steps[self.next_step_index];
            Some(step)
        }
    }
}

#[derive(Clone)]
struct Action {
    steps: Vec<ActionStep>,
}

#[derive(Clone, Copy)]
struct ActionStep {
    name: &'static str,
    specification: ActionStepSpecificationEnum,
}

#[derive(Clone, Copy)]
enum ActionStepSpecificationEnum {
    Attack(ActionStepSpecificationAttack),
    Wait(ActionStepSpecificationWait),
    Heal(ActionStepSpecificationHeal),
}
#[derive(Clone, Copy)]
struct ActionStepSpecificationAttack {
    power: f32,
}
#[derive(Clone, Copy)]
struct ActionStepSpecificationWait {
    invincible: bool,
}
#[derive(Clone, Copy)]
struct ActionStepSpecificationHeal {
    amount: i32,
}

fn create_enemy_attack() -> Action {
    Action {
        steps: vec![ActionStep {
            name: "爪攻撃",
            specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                power: 1.0,
            }),
        }],
    }
}
fn create_enemy_wait() -> Action {
    Action {
        steps: vec![ActionStep {
            name: "待機",
            specification: ActionStepSpecificationEnum::Wait(ActionStepSpecificationWait {
                invincible: false,
            }),
        }],
    }
}

/// 準備画面の設定から装備武器とアーツを作成する
fn create_equipped_weapons_from_preparation(
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
fn create_battle_from_preparation(
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
        stats: EnemyStats { hp: 1500, sp: 30 },
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

// 次ターンに表示される事前決定済み敵行動
#[derive(Resource)]
struct EnemyPlannedAction(Option<BattleConduct>);

// 行動選択メニューの状態
#[derive(Clone, PartialEq, Eq)]
enum ActionMenuState {
    ConsecutiveConfirm,                          // 連続コマンド確認画面
    ConsecutiveInput,                            // 連続コマンド入力中 - カテゴリ選択
    ConsecutiveBasicArts,                        // 連続コマンド入力中 - 基本アーツ選択
    ConsecutiveWeaponArts { weapon_idx: usize }, // 連続コマンド入力中 - 武器アーツ選択
}

#[derive(Clone)]
enum CommandSelectionState {
    Confirm,
    SelectCategory, // カテゴリ選択
    SelectBasicArt, // 基本アーツ選択
    // 武器アーツ選択
    SelectWeaponArt { weapon_idx: usize },
    // アーツ選択済み
    SelectedArt(SelectedArtEnum),
}
#[derive(Clone)]
enum SelectedArtEnum {
    Basic { art: Arc<Art> },
    Weapon { art: Arc<Art>, weapon_index: usize },
}

// 行動選択リソース
#[derive(Resource)]
struct ActionMenuSelection {
    menu_state: ActionMenuState,
    command_state: CommandSelectionState,
    // selected_art: Option<Arc<Art>>,
    // selected_weapon_index: Option<usize>,
}
impl Default for ActionMenuSelection {
    fn default() -> Self {
        ActionMenuSelection {
            menu_state: ActionMenuState::ConsecutiveInput,
            command_state: CommandSelectionState::SelectCategory,
        }
    }
}
impl ActionMenuSelection {
    // 確定選択
    fn confirm(&mut self) {
        self.menu_state = ActionMenuState::ConsecutiveConfirm;
        self.command_state = CommandSelectionState::Confirm;
    }
    // カテゴリ選択
    fn input(&mut self) {
        self.menu_state = ActionMenuState::ConsecutiveInput;
        self.command_state = CommandSelectionState::SelectCategory;
    }

    // 基本を選択
    fn select_category_basic(&mut self) {
        self.menu_state = ActionMenuState::ConsecutiveBasicArts;
        self.command_state = CommandSelectionState::SelectBasicArt;
    }
    // 基本アーツ選択
    fn select_basic_art(&mut self, art: Arc<Art>) {
        self.command_state = CommandSelectionState::SelectedArt(SelectedArtEnum::Basic { art });
    }

    // 武器を選択
    fn select_category_weapon(&mut self, weapon_idx: usize) {
        self.menu_state = ActionMenuState::ConsecutiveWeaponArts { weapon_idx };
        self.command_state = CommandSelectionState::SelectWeaponArt { weapon_idx };
    }
    // 武器アーツ選択
    fn select_weapon_art(&mut self, art: Arc<Art>, weapon_index: usize) {
        self.command_state =
            CommandSelectionState::SelectedArt(SelectedArtEnum::Weapon { art, weapon_index });
    }
}

// プレイヤーの基本アーツリスト
#[derive(Resource)]
struct PlayerBasicArts(Vec<Arc<Art>>);

// プレイヤーの装備武器とそれに対応するスキル
#[derive(Resource)]
struct PlayerEquippedWeapons {
    weapons: Vec<EquippedWeaponWithArts>,
}

#[derive(Clone)]
struct EquippedWeaponWithArts {
    weapon: Weapon,
    skills: Vec<Arc<Art>>,            // 技
    sorceries: Vec<Arc<Art>>,         // 術
    battle_weapon_id: BattleWeaponId, // 戦闘武器ID
}

// 選択されたアーツ（新システム）
#[derive(Resource, Default)]
struct SelectedArt {
    art: Option<Arc<Art>>,
    weapon_index: Option<usize>,
    battle_weapon_id: Option<BattleWeaponId>,
}

// 連続コマンド用リソース：最大3ターン分のアーツを保存
#[derive(Resource, Default)]
struct ConsecutiveCommands {
    commands: Vec<ConsecutiveCommandEntry>,
}

#[derive(Clone)]
struct ConsecutiveCommandEntry {
    art: Arc<Art>,
    weapon_index: Option<usize>,
    battle_weapon_id: Option<BattleWeaponId>,
}

// 戦闘画面全体のマーカー（クリーンアップ用）
#[derive(Component)]
struct BattleScreen;

// 戻るボタン
#[derive(Component)]
struct BackToPreparationButton;

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct UiStatus;

#[derive(Component)]
struct UiPhase;

#[derive(Component)]
struct UiLog;

// 有効値（コマンド別表示用）
#[derive(Component)]
struct UiEffAttack;
#[derive(Component)]
struct UiEffSkill;
#[derive(Component)]
struct UiEffHeal;
#[derive(Component)]
struct UiEffDefend;

//
#[derive(Component)]
struct UiBackground;

#[derive(Component)]
struct UiPlayerStatus;
#[derive(Component)]
struct UiHpText;
#[derive(Component)]
struct UiHpGaugeFill;
#[derive(Component)]
struct UiStaText;
#[derive(Component)]
struct UiStaGaugeFill;
#[derive(Component)]
struct UiSpText;
#[derive(Component)]
struct UiSpGaugeFill;
#[derive(Component)]
struct UiTranceText;
#[derive(Component)]
struct UiTranceGaugeFill;
#[derive(Component)]
struct UiTranceLevelText;
#[derive(Component)]
struct UiTranceEffectText;
#[derive(Component)]
struct UiKarmaCardsContainer;

#[derive(Component)]
struct UiEnemy;
#[derive(Component)]
struct UiEnemyStatus;

// UiEnemy 内部の更新ターゲット
#[derive(Component)]
struct UiEnemyHpGaugeFill;
#[derive(Component)]
struct UiEnemyBreakGaugeFill;
#[derive(Component)]
struct UiEnemyBreakLabel; // 「ブレイク中」表示用
#[derive(Component)]
struct UiEnemyNextActionText; // 「次の行動: ...」

// 敵ダメージ表示テキスト（HPゲージの横に一時表示）
#[derive(Component)]
struct UiEnemyDamageText;
#[derive(Component)]
struct UiMessage;

// 行動選択メニュー用コンポーネント
#[derive(Component)]
struct UiActionMenu;
#[derive(Component)]
struct UiActionMenuContainer;
// メニューアイテム（ボタン）
#[derive(Component)]
struct ActionMenuItem {
    item_type: ActionMenuItemType,
}
#[derive(Clone)]
enum ActionMenuItemType {
    Category(ActionMenuCategory),
    Art(Arc<Art>),
    ConsecutiveAction(ConsecutiveActionType), // 連続コマンド関連のアクション
}
#[derive(Clone, PartialEq, Eq)]
enum ActionMenuCategory {
    Basic,
    Weapon(usize), // 武器インデックス
    Back,          // 戻る
}
// 連続コマンド確認画面の選択肢
#[derive(Clone, PartialEq, Eq)]
enum ConsecutiveActionType {
    Execute,     // 連続コマンドを実行
    Reenter,     // コマンド入力しなおし
    FinishInput, // 入力完了（1〜2ターン分で終了）
}

#[derive(Component)]
struct UiCommand;
#[derive(Component)]
struct UiCommandHelp;

// ================== Boss Slain Banner ==================
#[derive(Component)]
struct BossSlainText; // ボス撃破表示用

#[derive(Component)]
struct BossSlainBanner {
    elapsed: f32,
    phase: BannerPhase,
}

// バナー背面の黒帯（グラデーション）
#[derive(Component)]
struct BossSlainBackdrop;
#[derive(Component)]
struct BossSlainBackdropCenter; // 中央の帯（不透明）
#[derive(Component)]
struct BossSlainBackdropRow(u8); // グラデーション行（0=最上段）

enum BannerPhase {
    FadeIn,
    Hold,
    FadeOut,
}

#[derive(Resource)]
struct BattleResource(Battle);

// 戦闘画面のセットアップ
fn setup_battle_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    prep_state: Res<PreparationState>,
    equipment_db: Res<EquipmentDatabase>,
    arts_db: Res<ArtsDatabase>,
) {
    commands.insert_resource(BattlePhase::DecideEnemyConduct);
    commands.insert_resource(Turn(1));
    // 初期ログと敵行動決定
    let mut rng = rand::rng();
    let attack = Arc::new(create_enemy_attack());
    let wait = Arc::new(create_enemy_wait());
    let first_action = if rng.random_bool(0.5) {
        ActionProcess::from(&attack)
    } else {
        ActionProcess::from(&wait)
    };
    commands.insert_resource(CombatLog(vec![
        format!("初期敵行動: {}", first_action.current_step().unwrap().name),
        "行動を選択してください".to_string(),
    ]));
    commands.insert_resource(SelectedArt::default());
    commands.insert_resource(EnemyPlannedAction(None));
    commands.insert_resource(ConsecutiveBatch::default());
    commands.insert_resource(EnemyDamagePopup::default());
    commands.insert_resource(ConsecutiveCommands::default());
    commands.insert_resource(KarmaCardsNeedsRedraw(true)); // 初回描画用フラグ
    // プレイヤー行動定義をリソースとして挿入
    // 基本アーツと武器データをリソースとして挿入（準備画面の設定を反映）
    let (basic_arts, equipped_weapons, battle_weapons) =
        create_equipped_weapons_from_preparation(&prep_state, &equipment_db, &arts_db);
    commands.insert_resource(PlayerBasicArts(basic_arts));
    commands.insert_resource(equipped_weapons);
    commands.insert_resource(ActionMenuSelection::default());
    // Battleモジュールの戦闘データを初期化（準備画面の設定を反映）
    commands.insert_resource(BattleResource(create_battle_from_preparation(
        &prep_state,
        &equipment_db,
        battle_weapons,
    )));

    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    // 画面下のログメッセージ（白枠、最大10行）
    commands
        .spawn((
            BattleScreen,
            Node {
                width: Val::Px(750.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                right: Val::Px(12.0),
                bottom: Val::Px(12.0),
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::from(LinearRgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.6,
            })),
            BorderColor::all(Color::WHITE),
            ZIndex(10),
        ))
        .with_children(|col| {
            col.spawn((
                UiMessage,
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

    // 敵UI（中央配置）: 画像の上にHP/ブレイクゲージと次の行動を表示
    let dragon = asset_server.load("images/dragon.png");
    commands
        .spawn((
            BattleScreen,
            UiEnemy,
            Node {
                width: percent(100),
                height: percent(100),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ZIndex(0),
        ))
        .with_children(|center| {
            // 画像コンテナ（相対位置指定にしてオーバーレイをAbsoluteで配置）
            center
                .spawn((
                    Node {
                        width: Val::Px(512.0),
                        height: Val::Px(384.0),
                        position_type: PositionType::Relative,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    ImageNode::new(dragon.clone()),
                ))
                .with_children(|over| {
                    // オーバーレイ（画像の上側に配置）
                    over.spawn((Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(16.0),
                        right: Val::Auto,
                        top: Val::Px(12.0),
                        bottom: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..default()
                    },))
                        .with_children(|col| {
                            // HPゲージ行（ゲージ＋ダメージ表示）
                            col.spawn((Node {
                                width: Val::Auto,
                                height: Val::Auto,
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(12.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },))
                                .with_children(|row| {
                                    // HPゲージ
                                    row.spawn((
                                        Node {
                                            width: Val::Px(360.0),
                                            height: Val::Px(14.0),
                                            border: UiRect::all(Val::Px(1.0)),
                                            ..default()
                                        },
                                        BackgroundColor(Color::from(LinearRgba {
                                            red: 0.15,
                                            green: 0.15,
                                            blue: 0.15,
                                            alpha: 1.0,
                                        })),
                                        BorderColor::all(Color::WHITE),
                                    ))
                                    .with_children(|g| {
                                        g.spawn((
                                            UiEnemyHpGaugeFill,
                                            Node {
                                                width: percent(0),
                                                height: percent(100),
                                                ..default()
                                            },
                                            BackgroundColor(Color::from(LinearRgba {
                                                red: 0.80,
                                                green: 0.20,
                                                blue: 0.20,
                                                alpha: 1.0,
                                            })),
                                        ));
                                    });

                                    // ダメージ表示テキスト（初期は非表示）
                                    row.spawn((
                                        UiEnemyDamageText,
                                        Text::new(""),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::from(LinearRgba {
                                            red: 0.95,
                                            green: 0.85,
                                            blue: 0.35,
                                            alpha: 1.0,
                                        })),
                                        Visibility::Hidden,
                                    ));
                                });

                            // ブレイク行（ゲージ＋「ブレイク中」ラベル）
                            col.spawn((Node {
                                width: Val::Auto,
                                height: Val::Auto,
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(8.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },))
                                .with_children(|row| {
                                    // ブレイクゲージ
                                    row.spawn((
                                        Node {
                                            width: Val::Px(360.0),
                                            height: Val::Px(10.0),
                                            border: UiRect::all(Val::Px(1.0)),
                                            ..default()
                                        },
                                        BackgroundColor(Color::from(LinearRgba {
                                            red: 0.15,
                                            green: 0.15,
                                            blue: 0.15,
                                            alpha: 1.0,
                                        })),
                                        BorderColor::all(Color::WHITE),
                                    ))
                                    .with_children(|g| {
                                        g.spawn((
                                            UiEnemyBreakGaugeFill,
                                            Node {
                                                width: percent(0),
                                                height: percent(100),
                                                ..default()
                                            },
                                            BackgroundColor(Color::from(LinearRgba {
                                                red: 0.25,
                                                green: 0.55,
                                                blue: 0.95,
                                                alpha: 1.0,
                                            })),
                                        ));
                                    });

                                    // ブレイク中ラベル（初期は非表示）
                                    row.spawn((
                                        UiEnemyBreakLabel,
                                        Text::new("ブレイク中"),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        Visibility::Hidden,
                                    ));
                                });

                            // 次の行動
                            col.spawn((
                                UiEnemyNextActionText,
                                Text::new(""),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
    // 右上にプレイヤーステータス枠（HP/スタミナの文字とゲージ）
    commands
        .spawn((
            BattleScreen,
            UiPlayerStatus,
            Node {
                width: Val::Px(280.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(8.0)),
                position_type: PositionType::Absolute,
                left: Val::Px(16.0),
                top: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            BorderColor::all(Color::WHITE),
        ))
        .with_children(|col| {
            // HP表示テキスト
            col.spawn((
                UiHpText,
                Text::new("HP: --- / ---"),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            // HPゲージ（枠）
            col.spawn((
                Node {
                    width: percent(100),
                    height: Val::Px(12.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::from(LinearRgba {
                    red: 0.15,
                    green: 0.15,
                    blue: 0.15,
                    alpha: 1.0,
                })),
                BorderColor::all(Color::WHITE),
            ))
            .with_children(|g| {
                g.spawn((
                    UiHpGaugeFill,
                    Node {
                        width: percent(0),
                        height: percent(100),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.80,
                        green: 0.20,
                        blue: 0.20,
                        alpha: 1.0,
                    })),
                ));
            });

            // スタミナ表示テキスト
            col.spawn((
                UiStaText,
                Text::new("スタミナ: --- / ---"),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            // スタミナゲージ（枠）
            col.spawn((
                Node {
                    width: percent(100),
                    height: Val::Px(12.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::from(LinearRgba {
                    red: 0.15,
                    green: 0.15,
                    blue: 0.15,
                    alpha: 1.0,
                })),
                BorderColor::all(Color::WHITE),
            ))
            .with_children(|g| {
                g.spawn((
                    UiStaGaugeFill,
                    Node {
                        width: percent(0),
                        height: percent(100),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.20,
                        green: 0.70,
                        blue: 0.25,
                        alpha: 1.0,
                    })),
                ));
            });

            // SP表示テキスト
            col.spawn((
                UiSpText,
                Text::new("SP: --- / ---"),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::from(LinearRgba {
                    red: 0.40,
                    green: 0.60,
                    blue: 1.00,
                    alpha: 1.0,
                })),
            ));
            // SPゲージ（枠）
            col.spawn((
                Node {
                    width: percent(100),
                    height: Val::Px(12.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::from(LinearRgba {
                    red: 0.15,
                    green: 0.15,
                    blue: 0.15,
                    alpha: 1.0,
                })),
                BorderColor::all(Color::from(LinearRgba {
                    red: 0.40,
                    green: 0.60,
                    blue: 1.00,
                    alpha: 1.0,
                })),
            ))
            .with_children(|g| {
                g.spawn((
                    UiSpGaugeFill,
                    Node {
                        width: percent(0),
                        height: percent(100),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.30,
                        green: 0.50,
                        blue: 0.90,
                        alpha: 1.0,
                    })),
                ));
            });

            // トランス表示テキスト（トランス値とレベル）
            col.spawn((Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },))
                .with_children(|row| {
                    row.spawn((
                        UiTranceText,
                        Text::new("トランス: --- / ---"),
                        TextFont {
                            font: font.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::from(LinearRgba {
                            red: 0.90,
                            green: 0.60,
                            blue: 0.90,
                            alpha: 1.0,
                        })),
                    ));
                    row.spawn((
                        UiTranceLevelText,
                        Text::new("Lv.0"),
                        TextFont {
                            font: font.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::from(LinearRgba {
                            red: 1.0,
                            green: 0.85,
                            blue: 0.30,
                            alpha: 1.0,
                        })),
                    ));
                });
            // トランスゲージ（枠）
            col.spawn((
                Node {
                    width: percent(100),
                    height: Val::Px(12.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::from(LinearRgba {
                    red: 0.15,
                    green: 0.15,
                    blue: 0.15,
                    alpha: 1.0,
                })),
                BorderColor::all(Color::from(LinearRgba {
                    red: 0.80,
                    green: 0.50,
                    blue: 0.80,
                    alpha: 1.0,
                })),
            ))
            .with_children(|g| {
                g.spawn((
                    UiTranceGaugeFill,
                    Node {
                        width: percent(0),
                        height: percent(100),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.75,
                        green: 0.30,
                        blue: 0.85,
                        alpha: 1.0,
                    })),
                ));
            });

            // トランス効果表示テキスト
            col.spawn((
                UiTranceEffectText,
                Text::new("効果: なし"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::from(LinearRgba {
                    red: 0.70,
                    green: 0.70,
                    blue: 0.90,
                    alpha: 1.0,
                })),
            ));

            // カルマカードセクションタイトル
            col.spawn((
                Node {
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
                Text::new("[フィールドカルマ]"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::from(LinearRgba {
                    red: 0.95,
                    green: 0.80,
                    blue: 0.40,
                    alpha: 1.0,
                })),
            ));

            // カルマカードコンテナ（動的に中身が変わる）
            col.spawn((
                UiKarmaCardsContainer,
                Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    margin: UiRect::top(Val::Px(4.0)),
                    ..default()
                },
            ));
        });

    // 行動選択メニュー（クリック可能なボタン式）
    commands
        .spawn((
            BattleScreen,
            UiActionMenu,
            Node {
                width: Val::Px(300.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                bottom: Val::Px(16.0),
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Color::from(LinearRgba {
                red: 0.0,
                green: 0.0,
                blue: 0.1,
                alpha: 0.9,
            })),
            BorderColor::all(Color::WHITE),
            Visibility::Hidden, // 初期は非表示（AwaitCommandで表示）
            ZIndex(10),
        ))
        .with_children(|menu| {
            // メニュータイトル
            menu.spawn((
                Text::new("[行動選択]"),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            // メニューコンテナ（動的に中身が変わる）
            menu.spawn((
                UiActionMenuContainer,
                Node {
                    width: percent(100),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
            ));
        });

    // 戻るボタン（右上）
    commands
        .spawn((
            BattleScreen,
            BackToPreparationButton,
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(36.0),
                position_type: PositionType::Absolute,
                right: Val::Px(16.0),
                top: Val::Px(16.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::from(LinearRgba {
                red: 0.3,
                green: 0.2,
                blue: 0.2,
                alpha: 0.9,
            })),
            BorderColor::all(Color::WHITE),
            ZIndex(20),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new("準備画面に戻る"),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

    println!("ゲーム開始: 行動をクリックして選択してください");
}

// ================== Cleanup ==================
/// 戦闘画面のクリーンアップ
fn cleanup_battle_screen(mut commands: Commands, query: Query<Entity, With<BattleScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// ================== Back Button ==================
/// 戻るボタンのインタラクション
fn back_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackToPreparationButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Preparation);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.45,
                    green: 0.3,
                    blue: 0.3,
                    alpha: 0.9,
                }));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.3,
                    green: 0.2,
                    blue: 0.2,
                    alpha: 0.9,
                }));
            }
        }
    }
}

// ================== Input & Battle Resolution ==================
fn player_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut phase: ResMut<BattlePhase>,
    mut turn: ResMut<Turn>,
    mut log: ResMut<CombatLog>,
    mut planned: ResMut<EnemyPlannedAction>,
    mut batch: ResMut<ConsecutiveBatch>,
    mut enemy_damage_popup: ResMut<EnemyDamagePopup>,
    mut selected_art: ResMut<SelectedArt>,
    mut action_menu: ResMut<ActionMenuSelection>,
    mut consecutive: ResMut<ConsecutiveCommands>,
    // Battleモジュール
    mut battle_resource: ResMut<BattleResource>,
    equipped_weapons: Res<PlayerEquippedWeapons>,
) {
    let battle = &mut battle_resource.0;

    match *phase {
        BattlePhase::DecideEnemyConduct => {
            // カルマドロー
            battle.karma_draw_card();
            // TODO: インシデント

            // 敵の行動決定
            let enemy_id = battle.enemies.first().map(|e| e.character_id).unwrap_or(2);
            planned.0 = Some(battle.decide_enemy_conduct(DecideEnemyConductRequest {
                enemy_character_id: enemy_id,
            }));

            // 連続コマンドが設定されている場合は確認画面へ
            if !consecutive.commands.is_empty() {
                action_menu.confirm();
                *phase = BattlePhase::ConfirmQueued;
            } else {
                action_menu.input();

                *phase = BattlePhase::AwaitCommand;
            }
        }
        BattlePhase::AwaitCommand => {
            // クリックでアーツが選択されたら処理
            if let CommandSelectionState::SelectedArt(selected) = &action_menu.command_state {
                let (art, weapon_index) = match selected {
                    SelectedArtEnum::Basic { art } => {
                        // 基本アーツ選択時
                        (Arc::clone(&art), None)
                    }
                    SelectedArtEnum::Weapon { art, weapon_index } => {
                        // 武器アーツ選択時
                        (Arc::clone(&art), Some(*weapon_index))
                    }
                };
                let weapon_id = if let Some(weapon_idx) = weapon_index {
                    equipped_weapons
                        .weapons
                        .get(weapon_idx)
                        .map(|w| w.battle_weapon_id.clone())
                } else {
                    None
                };

                let input_turn = consecutive.commands.len() + 1;
                // 連続コマンドに追加
                consecutive.commands.push(ConsecutiveCommandEntry {
                    art: Arc::clone(&art),
                    weapon_index,
                    battle_weapon_id: weapon_id,
                });
                log.0
                    .push(format!("{}ターン目: {}を設定", input_turn, art.name));

                // 次のターンへ、または確定
                if input_turn < 3 {
                    action_menu.input();
                } else if let Some(cmd) = consecutive.commands.first().cloned() {
                    // 3ターン分入力完了、実行開始
                    log.0.push("連続コマンド入力完了！ 実行します".to_string());

                    // コマンド実行
                    execute_consecutive_command(cmd, &mut selected_art, battle, false, &mut log);

                    *phase = BattlePhase::InBattle;

                    consecutive.commands.remove(0);

                    action_menu.input();
                }
            }
        }
        BattlePhase::ConfirmQueued => {
            // 連続コマンド確認画面での選択処理
            // action_menu_click_systemで処理されるので、ここでは待機
            // 選択されたらInBattleまたはAwaitCommandに遷移
        }
        BattlePhase::InBattle => {
            // 選択されたアーツを取得
            let art = if let Some(art) = selected_art.art.take() {
                art
            } else {
                // アーツが無い場合は待機に戻る
                *phase = BattlePhase::AwaitCommand;
                return;
            };

            // 選択された武器IDを取得
            let weapon_id = selected_art.battle_weapon_id.take();

            log.0
                .push(format!("ターン {} プレイヤーは{}を選択", turn.0, art.name));

            // Battleモジュールで行動実行
            let player_id = battle.player.character_id;
            let enemy_id = battle.enemies.first().map(|e| e.character_id).unwrap_or(2);

            // アーツの効果タイプに応じてターゲットを決定
            let target = match &art.rank1.potency {
                ArtPotency::Attack(_) => BattleConductTargetType::EnemySingle(enemy_id),
                ArtPotency::Support(_) => BattleConductTargetType::Player,
            };

            let player_conduct = BattleConduct {
                actor_character_id: player_id,
                target,
                art: Arc::clone(&art),
                battle_weapon_id: weapon_id,
            };

            let enemy_conduct = planned.0.clone().expect("敵の行動が未定");
            planned.0 = None; // 予定は消費

            // 行動順決定
            let order = battle.decide_order(BattleDecideOrderRequest {
                conducts: vec![&player_conduct, &enemy_conduct],
            });

            let mut player_dealt_damage_hp: u32 = 0;
            // 行動実行
            for actor_id in order {
                let conduct_to_execute = if actor_id == player_id {
                    player_conduct.clone()
                } else {
                    enemy_conduct.clone()
                };
                let incident = battle.execute_conduct(BattleExecuteConductRequest {
                    conduct: conduct_to_execute,
                });

                match incident.outcome {
                    BattleIncidentConductOutcome::Failure(failure) => {
                        log.0.push(format!("{}は不発", incident.conduct.art.name));
                    }
                    BattleIncidentConductOutcome::Success(s) => {
                        for character_incident in s.attacker.incidents.iter() {
                            for incident_concrete in character_incident.concretes.iter() {
                                match incident_concrete {
                                    BattleCharacterIncidentConcrete::CombinationSkillActivated(
                                        c,
                                    ) => {
                                        log.0.push(format!(
                                            "コンビネーション技 {} 発動！",
                                            c.combination_skill_name
                                        ));
                                    }
                                    BattleCharacterIncidentConcrete::DamageSp(d) => log.0.push(
                                        format!("SP -{} ({} → {})", d.damage, d.before, d.after),
                                    ),
                                    BattleCharacterIncidentConcrete::DamageStamina(d) => {
                                        log.0.push(format!(
                                            "Stamina -{} ({} → {})",
                                            d.damage, d.before, d.after
                                        ))
                                    }
                                    _ => {}
                                }
                            }
                        }
                        for def in s.defenders.iter() {
                            if def.is_evaded {
                                log.0.push("回避した".to_string());
                            }
                            if def.is_defended {
                                log.0.push("防御した".to_string());
                            }

                            for character_incident in def.character.incidents.iter() {
                                for incident_concrete in character_incident.concretes.iter() {
                                    match incident_concrete {
                                        BattleCharacterIncidentConcrete::DamageHp(d) => {
                                            if def.character.character_id == player_id {
                                                player_dealt_damage_hp = d.damage;
                                            }
                                            log.0.push(format!(
                                                "{} に{}ダメージ (HP {} → {})",
                                                if def.character.character_id == enemy_id {
                                                    "敵"
                                                } else {
                                                    "プレイヤー"
                                                },
                                                d.damage,
                                                d.before,
                                                d.after
                                            ));
                                            // 敵へのダメージをポップアップに反映
                                            if def.character.character_id == enemy_id {
                                                enemy_damage_popup.amount = d.damage as i32;
                                                enemy_damage_popup.timer = 1.0;
                                            }
                                        }
                                        BattleCharacterIncidentConcrete::RecoverHp(r) => {
                                            log.0.push(format!(
                                                "{} のHPを{}回復 ({} → {})",
                                                if def.character.character_id == player_id {
                                                    "プレイヤー"
                                                } else {
                                                    "敵"
                                                },
                                                r.recover,
                                                r.before,
                                                r.after
                                            ))
                                        }
                                        BattleCharacterIncidentConcrete::RecoverStamina(r) => {
                                            log.0.push(format!(
                                                "{} のスタミナを{}回復 ({} → {})",
                                                if def.character.character_id == player_id {
                                                    "プレイヤー"
                                                } else {
                                                    "敵"
                                                },
                                                r.recover,
                                                r.before,
                                                r.after
                                            ))
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ターン終了
            turn.0 += 1;
            *phase = BattlePhase::TurnEnd;
        }
        BattlePhase::TurnEnd => {
            battle.turn_end();

            *phase = BattlePhase::DecideEnemyConduct;
        }
        BattlePhase::Finished => {
            return;
        }
    }
}

// 連続コマンド実行ヘルパー
fn execute_consecutive_command(
    cmd: ConsecutiveCommandEntry,
    selected_art: &mut SelectedArt,
    battle: &mut Battle,
    is_combination: bool, // コンビネーション発動
    log: &mut ResMut<CombatLog>,
) {
    // プレイヤーの現行行動ログを初期化
    battle.player.initialize_current_conduct_log();

    if is_combination {
        // プレイヤーのコンビネーション発動
        let stamina_cost = cmd.art.stamina_cost;
        let incident_character = battle.player.combination(stamina_cost);

        log.0.push("コンビネーション発動！".to_string());
        for character_incident in incident_character.incidents.iter() {
            for incident_concrete in character_incident.concretes.iter() {
                match incident_concrete {
                    BattleCharacterIncidentConcrete::TranceIncrease(t) => {
                        log.0.push(format!(
                            "トランス値 +{} ({} → {})",
                            t.increase, t.before, t.after
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    selected_art.art = Some(cmd.art);
    selected_art.weapon_index = cmd.weapon_index;
    selected_art.battle_weapon_id = cmd.battle_weapon_id;
}

// ================== Action Menu Click System ==================
fn action_menu_click_system(
    mut phase: ResMut<BattlePhase>,
    mut action_menu: ResMut<ActionMenuSelection>,
    mut selected_art: ResMut<SelectedArt>,
    mut consecutive: ResMut<ConsecutiveCommands>,
    mut log: ResMut<CombatLog>,
    basic_arts: Res<PlayerBasicArts>,
    equipped_weapons: Res<PlayerEquippedWeapons>,
    mut interaction_query: Query<
        (&Interaction, &ActionMenuItem),
        (Changed<Interaction>, With<Button>),
    >,
    mut battle_resource: ResMut<BattleResource>,
) {
    // AwaitCommand または ConfirmQueued フェーズで処理
    if *phase != BattlePhase::AwaitCommand && *phase != BattlePhase::ConfirmQueued {
        return;
    }

    let battle = &mut battle_resource.0;

    for (interaction, menu_item) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match &menu_item.item_type {
                ActionMenuItemType::Category(category) => match category {
                    ActionMenuCategory::Basic => {
                        // 基本を選択時
                        action_menu.select_category_basic();
                    }
                    ActionMenuCategory::Weapon(idx) => {
                        // 武器を選択時
                        action_menu.select_category_weapon(*idx);
                    }
                    ActionMenuCategory::Back => {
                        // 戻るを選択時

                        if let ActionMenuState::ConsecutiveInput = action_menu.menu_state {
                            // コマンド取り消し時
                            if !consecutive.commands.is_empty() {
                                consecutive.commands.pop();
                            }
                        }
                        action_menu.menu_state = ActionMenuState::ConsecutiveInput;
                    }
                },
                ActionMenuItemType::Art(art) => {
                    if let CommandSelectionState::SelectWeaponArt { weapon_idx } =
                        action_menu.command_state.clone()
                    {
                        // 武器アーツ選択時
                        action_menu.select_weapon_art(Arc::clone(art), weapon_idx);
                    } else if let CommandSelectionState::SelectBasicArt = action_menu.command_state
                    {
                        // 基本アーツ選択時
                        action_menu.select_basic_art(Arc::clone(art));
                    }
                }
                ActionMenuItemType::ConsecutiveAction(action_type) => {
                    match action_type {
                        ConsecutiveActionType::Execute => {
                            // 連続コマンドを実行
                            if let Some(cmd) = consecutive.commands.first().cloned() {
                                log.0.push(format!("連続コマンド実行: {}", cmd.art.name));

                                // コマンド実行
                                execute_consecutive_command(
                                    cmd,
                                    &mut selected_art,
                                    battle,
                                    true,
                                    &mut log,
                                );

                                consecutive.commands.remove(0);

                                *phase = BattlePhase::InBattle;
                            }
                        }
                        ConsecutiveActionType::Reenter => {
                            // コマンド入力しなおし
                            consecutive.commands.clear();
                            action_menu.input();
                            log.0.push("連続コマンドを破棄しました".to_string());

                            *phase = BattlePhase::AwaitCommand;
                        }
                        ConsecutiveActionType::FinishInput => {
                            // 入力完了（1〜2ターン分で終了）
                            if !consecutive.commands.is_empty() {
                                if let Some(cmd) = consecutive.commands.first().cloned() {
                                    let count = consecutive.commands.len();
                                    log.0
                                        .push(format!("連続コマンド入力完了（{}ターン分）", count));

                                    // 最初のコマンドを実行
                                    execute_consecutive_command(
                                        cmd,
                                        &mut selected_art,
                                        battle,
                                        false,
                                        &mut log,
                                    );

                                    consecutive.commands.remove(0);

                                    *phase = BattlePhase::InBattle;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ================== Action Menu Update System ==================
fn action_menu_update_system(
    phase: Res<BattlePhase>,
    action_menu: Res<ActionMenuSelection>,
    consecutive: Res<ConsecutiveCommands>,
    basic_arts: Res<PlayerBasicArts>,
    equipped_weapons: Res<PlayerEquippedWeapons>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut menu_vis_q: Query<&mut Visibility, With<UiActionMenu>>,
    container_q: Query<Entity, With<UiActionMenuContainer>>,
    menu_items_q: Query<Entity, With<ActionMenuItem>>,
) {
    // メニューの表示/非表示 (AwaitCommand または ConfirmQueued で表示)
    if let Ok(mut vis) = menu_vis_q.single_mut() {
        *vis = if *phase == BattlePhase::AwaitCommand || *phase == BattlePhase::ConfirmQueued {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    // AwaitCommand または ConfirmQueued フェーズで処理
    if *phase != BattlePhase::AwaitCommand && *phase != BattlePhase::ConfirmQueued {
        return;
    }

    // メニューアイテムが存在するかチェック
    let has_menu_items = menu_items_q.iter().next().is_some();

    // リソースが変更されていない場合、かつメニューアイテムが既に存在する場合はスキップ
    if !action_menu.is_changed() && has_menu_items {
        return;
    }

    let Ok(container_entity) = container_q.single() else {
        return;
    };

    // 古いメニューアイテムを削除
    for entity in menu_items_q.iter() {
        commands.entity(entity).despawn();
    }

    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    // メニュー状態に応じてボタンを生成
    match &action_menu.menu_state {
        ActionMenuState::ConsecutiveConfirm => {
            // 連続コマンド確認画面
            commands.entity(container_entity).with_children(|parent| {
                // 設定済みコマンドの表示
                spawn_menu_label(parent, &font, "【設定済み連続コマンド】");
                for (i, cmd) in consecutive.commands.iter().enumerate() {
                    let label = format!("{}ターン目: {}", i + 1, cmd.art.name);
                    spawn_menu_label(parent, &font, &label);
                }

                // 選択肢
                spawn_menu_label(parent, &font, "");
                spawn_menu_button(
                    parent,
                    &font,
                    "▶ 連続コマンドを実行",
                    ActionMenuItemType::ConsecutiveAction(ConsecutiveActionType::Execute),
                );
                spawn_menu_button(
                    parent,
                    &font,
                    "✕ コマンド入力しなおし",
                    ActionMenuItemType::ConsecutiveAction(ConsecutiveActionType::Reenter),
                );
            });
        }
        ActionMenuState::ConsecutiveInput => {
            // 連続コマンド入力画面 - カテゴリ選択
            commands.entity(container_entity).with_children(|parent| {
                let turn = consecutive.commands.len() + 1;
                let title = format!("【連続コマンド入力 - {}ターン目】", turn);
                spawn_menu_label(parent, &font, &title);

                // 既に入力済みのコマンドを表示
                for (i, cmd) in consecutive.commands.iter().enumerate() {
                    let label = format!("  {}ターン目: {} ✓", i + 1, cmd.art.name);
                    spawn_menu_label(parent, &font, &label);
                }

                spawn_menu_label(parent, &font, "");

                // カテゴリ選択: 基本 + 武器
                spawn_menu_button(
                    parent,
                    &font,
                    "基本",
                    ActionMenuItemType::Category(ActionMenuCategory::Basic),
                );

                for (idx, weapon) in equipped_weapons.weapons.iter().enumerate() {
                    spawn_menu_button(
                        parent,
                        &font,
                        &weapon.weapon.name,
                        ActionMenuItemType::Category(ActionMenuCategory::Weapon(idx)),
                    );
                }

                // 1つ以上入力済みなら「入力完了」ボタンを表示
                if !consecutive.commands.is_empty() {
                    let finish_label =
                        format!("入力完了（{}ターン分）", consecutive.commands.len());
                    spawn_menu_button(
                        parent,
                        &font,
                        &finish_label,
                        ActionMenuItemType::ConsecutiveAction(ConsecutiveActionType::FinishInput),
                    );

                    // 戻るボタン
                    spawn_menu_button(
                        parent,
                        &font,
                        "前の行動を取り消す",
                        ActionMenuItemType::Category(ActionMenuCategory::Back),
                    );
                }
            });
        }
        ActionMenuState::ConsecutiveBasicArts => {
            // 連続コマンド入力画面 - 基本アーツ選択
            commands.entity(container_entity).with_children(|parent| {
                let turn = consecutive.commands.len() + 1;
                let title = format!("【連続コマンド - {}ターン目 - 基本】", turn);
                spawn_menu_label(parent, &font, &title);

                // 既に入力済みのコマンドを表示
                for (i, cmd) in consecutive.commands.iter().enumerate() {
                    let label = format!("  {}ターン目: {} ✓", i + 1, cmd.art.name);
                    spawn_menu_label(parent, &font, &label);
                }

                spawn_menu_label(parent, &font, "");

                // 戻るボタン
                spawn_menu_button(
                    parent,
                    &font,
                    "← 戻る",
                    ActionMenuItemType::Category(ActionMenuCategory::Back),
                );

                // 基本アーツ
                for art in basic_arts.0.iter() {
                    let label = format!("{} (ST{})", art.name, art.stamina_cost);
                    spawn_menu_button(
                        parent,
                        &font,
                        &label,
                        ActionMenuItemType::Art(Arc::clone(art)),
                    );
                }
            });
        }
        ActionMenuState::ConsecutiveWeaponArts { weapon_idx } => {
            // 連続コマンド入力画面 - 武器アーツ選択
            if let Some(weapon_data) = equipped_weapons.weapons.get(*weapon_idx) {
                commands.entity(container_entity).with_children(|parent| {
                    let turn = consecutive.commands.len() + 1;
                    let title = format!(
                        "【連続コマンド - {}ターン目 - {}】",
                        turn, weapon_data.weapon.name
                    );
                    spawn_menu_label(parent, &font, &title);

                    // 既に入力済みのコマンドを表示
                    for (i, cmd) in consecutive.commands.iter().enumerate() {
                        let label = format!("  {}ターン目: {} ✓", i + 1, cmd.art.name);
                        spawn_menu_label(parent, &font, &label);
                    }

                    spawn_menu_label(parent, &font, "");

                    // 戻るボタン
                    spawn_menu_button(
                        parent,
                        &font,
                        "← 戻る",
                        ActionMenuItemType::Category(ActionMenuCategory::Back),
                    );

                    // 技
                    if !weapon_data.skills.is_empty() {
                        spawn_menu_label(parent, &font, "【技】");
                        for art in weapon_data.skills.iter() {
                            let label = format!("{} (ST{})", art.name, art.stamina_cost);
                            spawn_menu_button(
                                parent,
                                &font,
                                &label,
                                ActionMenuItemType::Art(Arc::clone(art)),
                            );
                        }
                    }

                    // 術
                    if !weapon_data.sorceries.is_empty() {
                        spawn_menu_label(parent, &font, "【術】");
                        for art in weapon_data.sorceries.iter() {
                            let label =
                                format!("{} (SP{}/ST{})", art.name, art.sp_cost, art.stamina_cost);
                            spawn_menu_button(
                                parent,
                                &font,
                                &label,
                                ActionMenuItemType::Art(Arc::clone(art)),
                            );
                        }
                    }

                    // 技も術もない場合
                    if weapon_data.skills.is_empty() && weapon_data.sorceries.is_empty() {
                        spawn_menu_label(parent, &font, "(この武器には技・術がありません)");
                    }
                });
            }
        }
    }
}

// ヘルパー: メニューボタン生成
fn spawn_menu_button<'a>(
    parent: &mut ChildSpawnerCommands<'a>,
    font: &Handle<Font>,
    label: &str,
    item_type: ActionMenuItemType,
) {
    parent
        .spawn((
            ActionMenuItem { item_type },
            Button,
            Node {
                width: percent(100),
                height: Val::Px(32.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                margin: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::from(LinearRgba {
                red: 0.15,
                green: 0.15,
                blue: 0.25,
                alpha: 1.0,
            })),
            BorderColor::all(Color::from(LinearRgba {
                red: 0.4,
                green: 0.4,
                blue: 0.6,
                alpha: 1.0,
            })),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label.to_string()),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

// ヘルパー: メニューラベル生成
fn spawn_menu_label<'a>(parent: &mut ChildSpawnerCommands<'a>, font: &Handle<Font>, label: &str) {
    parent
        .spawn((
            ActionMenuItem {
                item_type: ActionMenuItemType::Category(ActionMenuCategory::Back),
            }, // ダミー
            Node {
                width: percent(100),
                height: Val::Px(24.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(4.0)),
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            },
        ))
        .with_children(|lbl| {
            lbl.spawn((
                Text::new(label.to_string()),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::from(LinearRgba {
                    red: 0.7,
                    green: 0.7,
                    blue: 0.9,
                    alpha: 1.0,
                })),
            ));
        });
}

// ================== End Check ==================
fn battle_end_check_system(
    mut phase: ResMut<BattlePhase>,
    mut log: ResMut<CombatLog>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut vis_params: ParamSet<(
        Query<&mut Visibility, With<UiEnemy>>,
        Query<&mut Visibility, With<UiEnemyBreakLabel>>,
    )>,
    mut ui_enemy_next_text_q: Query<&mut Text, With<UiEnemyNextActionText>>,
    mut gauge_params_end: ParamSet<(
        Query<&mut Node, With<UiEnemyHpGaugeFill>>,
        Query<&mut Node, With<UiEnemyBreakGaugeFill>>,
    )>,
    mut battle_resource: ResMut<BattleResource>,
) {
    if *phase == BattlePhase::Finished {
        return;
    }

    let battle = &mut battle_resource.0;
    // TODO: 仮
    let player = &battle.player;
    let player_hp = player.hp.current_hp;
    let enemy = battle.enemies.first().unwrap();
    let enemy_hp = enemy.hp.current_hp;

    if enemy_hp == 0 {
        *phase = BattlePhase::Finished;
        log.0.push("勝利! 敵を倒しました".to_string());

        // 敵UIを即時非表示（HP表示などは一瞬で消す）
        if let Ok(mut vis) = vis_params.p0().single_mut() {
            *vis = Visibility::Hidden;
        }
        if let Ok(mut br_vis) = vis_params.p1().single_mut() {
            *br_vis = Visibility::Hidden;
        }
        if let Ok(mut next_text) = ui_enemy_next_text_q.single_mut() {
            next_text.0 = String::new();
        }
        if let Ok(mut hp_node) = gauge_params_end.p0().single_mut() {
            hp_node.width = percent(0);
        }
        if let Ok(mut br_node) = gauge_params_end.p1().single_mut() {
            br_node.width = percent(0);
        }
        // 少し遅らせてからバナー表示（敵消失後に表示）
        let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");
        commands
            .spawn((
                BossSlainBanner {
                    elapsed: -0.3, // 0.3秒遅延してからフェードイン開始
                    phase: BannerPhase::FadeIn,
                },
                Node {
                    width: percent(100),
                    height: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ZIndex(100),
            ))
            .with_children(|builder| {
                // 背景の黒帯（左右いっぱい、上下グラデ）
                builder
                    .spawn((
                        BossSlainBackdrop,
                        Node {
                            width: percent(100),
                            height: Val::Auto,
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            right: Val::Px(0.0),
                            // 画面全高に広げ、中央帯＋上下グラデを内包
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(0.0),
                            ..default()
                        },
                        ZIndex(100),
                    ))
                    .with_children(|back| {
                        // 上グラデーション（薄→濃へ）
                        for i in (0..6u8).rev() {
                            let alpha = (i as f32) * 0.08; // 0.0, 0.08, ...
                            back.spawn((
                                BossSlainBackdropRow(i),
                                Node {
                                    width: percent(100),
                                    height: Val::Px(12.0),
                                    ..default()
                                },
                                BackgroundColor(Color::from(LinearRgba {
                                    red: 0.0,
                                    green: 0.0,
                                    blue: 0.0,
                                    alpha: 0.0, // フェーズで乗算する
                                })),
                            ));
                        }

                        // 中央帯（不透明に近い）
                        back.spawn((
                            BossSlainBackdropCenter,
                            Node {
                                width: percent(100),
                                height: Val::Px(140.0),
                                ..default()
                            },
                            BackgroundColor(Color::from(LinearRgba {
                                red: 0.0,
                                green: 0.0,
                                blue: 0.0,
                                alpha: 0.0, // フェーズで乗算する
                            })),
                        ));

                        // 下グラデーション（濃→薄へ）
                        for i in 0..6u8 {
                            let alpha = (i as f32) * 0.08; // 0.0, 0.08, ...
                            back.spawn((
                                BossSlainBackdropRow(10 + i),
                                Node {
                                    width: percent(100),
                                    height: Val::Px(12.0),
                                    ..default()
                                },
                                BackgroundColor(Color::from(LinearRgba {
                                    red: 0.0,
                                    green: 0.0,
                                    blue: 0.0,
                                    alpha: 0.0, // フェーズで乗算する
                                })),
                            ));
                        }
                    });

                builder.spawn((
                    BossSlainText,
                    Text::new("DRAGON SLAIN"),
                    TextFont {
                        font: font.clone(),
                        font_size: 96.0,
                        ..default()
                    },
                    TextColor(Color::from(LinearRgba {
                        red: 0.83,
                        green: 0.72,
                        blue: 0.20,
                        alpha: 0.0,
                    })),
                    ZIndex(101),
                ));
            });
    } else if player_hp == 0 {
        *phase = BattlePhase::Finished;
        log.0.push("敗北... プレイヤーのHPが0です".to_string());
    }
}

fn ui_update_system(
    phase: Res<BattlePhase>,
    log: Res<CombatLog>,
    // mut ui_q: Query<&mut Children, With<UiRoot>>,
    planned: Res<EnemyPlannedAction>,
    battle_resource: Res<BattleResource>,
    mut ui_staus_q: Query<&mut Text, (With<UiStatus>, Without<UiPhase>, Without<UiLog>)>,
    // プレイヤーステータス（右上）の更新用: テキスト群（HP、スタミナ）
    // 右上プレイヤーステータスは別システムで更新（引数が多すぎるため分割）
    mut ui_eff_atk_q: Query<
        (&mut Text, &mut TextColor),
        (
            With<UiEffAttack>,
            Without<UiEffSkill>,
            Without<UiEffHeal>,
            Without<UiEffDefend>,
            Without<UiStatus>,
            Without<UiPhase>,
            Without<UiLog>,
        ),
    >,
    mut ui_eff_heal_q: Query<
        (&mut Text, &mut TextColor),
        (
            With<UiEffHeal>,
            Without<UiEffAttack>,
            Without<UiEffSkill>,
            Without<UiEffDefend>,
            Without<UiStatus>,
            Without<UiPhase>,
            Without<UiLog>,
        ),
    >,
    mut ui_eff_def_q: Query<
        (&mut Text, &mut TextColor),
        (
            With<UiEffDefend>,
            Without<UiEffAttack>,
            Without<UiEffSkill>,
            Without<UiEffHeal>,
            Without<UiStatus>,
            Without<UiPhase>,
            Without<UiLog>,
        ),
    >,
    mut ui_phase_q: Query<&mut Text, (With<UiPhase>, Without<UiStatus>, Without<UiLog>)>,
    mut ui_log_q: Query<&mut Text, (With<UiLog>, Without<UiStatus>, Without<UiPhase>)>,
) {
    let Ok(mut ui_status_text) = ui_staus_q.single_mut() else {
        return;
    };
    let Ok(mut ui_phase_text) = ui_phase_q.single_mut() else {
        return;
    };
    let Ok(mut ui_log_text) = ui_log_q.single_mut() else {
        return;
    };

    let battle = &battle_resource.0;

    let player = &battle.player;
    let p_hp = player.hp.current_hp;
    let p_stamina = player.stamina.current_stamina;
    let enemy = battle.enemies.first().unwrap();
    let e_hp = enemy.hp.current_hp;
    let e_break = enemy.status_ailment.breaking.accumulation;
    let e_break_max = enemy.status_ailment.breaking.max_accumulation;
    ui_status_text.0 = format!(
        "プレイヤーHP: {} / {}\nスタミナ: {} / {}\n100\n\n敵HP: {} / {}\n敵ブレイク値: {} / {}\n敵状態: {}\n\n",
        p_hp,
        player.hp.max_hp,
        p_stamina,
        player.stamina.max_stamina,
        e_hp,
        enemy.hp.max_hp,
        e_break,
        e_break_max,
        "通常" // TODO: 敵状態表示
               // if e_bstate.remaining_turns > 0 {
               //     "ブレイク中"
               // } else {
               //     "通常"
               // },
    );

    // 有効値（コマンド別）テキスト更新＆色切り替え
    let Ok((mut eff_atk_text, mut eff_atk_color)) = ui_eff_atk_q.single_mut() else {
        return;
    };
    eff_atk_color.0 = Color::WHITE;

    // 強攻撃の有効値表示は別システムで更新

    let Ok((mut eff_heal_text, mut eff_heal_color)) = ui_eff_heal_q.single_mut() else {
        return;
    };
    // eff_heal_text.0 = format!("回復 量:{} 消費:{}\n", heal_amount, heal_cost);
    eff_heal_color.0 = Color::WHITE;

    let Ok((mut eff_def_text, mut eff_def_color)) = ui_eff_def_q.single_mut() else {
        return;
    };
    // eff_def_text.0 = format!("防御 消費:{}\n\n", def_cost);
    eff_def_color.0 = Color::WHITE;

    let enemy_action_str = if let Some(conduct) = &planned.0 {
        conduct.art.name.to_string()
    } else {
        "不明".to_string()
    };

    let phase_str = match *phase {
        BattlePhase::DecideEnemyConduct => {
            format!("敵の行動決定中... 次の行動: {}", enemy_action_str)
        }
        BattlePhase::AwaitCommand => "行動を選択してください".to_string(),
        BattlePhase::ConfirmQueued => "連続コマンドを実行しますか？".to_string(),
        BattlePhase::InBattle => "処理中".to_string(),
        BattlePhase::TurnEnd => "ターン終了".to_string(),
        BattlePhase::Finished => "終了".to_string(),
    };
    ui_phase_text.0 = format!("フェーズ: {phase_str}\n\n");

    let mut log_text = String::from("ログ:\n");
    let log_max_lines = 30;
    let start = if log.0.len() > log_max_lines {
        log.0.len() - log_max_lines
    } else {
        0
    };
    for line in &log.0[start..] {
        log_text.push_str(line);
        log_text.push('\n');
    }
    ui_log_text.0 = log_text;
}

// コマンド入力表示（旧システム - 新UIメニューに置き換え済み）
fn ui_update_command_system(// 新しいUIシステムに置き換え済みのため、何もしない
) {
    // 古いUiCommandパネルは削除され、action_menu_update_system で処理される
}

// 画面下のUiMessageに最新メッセージを最大20行表示
fn ui_update_message_system(log: Res<CombatLog>, mut msg_q: Query<&mut Text, With<UiMessage>>) {
    let Ok(mut msg) = msg_q.single_mut() else {
        return;
    };
    let max_lines = 20usize;
    let start = if log.0.len() > max_lines {
        log.0.len() - max_lines
    } else {
        0
    };
    let mut s = String::new();
    for line in &log.0[start..] {
        s.push_str(line);
        s.push('\n');
    }
    msg.0 = s;
}

// 右上プレイヤーステータスの更新（HP/スタミナ/SP/トランス テキスト＆ゲージ）
fn ui_update_player_status_system(
    battle_resource: Res<BattleResource>,
    mut hp_text_q: Query<
        &mut Text,
        (
            With<UiHpText>,
            Without<UiStaText>,
            Without<UiSpText>,
            Without<UiTranceText>,
            Without<UiTranceLevelText>,
            Without<UiTranceEffectText>,
        ),
    >,
    mut sta_text_q: Query<
        &mut Text,
        (
            With<UiStaText>,
            Without<UiHpText>,
            Without<UiSpText>,
            Without<UiTranceText>,
            Without<UiTranceLevelText>,
            Without<UiTranceEffectText>,
        ),
    >,
    mut sp_text_q: Query<
        &mut Text,
        (
            With<UiSpText>,
            Without<UiHpText>,
            Without<UiStaText>,
            Without<UiTranceText>,
            Without<UiTranceLevelText>,
            Without<UiTranceEffectText>,
        ),
    >,
    mut trance_text_q: Query<
        &mut Text,
        (
            With<UiTranceText>,
            Without<UiHpText>,
            Without<UiStaText>,
            Without<UiSpText>,
            Without<UiTranceLevelText>,
            Without<UiTranceEffectText>,
        ),
    >,
    mut trance_level_text_q: Query<
        &mut Text,
        (
            With<UiTranceLevelText>,
            Without<UiHpText>,
            Without<UiStaText>,
            Without<UiSpText>,
            Without<UiTranceText>,
            Without<UiTranceEffectText>,
        ),
    >,
    mut trance_effect_text_q: Query<
        &mut Text,
        (
            With<UiTranceEffectText>,
            Without<UiHpText>,
            Without<UiStaText>,
            Without<UiSpText>,
            Without<UiTranceText>,
            Without<UiTranceLevelText>,
        ),
    >,
    mut gauge_params: ParamSet<(
        Query<&mut Node, With<UiHpGaugeFill>>,
        Query<&mut Node, With<UiStaGaugeFill>>,
        Query<&mut Node, With<UiSpGaugeFill>>,
        Query<&mut Node, With<UiTranceGaugeFill>>,
    )>,
) {
    let battle = &battle_resource.0;

    let player = &battle.player;
    let p_hp = player.hp.current_hp;
    let p_sta = player.stamina.current_stamina;
    let p_sp = player.sp.current_sp;

    // コンテナ内の最初のTextを簡潔表示用に更新
    if let Ok(mut hp_text) = hp_text_q.single_mut() {
        hp_text.0 = format!("HP: {} / {}", p_hp, player.hp.max_hp);
    }
    if let Ok(mut sta_text) = sta_text_q.single_mut() {
        sta_text.0 = format!("スタミナ: {} / {}", p_sta, player.stamina.max_stamina);
    }
    if let Ok(mut sp_text) = sp_text_q.single_mut() {
        sp_text.0 = format!("SP: {} / {}", p_sp, player.sp.max_sp);
    }

    // トランス表示更新
    if let Some(trance) = &player.trance {
        use crate::battle::BattleTranceController;
        let current_trance = trance.current_trance;
        let max_trance = trance.max_trance;
        let trance_level = trance.trance_level();
        let heart_effects = trance.current_heart_effects();

        if let Ok(mut trance_text) = trance_text_q.single_mut() {
            trance_text.0 = format!("トランス: {} / {}", current_trance, max_trance);
        }
        if let Ok(mut level_text) = trance_level_text_q.single_mut() {
            level_text.0 = format!("Lv.{}", trance_level);
        }
        if let Ok(mut effect_text) = trance_effect_text_q.single_mut() {
            if heart_effects.is_empty() {
                effect_text.0 = "効果: なし".to_string();
            } else {
                let effect_strs: Vec<String> = heart_effects
                    .iter()
                    .map(|e| format_heart_effect(e))
                    .collect();
                effect_text.0 = format!("効果: {}", effect_strs.join(", "));
            }
        }

        // トランスゲージ幅更新
        if let Ok(mut trance_node) = gauge_params.p3().single_mut() {
            let ratio = if max_trance > 0 {
                (current_trance as f32 / max_trance as f32).clamp(0.0, 1.0)
            } else {
                0.0
            };
            trance_node.width = percent((ratio * 100.0).round());
        }
    }

    // ゲージ幅更新
    if let Ok(mut hp_node) = gauge_params.p0().single_mut() {
        let ratio = if player.hp.max_hp > 0 {
            (p_hp as f32 / player.hp.max_hp as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        hp_node.width = percent((ratio * 100.0).round());
    }
    if let Ok(mut sta_node) = gauge_params.p1().single_mut() {
        let ratio = if player.stamina.max_stamina > 0 {
            (p_sta as f32 / player.stamina.max_stamina as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        sta_node.width = percent((ratio * 100.0).round());
    }
    // SPゲージ幅更新
    if let Ok(mut sp_node) = gauge_params.p2().single_mut() {
        let ratio = if player.sp.max_sp > 0 {
            (p_sp as f32 / player.sp.max_sp as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        sp_node.width = percent((ratio * 100.0).round());
    }
}

// HeartEffectを表示用文字列に変換するヘルパー関数
fn format_heart_effect(effect: &HeartEffect) -> String {
    match effect {
        HeartEffect::PhysicalDefenseModifier(m) => {
            format!("物防+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
        HeartEffect::MagicalDefenseModifier(m) => {
            format!("魔防+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
        HeartEffect::PhysicalAttackModifier(m) => {
            format!("物攻+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
        HeartEffect::MagicalAttackModifier(m) => format!("魔攻+{:.0}%", (m.modifier - 1.0) * 100.0),
        HeartEffect::StaminaRecoveryModifier(m) => {
            format!("スタミナ回復+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
        HeartEffect::AbilityIncrease(e) => {
            let ability_name = match e.ability_type {
                AbilityType::Strength => "筋力",
                AbilityType::Dexterity => "技量",
                AbilityType::Intelligence => "知力",
                AbilityType::Faith => "信仰",
                AbilityType::Arcane => "神秘",
                AbilityType::Agility => "敏捷性",
                AbilityType::Vitality => "生命力",
                AbilityType::Spirit => "精神力",
                AbilityType::Endurance => "持久力",
            };
            format!("{}+{}", ability_name, e.amount)
        }
    }
}

// KarmaEffectを表示用文字列に変換するヘルパー関数
fn format_karma_effect(effect: &KarmaEffect) -> String {
    match effect {
        KarmaEffect::AttackDamageModifier(m) => {
            format!("与ダメ+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
        KarmaEffect::ReceiveDamageModifier(m) => {
            let diff = (m.modifier - 1.0) * 100.0;
            if diff < 0.0 {
                format!("被ダメ{:.0}%", diff)
            } else {
                format!("被ダメ+{:.0}%", diff)
            }
        }
        KarmaEffect::AbilityIncrease(e) => {
            let ability_name = match e.ability_type {
                AbilityType::Strength => "筋力",
                AbilityType::Dexterity => "技量",
                AbilityType::Intelligence => "知力",
                AbilityType::Faith => "信仰",
                AbilityType::Arcane => "神秘",
                AbilityType::Agility => "敏捷性",
                AbilityType::Vitality => "生命力",
                AbilityType::Spirit => "精神力",
                AbilityType::Endurance => "持久力",
            };
            format!("{}+{}", ability_name, e.amount)
        }
    }
}

// カルマカードのUI更新システム
fn ui_update_karma_cards_system(
    mut commands: Commands,
    battle_resource: Res<BattleResource>,
    asset_server: Res<AssetServer>,
    container_q: Query<Entity, With<UiKarmaCardsContainer>>,
    children_q: Query<&Children>,
    mut redraw_flag: ResMut<KarmaCardsNeedsRedraw>,
) {
    // // 初回のみ描画（カルマカードの変更があった時は別途フラグを立てる）
    // if !redraw_flag.0 {
    //     return;
    // }
    // redraw_flag.0 = false;

    let battle = &battle_resource.0;
    let player = &battle.player;

    let Ok(container_entity) = container_q.single() else {
        return;
    };

    // 既存の子要素を削除
    if let Ok(children) = children_q.get(container_entity) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    let font: Handle<Font> = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    // カルマカードを表示
    if let Some(karma) = &player.karma {
        if karma.field_cards.is_empty() {
            // カードがない場合
            commands.entity(container_entity).with_children(|parent| {
                parent.spawn((
                    Text::new("なし"),
                    TextFont {
                        font: font.clone(),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::from(LinearRgba {
                        red: 0.6,
                        green: 0.6,
                        blue: 0.6,
                        alpha: 1.0,
                    })),
                ));
            });
        } else {
            // 各カードを表示
            for card in &karma.field_cards {
                let effect_strs: Vec<String> = card
                    .card
                    .effects
                    .iter()
                    .map(|e| format_karma_effect(e))
                    .collect();
                let effect_text = if effect_strs.is_empty() {
                    "効果なし".to_string()
                } else {
                    effect_strs.join(", ")
                };

                commands.entity(container_entity).with_children(|parent| {
                    // カード枠（横一列表示）
                    parent
                        .spawn((
                            Node {
                                width: percent(100),
                                padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(8.0),
                                ..default()
                            },
                            BackgroundColor(Color::from(LinearRgba {
                                red: 0.12,
                                green: 0.10,
                                blue: 0.18,
                                alpha: 1.0,
                            })),
                            BorderColor::all(Color::from(LinearRgba {
                                red: 0.70,
                                green: 0.55,
                                blue: 0.30,
                                alpha: 1.0,
                            })),
                        ))
                        .with_children(|card_box| {
                            // カード名
                            card_box.spawn((
                                Text::new(&card.card.name),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::from(LinearRgba {
                                    red: 1.0,
                                    green: 0.90,
                                    blue: 0.60,
                                    alpha: 1.0,
                                })),
                            ));
                            // 効果
                            card_box.spawn((
                                Node {
                                    flex_grow: 1.0,
                                    ..default()
                                },
                                Text::new(&effect_text),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 11.0,
                                    ..default()
                                },
                                TextColor(Color::from(LinearRgba {
                                    red: 0.80,
                                    green: 0.80,
                                    blue: 0.95,
                                    alpha: 1.0,
                                })),
                            ));
                            // 残りターン数
                            card_box.spawn((
                                Text::new(format!("{}T", card.remaining_turns)),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 11.0,
                                    ..default()
                                },
                                TextColor(Color::from(LinearRgba {
                                    red: 0.60,
                                    green: 0.85,
                                    blue: 0.60,
                                    alpha: 1.0,
                                })),
                            ));
                        });
                });
            }
        }
    } else {
        // karmaがない場合
        commands.entity(container_entity).with_children(|parent| {
            parent.spawn((
                Text::new("-"),
                TextFont {
                    font: font.clone(),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::from(LinearRgba {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.5,
                    alpha: 1.0,
                })),
            ));
        });
    }
}

// （演出簡易版につきフェード等の更新システムは未実装）
fn boss_slain_banner_system(
    time: Res<Time>,
    mut commands: Commands,
    mut banner_q: Query<(Entity, &mut BossSlainBanner, &Children)>,
    mut text_colors: Query<&mut TextColor, With<BossSlainText>>,
    mut backdrop_colors: ParamSet<(
        Query<&mut BackgroundColor, With<BossSlainBackdropRow>>,
        Query<&mut BackgroundColor, With<BossSlainBackdropCenter>>,
    )>,
) {
    const FADE_IN: f32 = 0.5;
    const HOLD: f32 = 3.0;
    const FADE_OUT: f32 = 1.0;

    for (entity, mut banner, children) in banner_q.iter_mut() {
        banner.elapsed += time.delta().as_secs_f32();
        match banner.phase {
            BannerPhase::FadeIn => {
                let phase_alpha = (banner.elapsed / FADE_IN).clamp(0.0, 1.0);
                for mut c in text_colors.iter_mut() {
                    c.0 = Color::from(LinearRgba {
                        red: 0.83,
                        green: 0.72,
                        blue: 0.20,
                        alpha: phase_alpha,
                    });
                }
                for mut bc in backdrop_colors.p1().iter_mut() {
                    bc.0 = Color::from(LinearRgba {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: phase_alpha,
                    });
                }
                for mut br in backdrop_colors.p0().iter_mut() {
                    br.0 = Color::from(LinearRgba {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: 0.9 * phase_alpha,
                    });
                }
                if banner.elapsed >= FADE_IN {
                    banner.phase = BannerPhase::Hold;
                    banner.elapsed = 0.0;
                }
            }
            BannerPhase::Hold => {
                for mut c in text_colors.iter_mut() {
                    c.0 = Color::from(LinearRgba {
                        red: 0.83,
                        green: 0.72,
                        blue: 0.20,
                        alpha: 1.0,
                    });
                }
                for mut bc in backdrop_colors.p1().iter_mut() {
                    bc.0 = Color::from(LinearRgba {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: 1.0,
                    });
                }
                for mut br in backdrop_colors.p0().iter_mut() {
                    br.0 = Color::from(LinearRgba {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: 0.9,
                    });
                }
                if banner.elapsed >= HOLD {
                    banner.phase = BannerPhase::FadeOut;
                    banner.elapsed = 0.0;
                }
            }
            BannerPhase::FadeOut => {
                let phase_alpha = 1.0 - (banner.elapsed / FADE_OUT).clamp(0.0, 1.0);
                for mut c in text_colors.iter_mut() {
                    c.0 = Color::from(LinearRgba {
                        red: 0.83,
                        green: 0.72,
                        blue: 0.20,
                        alpha: phase_alpha,
                    });
                }
                for mut bc in backdrop_colors.p1().iter_mut() {
                    bc.0 = Color::from(LinearRgba {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: phase_alpha,
                    });
                }
                for mut br in backdrop_colors.p0().iter_mut() {
                    br.0 = Color::from(LinearRgba {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: 0.9 * phase_alpha,
                    });
                }
                if banner.elapsed >= FADE_OUT {
                    // 完了後削除
                    for i in 0..children.len() {
                        let child = children[i];
                        commands.entity(child).despawn();
                    }
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

// 強攻撃の有効値表示（ガードカウンターの反映もここで実施）
fn ui_update_skill_effect_system(
    battle_resource: Res<BattleResource>,
    mut ui_eff_skl_q: Query<
        (&mut Text, &mut TextColor),
        (
            With<UiEffSkill>,
            Without<UiEffAttack>,
            Without<UiEffHeal>,
            Without<UiEffDefend>,
            Without<UiStatus>,
            Without<UiPhase>,
            Without<UiLog>,
        ),
    >,
) {
    let battle = &battle_resource.0;

    let skl_power = 25;
    let skl_cost = 25;

    let Ok((mut eff_skl_text, mut eff_skl_color)) = ui_eff_skl_q.single_mut() else {
        return;
    };
    let display_skl_power = skl_power;
    let display_break = 25;
    eff_skl_text.0 = format!(
        "強攻撃 威力:{} 消費:{} / ブレイク+{}\n",
        display_skl_power, skl_cost, display_break
    );
    eff_skl_color.0 = Color::WHITE;
}

// 敵UI（中央配置）の更新（HP/ブレイクのゲージ幅、ブレイク中表示、次の行動）
fn ui_update_enemy_system(
    battle_resource: Res<BattleResource>,
    planned: Res<EnemyPlannedAction>,
    mut gauge_params: ParamSet<(
        Query<&mut Node, With<UiEnemyHpGaugeFill>>,
        Query<&mut Node, With<UiEnemyBreakGaugeFill>>,
    )>,
    mut br_label_q: Query<&mut Visibility, With<UiEnemyBreakLabel>>,
    mut next_text_q: Query<&mut Text, With<UiEnemyNextActionText>>,
) {
    let battle = &battle_resource.0;

    let enemy = battle.enemies.first().unwrap();
    let e_hp = enemy.hp.current_hp;
    let e_break = enemy.status_ailment.breaking.accumulation;
    let e_break_max = enemy.status_ailment.breaking.max_accumulation;

    if let Ok(mut hp_node) = gauge_params.p0().single_mut() {
        let ratio = if enemy.hp.max_hp > 0 {
            (e_hp as f32 / enemy.hp.max_hp as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        hp_node.width = percent((ratio * 100.0).round());
    }
    if let Ok(mut br_node) = gauge_params.p1().single_mut() {
        let ratio = (e_break as f32 / e_break_max as f32).clamp(0.0, 1.0);
        br_node.width = percent((ratio * 100.0).round());
    }
    // TODO: ブレイク状態の表示何とかする
    // if let Ok(mut vis) = br_label_q.single_mut() {
    //     *vis = if e_bstate.remaining_turns > 0 {
    //         Visibility::Visible
    //     } else {
    //         Visibility::Hidden
    //     };
    // }
    if let Ok(mut t) = next_text_q.single_mut() {
        let enemy_action_str = if let Some(conduct) = &planned.0 {
            conduct.art.name.to_string()
        } else {
            "不明".to_string()
        };
        t.0 = format!("次の行動: {}", enemy_action_str);
    }
}

// 敵ダメージの一時表示更新（一定時間で非表示に戻す）
fn ui_update_enemy_damage_popup_system(
    time: Res<Time>,
    mut popup: ResMut<EnemyDamagePopup>,
    mut dmg_q: Query<(&mut Text, &mut Visibility), With<UiEnemyDamageText>>,
) {
    if let Ok((mut text, mut vis)) = dmg_q.single_mut() {
        if popup.timer > 0.0 {
            popup.timer -= time.delta_secs();
            *vis = Visibility::Visible;
            text.0 = format!("-{}", popup.amount);
        } else {
            *vis = Visibility::Hidden;
            text.0.clear();
        }
    }
}
