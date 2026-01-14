mod battle;
mod types;

use std::sync::Arc;

use crate::battle::{
    BattleDecideOrderRequest, BattleExecuteConductRequest, DecideEnemyConductRequest,
};
use crate::types::*;
use bevy::prelude::*;
use rand::Rng;

// 画面レイアウト切替用定数（false: 既存レイアウト / true: 新レイアウト）
const USE_DQ_LIKE_LAYOUT: bool = true;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_input_system)
        .add_systems(Update, battle_end_check_system)
        .add_systems(Update, ui_update_system)
        .add_systems(Update, ui_update_enemy_system)
        .add_systems(Update, ui_update_enemy_damage_popup_system)
        .add_systems(Update, ui_update_player_status_system)
        .add_systems(Update, ui_update_command_system)
        .add_systems(Update, ui_update_message_system)
        .add_systems(Update, ui_update_skill_effect_system)
        .add_systems(Update, boss_slain_banner_system)
        .run();
}

// ================== Components & Resources ==================
#[derive(Resource, PartialEq, Eq)]
enum BattlePhase {
    AwaitCommand,
    // 連続コマンドの次コマンドを実行するか確認するフェーズ
    ConfirmQueued,
    InBattle,
    Finished,
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

// プレイヤーの行動定義（リソース）
#[derive(Resource)]
struct PlayerConducts {
    attack: Arc<Conduct>,
    skill: Arc<Conduct>,
    heal: Arc<Conduct>,
    defend: Arc<Conduct>,
    wait: Arc<Conduct>,
}

fn create_default_player_conducts() -> PlayerConducts {
    PlayerConducts {
        attack: Arc::new(Conduct {
            name: "攻撃".to_string(),
            sp_cost: 0,
            stamina_cost: 5,
            perks: vec![ConductPerk::Melee],
            requirement: ConductRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            conduct_type: ConductType::Basic(ConductTypeBasic::Attack(ConductTypeBasicAttack {
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
                break_power: 10,
            })),
        }),
        skill: Arc::new(Conduct {
            name: "強攻撃".to_string(),
            sp_cost: 0,
            stamina_cost: 25,
            perks: vec![ConductPerk::Melee],
            requirement: ConductRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            conduct_type: ConductType::Basic(ConductTypeBasic::Attack(ConductTypeBasicAttack {
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
                break_power: 20,
            })),
        }),
        heal: Arc::new(Conduct {
            name: "回復".to_string(),
            sp_cost: 0,
            stamina_cost: 25,
            perks: vec![],
            requirement: ConductRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            conduct_type: ConductType::Basic(ConductTypeBasic::Support(
                ConductTypeBasicSupport::Recover(SupportRecover {
                    potencies: vec![SupportRecoverPotency::Hp(SupportRecoverPotencyHp {
                        hp_recover: 50,
                    })],
                }),
            )),
        }),
        defend: Arc::new(Conduct {
            name: "防御".to_string(),
            sp_cost: 0,
            stamina_cost: 5,
            perks: vec![],
            requirement: ConductRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            conduct_type: ConductType::Basic(ConductTypeBasic::Support(
                ConductTypeBasicSupport::StatusEffect(SuportStatusEffect {
                    status_effects: vec![StatusEffect {
                        potency: StatusEffectPotency::Resistance(StatusEffectResistance {
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
                        }),
                        duration: StatusEffectDuration::Turn(StatusEffectDurationTurn { turns: 1 }),
                    }],
                }),
            )),
        }),
        wait: Arc::new(Conduct {
            name: "待機".to_string(),
            sp_cost: 0,
            stamina_cost: 0,
            perks: vec![],
            requirement: ConductRequirement {
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
                agility: 0,
            },
            conduct_type: ConductType::Basic(ConductTypeBasic::Support(
                ConductTypeBasicSupport::Recover(SupportRecover {
                    potencies: vec![SupportRecoverPotency::Stamina(
                        SupportRecoverPotencyStamina {
                            stamina_recover: 60,
                        },
                    )],
                }),
            )),
        }),
    }
}

// 敵の行動種別（事前決定）
#[derive(Clone, Copy)]
enum EnemyAction {
    Attack,
    Wait,
    Heal,
    ChargeStart,
    ChargeHit,
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
fn create_enemy_claw_strong() -> Action {
    Action {
        steps: vec![
            ActionStep {
                name: "強力な爪攻撃",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 2.0,
                }),
            },
            ActionStep {
                name: "体勢を立て直す",
                specification: ActionStepSpecificationEnum::Wait(ActionStepSpecificationWait {
                    invincible: false,
                }),
            },
        ],
    }
}
fn create_enemy_claw_combo() -> Action {
    Action {
        steps: vec![
            ActionStep {
                name: "爪連撃(1)",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 0.7,
                }),
            },
            ActionStep {
                name: "爪連撃(2)",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 0.7,
                }),
            },
            ActionStep {
                name: "待機",
                specification: ActionStepSpecificationEnum::Wait(ActionStepSpecificationWait {
                    invincible: false,
                }),
            },
        ],
    }
}
fn create_enemy_claw_combo_strong() -> Action {
    Action {
        steps: vec![
            ActionStep {
                name: "爪連撃(1)",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 0.8,
                }),
            },
            ActionStep {
                name: "爪連撃(2)",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 0.8,
                }),
            },
            ActionStep {
                name: "噛みつき",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 2.0,
                }),
            },
            ActionStep {
                name: "待機",
                specification: ActionStepSpecificationEnum::Wait(ActionStepSpecificationWait {
                    invincible: false,
                }),
            },
        ],
    }
}
fn create_enemy_stomp() -> Action {
    Action {
        steps: vec![
            ActionStep {
                name: "飛び上がり",
                specification: ActionStepSpecificationEnum::Wait(ActionStepSpecificationWait {
                    invincible: false,
                }),
            },
            ActionStep {
                name: "踏みつけ",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 2.5,
                }),
            },
        ],
    }
}
// ファイアブレス
fn create_enemy_fire_breath() -> Action {
    Action {
        steps: vec![
            ActionStep {
                name: "息を吸い込む",
                specification: ActionStepSpecificationEnum::Wait(ActionStepSpecificationWait {
                    invincible: false,
                }),
            },
            ActionStep {
                name: "炎を吐き始めた",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 1.0,
                }),
            },
            ActionStep {
                name: "炎を吐き続ける",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 2.5,
                }),
            },
            ActionStep {
                name: "炎を吐き続ける",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 3.0,
                }),
            },
            ActionStep {
                name: "炎を吐き続ける",
                specification: ActionStepSpecificationEnum::Attack(ActionStepSpecificationAttack {
                    power: 0.5,
                }),
            },
            ActionStep {
                name: "息切れ",
                specification: ActionStepSpecificationEnum::Wait(ActionStepSpecificationWait {
                    invincible: false,
                }),
            },
        ],
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
fn create_enemy_heal() -> Action {
    Action {
        steps: vec![ActionStep {
            name: "回復",
            specification: ActionStepSpecificationEnum::Heal(ActionStepSpecificationHeal {
                amount: 100,
            }),
        }],
    }
}

// ================== Battle Module Helpers ==================
fn create_mock_battle() -> Battle {
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

    // プレイヤー原本（仮）
    let player_original = Arc::new(types::Player {
        ability: PlayerAbility {
            vitality: 10,
            spirit: 10,
            endurance: 10,
            agility: 15,
            strength: 10,
            dexterity: 10,
            intelligence: 10,
            faith: 10,
            arcane: 10,
        },
        stats: PlayerStats {
            hp: 100,
            sp: 30,
            stamina: 100,
            equip_load: 0,
        },
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

    // 敵原本（仮）
    let enemy_original = Arc::new(types::Enemy {
        ability: EnemyAbility {
            agility: 10,
            strength: 10,
            dexterity: 10,
            intelligence: 10,
            faith: 10,
            arcane: 10,
        },
        stats: EnemyStats {
            hp: 1500,
            sp: 30,
            break_max: 100,
            break_recovery: 10,
            break_turn: 4,
        },
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
        players: vec![BattlePlayer {
            character_id: 1,
            original: player_original,
            base: BattleCharacterBase {
                current_ability: BattleAbility {
                    agility: 15,
                    strength: 10,
                    dexterity: 10,
                    intelligence: 10,
                    faith: 10,
                    arcane: 10,
                },
                current_stats: BattleStats {
                    max_hp: 100,
                    max_sp: 30,
                    max_stamina: 100,
                    hp_damage: 0,
                    sp_damage: 0,
                    stamina_damage: 0,
                },
                defense_power: def.clone(),
                status_effects: vec![],
            },
        }],
        enemies: vec![BattleEnemy {
            character_id: 2,
            original: enemy_original,
            base: BattleCharacterBase {
                current_ability: BattleAbility {
                    agility: 10,
                    strength: 10,
                    dexterity: 10,
                    intelligence: 10,
                    faith: 10,
                    arcane: 10,
                },
                current_stats: BattleStats {
                    max_hp: 1500,
                    max_sp: 30,
                    max_stamina: 0,
                    hp_damage: 0,
                    sp_damage: 0,
                    stamina_damage: 0,
                },
                defense_power: def,
                status_effects: vec![],
            },
            current_enemy_only_stats: BattleEnemyOnlyStats {
                max_break: 100,
                max_break_turn: 4,
                break_recovery: 10,
                break_damage: 0,
                break_not_damaged_turns: 0,
                break_turns: 0,
            },
        }],
    }
}

// 次ターンに表示される事前決定済み敵行動
#[derive(Resource)]
struct EnemyPlannedAction(ActionProcess);

// コマンド種別
#[derive(Clone, Copy)]
enum CommandKind {
    Attack,
    Skill,
    Heal,
    Defend,
    Wait,
}

// 予約コマンドのキュー
#[derive(Resource, Default)]
struct CommandQueue(std::collections::VecDeque<CommandKind>);

// 直前のプレイヤー実行コマンドが攻撃だったかを保持（攻撃後の攻撃=連撃）
#[derive(Resource, Default)]
struct PlayerChainState {
    last_was_attack: bool,
}

// 未確定の複数選択バッファ（Enterで確定）
#[derive(Resource, Default)]
struct PendingSelections(Vec<CommandKind>);

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
struct UiMomentumText;
#[derive(Component)]
struct UiBuffsText;

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

// ================== Setup ==================
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(BattlePhase::AwaitCommand);
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
        "コマンドを選択してください (A=攻撃 S=強攻撃 H=回復 D=防御 W=待機 / Backspace=直前取り消し / Esc=全クリア / Enter=決定)".to_string(),
    ]));
    commands.insert_resource(CommandQueue::default());
    commands.insert_resource(PlayerChainState::default());
    commands.insert_resource(PendingSelections::default());
    commands.insert_resource(EnemyPlannedAction(first_action));
    commands.insert_resource(ConsecutiveBatch::default());
    commands.insert_resource(EnemyDamagePopup::default());
    // プレイヤー行動定義をリソースとして挿入
    commands.insert_resource(create_default_player_conducts());
    // Battleモジュールの戦闘データを初期化
    commands.insert_resource(BattleResource(create_mock_battle()));

    const MARGIN: Val = Val::Px(12.);
    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    // 画面下のログメッセージ（白枠、最大10行）
    commands
        .spawn((
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
    // 右上にプレイヤーステータス枠（HP/スタミナの文字とゲージ、モメンタム表示）
    commands
        .spawn((
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

            // モメンタム表示テキスト
            col.spawn((
                UiMomentumText,
                Text::new("モメンタム: --- / 100"),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            // 強化状態表示テキスト
            col.spawn((
                UiBuffsText,
                Text::new("強化: なし"),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

    // 画面右端のコマンド入力表示（白枠）
    commands
        .spawn((
            UiCommand,
            Node {
                width: Val::Px(320.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                bottom: Val::Px(16.0),
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            BorderColor::all(Color::WHITE),
            Visibility::Hidden, // 初期は非表示
            ZIndex(10),
        ))
        .with_children(|col| {
            col.spawn((
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

    // コマンド説明（コマンド入力パネルの上に固定表示）
    commands
        .spawn((
            UiCommandHelp,
            Node {
                width: Val::Px(320.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                bottom: Val::Px(180.0), // 入力パネルの上に来るようマージン多め
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
                Text::new(
                    "[コマンド説明]
攻撃:   消費15/威力10/ブレイク10 (強化中: 消費5/威力25/ブレイク25)
        攻撃、強攻撃後の「連撃」に変化 消費が5になる
強攻撃: 消費25/威力25/ブレイク25 (強化中: 威力45/ブレイク40)
        防御直後「ガードカウンター」に変化 威力+5,ブレイク+20
回復:   消費15/回復50 (強化中: 消費20 / 回復60)
防御:   消費10/次の敵攻撃を無効化 (強化中: 消費5)
待機:   消費0/スタミナ+60
強化:   モメンタム50消費 / 11ターン持続\n",
                ),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    println!(
        "ゲーム開始: A=攻撃 S=強攻撃 H=回復(+50) D=防御 W=待機(+スタミナ50回復) / Backspace=直前取り消し / Esc=全クリア / Enter=決定"
    );
}

// ================== Input & Battle Resolution ==================
fn player_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut phase: ResMut<BattlePhase>,
    mut turn: ResMut<Turn>,
    mut log: ResMut<CombatLog>,
    mut queue: ResMut<CommandQueue>,
    mut chain_state: ResMut<PlayerChainState>,
    mut pending: ResMut<PendingSelections>,
    mut planned: ResMut<EnemyPlannedAction>,
    mut batch: ResMut<ConsecutiveBatch>,
    mut enemy_damage_popup: ResMut<EnemyDamagePopup>,
    player_conducts: Res<PlayerConducts>,
    // Battleモジュール
    mut battle_resource: ResMut<BattleResource>,
) {
    if *phase == BattlePhase::Finished {
        return;
    }

    let battle = &mut battle_resource.0;

    // 連続コマンド確認フェーズの処理（Y=実行 / N=選択しなおし）
    if *phase == BattlePhase::ConfirmQueued {
        // キューが空なら待機に戻る
        if queue.0.front().is_none() {
            *phase = BattlePhase::AwaitCommand;
            return;
        }
        // 実行確定（YまたはEnter）
        if keyboard.just_pressed(KeyCode::KeyY) || keyboard.just_pressed(KeyCode::Enter) {
            if let Some(next) = queue.0.pop_front() {
                batch.executed += 1;
                // この後の通常解決フローで処理する
                let mut commands_to_process: Vec<CommandKind> = vec![next];

                // 共通のコマンド解決処理
                let mut resolve_command = |cmd: CommandKind| {
                    *phase = BattlePhase::InBattle;
                    let name = match cmd {
                        CommandKind::Attack => "攻撃",
                        CommandKind::Skill => "強攻撃",
                        CommandKind::Heal => "回復",
                        CommandKind::Defend => "防御",
                        CommandKind::Wait => "待機",
                    };
                    log.0
                        .push(format!("ターン {} プレイヤーは{}を選択", turn.0, name));
                    // 連撃判定（直前が攻撃または強攻撃 かつ 今回が攻撃）
                    let is_chain =
                        chain_state.last_was_attack && matches!(cmd, CommandKind::Attack);

                    // Battleモジュールで行動実行
                    let player_id = battle.players.first().map(|p| p.character_id).unwrap_or(1);
                    let enemy_id = battle.enemies.first().map(|e| e.character_id).unwrap_or(2);
                    let player_conduct = match cmd {
                        CommandKind::Attack => {
                            let c = player_conducts.attack.as_ref();
                            BattleConduct {
                                actor_character_id: player_id,
                                target_character_id: enemy_id,
                                conduct: Conduct {
                                    name: c.name.clone(),
                                    sp_cost: c.sp_cost,
                                    stamina_cost: c.stamina_cost,
                                    perks: c.perks.clone(),
                                    requirement: ConductRequirement {
                                        strength: c.requirement.strength,
                                        dexterity: c.requirement.dexterity,
                                        intelligence: c.requirement.intelligence,
                                        faith: c.requirement.faith,
                                        arcane: c.requirement.arcane,
                                        agility: c.requirement.agility,
                                    },
                                    conduct_type: c.conduct_type.clone(),
                                },
                                weapon: None,
                            }
                        }
                        CommandKind::Skill => {
                            let c = player_conducts.skill.as_ref();
                            BattleConduct {
                                actor_character_id: player_id,
                                target_character_id: enemy_id,
                                conduct: Conduct {
                                    name: c.name.clone(),
                                    sp_cost: c.sp_cost,
                                    stamina_cost: c.stamina_cost,
                                    perks: c.perks.clone(),
                                    requirement: ConductRequirement {
                                        strength: c.requirement.strength,
                                        dexterity: c.requirement.dexterity,
                                        intelligence: c.requirement.intelligence,
                                        faith: c.requirement.faith,
                                        arcane: c.requirement.arcane,
                                        agility: c.requirement.agility,
                                    },
                                    conduct_type: c.conduct_type.clone(),
                                },
                                weapon: None,
                            }
                        }
                        CommandKind::Heal => {
                            let c = player_conducts.heal.as_ref();
                            BattleConduct {
                                actor_character_id: player_id,
                                target_character_id: player_id,
                                conduct: Conduct {
                                    name: c.name.clone(),
                                    sp_cost: c.sp_cost,
                                    stamina_cost: c.stamina_cost,
                                    perks: c.perks.clone(),
                                    requirement: ConductRequirement {
                                        strength: c.requirement.strength,
                                        dexterity: c.requirement.dexterity,
                                        intelligence: c.requirement.intelligence,
                                        faith: c.requirement.faith,
                                        arcane: c.requirement.arcane,
                                        agility: c.requirement.agility,
                                    },
                                    conduct_type: c.conduct_type.clone(),
                                },
                                weapon: None,
                            }
                        }
                        CommandKind::Defend => {
                            let c = player_conducts.defend.as_ref();
                            BattleConduct {
                                actor_character_id: player_id,
                                target_character_id: player_id,
                                conduct: Conduct {
                                    name: c.name.clone(),
                                    sp_cost: c.sp_cost,
                                    stamina_cost: c.stamina_cost,
                                    perks: c.perks.clone(),
                                    requirement: ConductRequirement {
                                        strength: c.requirement.strength,
                                        dexterity: c.requirement.dexterity,
                                        intelligence: c.requirement.intelligence,
                                        faith: c.requirement.faith,
                                        arcane: c.requirement.arcane,
                                        agility: c.requirement.agility,
                                    },
                                    conduct_type: c.conduct_type.clone(),
                                },
                                weapon: None,
                            }
                        }
                        CommandKind::Wait => {
                            let c = player_conducts.wait.as_ref();
                            BattleConduct {
                                actor_character_id: player_id,
                                target_character_id: player_id,
                                conduct: Conduct {
                                    name: c.name.clone(),
                                    sp_cost: c.sp_cost,
                                    stamina_cost: c.stamina_cost,
                                    perks: c.perks.clone(),
                                    requirement: ConductRequirement {
                                        strength: c.requirement.strength,
                                        dexterity: c.requirement.dexterity,
                                        intelligence: c.requirement.intelligence,
                                        faith: c.requirement.faith,
                                        arcane: c.requirement.arcane,
                                        agility: c.requirement.agility,
                                    },
                                    conduct_type: c.conduct_type.clone(),
                                },
                                weapon: None,
                            }
                        }
                    };

                    let enemy_conduct = battle.decide_enemy_conduct(DecideEnemyConductRequest {
                        enemy_character_id: enemy_id,
                    });
                    let order = battle.decide_order(BattleDecideOrderRequest {
                        character_ids: vec![player_id, enemy_id],
                    });

                    let mut player_dealt_damage_hp: u32 = 0;
                    for actor_id in order {
                        let conduct_to_execute = if actor_id == player_id {
                            &player_conduct
                        } else {
                            &enemy_conduct
                        };
                        let incident = battle.execute_conduct(BattleExecuteConductRequest {
                            conduct: BattleConduct {
                                actor_character_id: conduct_to_execute.actor_character_id,
                                target_character_id: conduct_to_execute.target_character_id,
                                conduct: Conduct {
                                    name: conduct_to_execute.conduct.name.clone(),
                                    sp_cost: conduct_to_execute.conduct.sp_cost,
                                    stamina_cost: conduct_to_execute.conduct.stamina_cost,
                                    perks: conduct_to_execute.conduct.perks.clone(),
                                    requirement: ConductRequirement {
                                        strength: conduct_to_execute.conduct.requirement.strength,
                                        dexterity: conduct_to_execute.conduct.requirement.dexterity,
                                        intelligence: conduct_to_execute
                                            .conduct
                                            .requirement
                                            .intelligence,
                                        faith: conduct_to_execute.conduct.requirement.faith,
                                        arcane: conduct_to_execute.conduct.requirement.arcane,
                                        agility: conduct_to_execute.conduct.requirement.agility,
                                    },
                                    conduct_type: conduct_to_execute.conduct.conduct_type.clone(),
                                },
                                weapon: None,
                            },
                        });

                        match incident {
                            BattleIncident::Conduct(c) => match c.outcome {
                                BattleIncidentConductOutcome::Failure(_) => {
                                    log.0.push(format!("{}は不発", c.conduct.conduct.name));
                                }
                                BattleIncidentConductOutcome::Success(s) => {
                                    for change in s.attacker.stats_changes.iter() {
                                        match change {
                                            BattleIncidentStats::DamageSp(d) => {
                                                log.0.push(format!(
                                                    "SP -{} ({} → {})",
                                                    d.damage, d.before, d.after
                                                ))
                                            }
                                            BattleIncidentStats::DamageStamina(d) => {
                                                log.0.push(format!(
                                                    "Stamina -{} ({} → {})",
                                                    d.damage, d.before, d.after
                                                ))
                                            }
                                            _ => {}
                                        }
                                    }
                                    for def in s.defenders.iter() {
                                        for change in def.stats_changes.iter() {
                                            match change {
                                                BattleIncidentStats::DamageHp(d) => {
                                                    if c.attacker_id == player_id {
                                                        player_dealt_damage_hp = d.damage;
                                                    }
                                                    log.0.push(format!(
                                                        "{} に{}ダメージ (HP {} → {})",
                                                        if def.character_id == enemy_id {
                                                            "敵"
                                                        } else {
                                                            "プレイヤー"
                                                        },
                                                        d.damage,
                                                        d.before,
                                                        d.after
                                                    ));
                                                }
                                                BattleIncidentStats::RecoverHp(r) => {
                                                    log.0.push(format!(
                                                        "{} のHPを{}回復 ({} → {})",
                                                        if def.character_id == player_id {
                                                            "プレイヤー"
                                                        } else {
                                                            "敵"
                                                        },
                                                        r.recover,
                                                        r.before,
                                                        r.after
                                                    ))
                                                }
                                                BattleIncidentStats::DamageBreak(d) => log.0.push(
                                                    format!("敵にブレイクダメージ {}", d.damage),
                                                ),
                                                BattleIncidentStats::RecoverBreak(r) => log.0.push(
                                                    format!("敵のブレイク回復 {}", r.recover),
                                                ),
                                                BattleIncidentStats::RecoverStamina(r) => {
                                                    log.0.push(format!(
                                                        "Stamina +{} ({} → {})",
                                                        r.recover, r.before, r.after
                                                    ))
                                                }
                                                _ => {}
                                            }
                                        }
                                        if def.is_evaded {
                                            log.0.push("回避した".to_string());
                                        }
                                        if def.is_defended {
                                            log.0.push("防御した".to_string());
                                        }
                                    }
                                }
                            },
                            BattleIncident::AutoTrigger(_) => {}
                        }
                    }

                    // TODO: 敵の行動をexecute_conductで処理するようにする
                    //     if e_hp.current > 0 {
                    //         // 事前決定済みの敵行動を実行
                    //         if enemy_action_canceled_this_turn {
                    //             // このターンの行動はキャンセル
                    //             log.0.push("敵の行動はブレイクによりキャンセル".to_string());
                    //         } else {
                    //             let action = &mut planned.0;
                    //             let step = action.current_step().unwrap();
                    //             match step.specification {
                    //                 ActionStepSpecificationEnum::Attack(spec) => {
                    //                     let mut incoming = (e_attack.0 as f32 * spec.power) as i32;
                    //                     {
                    //                         let mut d = def_guard.p0();
                    //                         if d.0 {
                    //                             incoming = 0;
                    //                             d.0 = false; // 一度きり
                    //                         }
                    //                     }
                    //                     p_hp.current = (p_hp.current - incoming).max(0);
                    //                     log.0.push(format!(
                    //                         "敵の行動: {} → {}ダメージ (プレイヤーHP {} / {})",
                    //                         step.name, incoming, p_hp.current, p_hp.max
                    //                     ));
                    //                 }
                    //                 ActionStepSpecificationEnum::Wait(_) => {
                    //                     log.0.push(format!("敵の行動: {} (何もしない)", step.name));
                    //                 }
                    //                 ActionStepSpecificationEnum::Heal(spec) => {
                    //                     // プレイヤーがこのターンに攻撃していた場合、敵の回復量は半減
                    //                     let base_heal = spec.amount;
                    //                     let heal_amount = if matches!(
                    //                         cmd,
                    //                         CommandKind::Attack | CommandKind::Skill
                    //                     ) {
                    //                         base_heal / 2
                    //                     } else {
                    //                         base_heal
                    //                     };
                    //                     let before = e_hp.current;
                    //                     e_hp.current = (e_hp.current + heal_amount).min(e_hp.max);
                    //                     let healed = e_hp.current - before;
                    //                     log.0.push(format!(
                    //                         "敵の行動: {} → HPを{}回復 (敵HP {} / {})",
                    //                         step.name, healed, e_hp.current, e_hp.max
                    //                     ));
                    //                 }
                    //             }
                    //             action.next();
                    //         }
                    //     }

                    turn.0 += 1;
                    *phase = BattlePhase::AwaitCommand;
                };

                // 今回は1件だけ処理（各ターン1コマンドのルール）
                resolve_command(commands_to_process[0]);
            }
            return;
        }
        // 再選択（NまたはEsc）: 以降の予約コマンドをリセット
        if keyboard.just_pressed(KeyCode::KeyN) || keyboard.just_pressed(KeyCode::Escape) {
            let cleared = queue.0.len();
            queue.0.clear();
            pending.0.clear();
            batch.total = 0;
            batch.executed = 0;
            if cleared > 0 {
                log.0.push(
                    "連続コマンドの予約をリセットしました。コマンドを選び直してください"
                        .to_string(),
                );
            }
            *phase = BattlePhase::AwaitCommand;

            return;
        }
        // 入力待ち
        return;
    }

    // フェーズが待機でない場合は何もしない
    if *phase != BattlePhase::AwaitCommand {
        return;
    }

    // 予約キューがあれば、先頭を実行するか確認フェーズに遷移
    let mut commands_to_process: Vec<CommandKind> = Vec::new();
    if let Some(_next) = queue.0.front() {
        // コマンド入力パネルで確認表示を行うため、ここではログ出力しない
        *phase = BattlePhase::ConfirmQueued;
        return;
    } else {
        // 未確定選択へ追加（このフレームで押されたキー）
        let mut added: Vec<&'static str> = Vec::new();
        // 最大選択数制限（3件）
        const MAX_SELECT: usize = 3;
        let at_limit = pending.0.len() >= MAX_SELECT;

        // 取り消し操作: Backspace=直前取り消し / Escape=全クリア（ログには出さない）
        if keyboard.just_pressed(KeyCode::Escape) {
            if !pending.0.is_empty() {
                pending.0.clear();
            }
        }
        if keyboard.just_pressed(KeyCode::Backspace) {
            if let Some(removed) = pending.0.pop() {
                let _ = removed; // ログは出さない
            }
        }
        if keyboard.just_pressed(KeyCode::KeyA) {
            if !at_limit {
                pending.0.push(CommandKind::Attack);
                added.push("攻撃");
            } else {
                log.0
                    .push("これ以上選択を追加できません (最大3件)".to_string());
            }
        }
        if keyboard.just_pressed(KeyCode::KeyS) {
            if !at_limit {
                pending.0.push(CommandKind::Skill);
                added.push("強攻撃");
            } else {
                log.0
                    .push("これ以上選択を追加できません (最大3件)".to_string());
            }
        }
        if keyboard.just_pressed(KeyCode::KeyH) {
            if !at_limit {
                pending.0.push(CommandKind::Heal);
                added.push("回復");
            } else {
                log.0
                    .push("これ以上選択を追加できません (最大3件)".to_string());
            }
        }
        if keyboard.just_pressed(KeyCode::KeyD) {
            if !at_limit {
                pending.0.push(CommandKind::Defend);
                added.push("防御");
            } else {
                log.0
                    .push("これ以上選択を追加できません (最大3件)".to_string());
            }
        }
        if keyboard.just_pressed(KeyCode::KeyW) {
            if !at_limit {
                pending.0.push(CommandKind::Wait);
                added.push("待機");
            } else {
                log.0
                    .push("これ以上選択を追加できません (最大3件)".to_string());
            }
        }
        // 強化系コマンドは廃止
        // 選択追加のログは出さず、UI側表示に任せる

        // Enterで確定: 先頭を実行、2つ目以降を予約キューへ
        if keyboard.just_pressed(KeyCode::Enter) && !pending.0.is_empty() {
            // 確定時、選択した全コマンドをログ出力
            let all_names = pending
                .0
                .iter()
                .map(|c| match c {
                    CommandKind::Attack => "攻撃",
                    CommandKind::Skill => "強攻撃",
                    CommandKind::Heal => "回復",
                    CommandKind::Defend => "防御",
                    CommandKind::Wait => "待機",
                })
                .collect::<Vec<_>>()
                .join(", ");
            log.0.push(format!("選択確定: {}", all_names));
            // 先頭を今回実行（連撃判定は実行時に行う）
            commands_to_process.push(pending.0[0]);
            // 残りをキューへ（連撃判定は実行時に行う）
            for &cmd in pending.0.iter().skip(1) {
                queue.0.push_back(cmd);
            }
            // 連続バッチ総数の記録と実行済み数のリセット
            batch.total = pending.0.len();
            batch.executed = 0;
            // モメンタム増加は実行選択時に行うため、ここでは加算しない
            // ログ出力
            if pending.0.len() > 1 {
                let names = pending
                    .0
                    .iter()
                    .skip(1)
                    .map(|c| match c {
                        CommandKind::Attack => "攻撃",
                        CommandKind::Skill => "強攻撃",
                        CommandKind::Heal => "回復",
                        CommandKind::Defend => "防御",
                        CommandKind::Wait => "待機",
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                log.0.push(format!(
                    "{}件のコマンドを予約 ({})",
                    pending.0.len() - 1,
                    names
                ));
            }
            // バッファをクリア
            pending.0.clear();
        }
    }

    if commands_to_process.is_empty() {
        return; // 入力も予約もなし
    }

    // 共通のコマンド解決処理
    let mut resolve_command = |cmd: CommandKind| {
        *phase = BattlePhase::InBattle;
        let name = match cmd {
            CommandKind::Attack => "攻撃",
            CommandKind::Skill => "強攻撃",
            CommandKind::Heal => "回復",
            CommandKind::Defend => "防御",
            CommandKind::Wait => "待機",
        };
        log.0
            .push(format!("ターン {} プレイヤーは{}を選択", turn.0, name));
        // 連撃判定（直前が攻撃または強攻撃 かつ 今回が攻撃）
        let is_chain = chain_state.last_was_attack && matches!(cmd, CommandKind::Attack);

        // コストチェック（実行時にも確認）。不足なら行動失敗。
        let cost = match cmd {
            CommandKind::Attack => 5,
            CommandKind::Skill => 25,
            CommandKind::Heal => 15,
            CommandKind::Defend => 10,
            CommandKind::Wait => 0,
        };

        let player_id = battle.players.first().map(|p| p.character_id).unwrap_or(1);
        let enemy_id = battle.enemies.first().map(|e| e.character_id).unwrap_or(2);
        let player_conduct = match cmd {
            CommandKind::Attack => BattleConduct {
                actor_character_id: player_id,
                target_character_id: enemy_id,
                conduct: Conduct {
                    name: "攻撃".to_string(),
                    sp_cost: 0,
                    stamina_cost: cost as u32,
                    perks: vec![ConductPerk::Melee],
                    requirement: ConductRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    conduct_type: ConductType::Basic(ConductTypeBasic::Attack(
                        ConductTypeBasicAttack {
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
                            break_power: 10,
                        },
                    )),
                },
                weapon: None,
            },
            CommandKind::Skill => BattleConduct {
                actor_character_id: player_id,
                target_character_id: enemy_id,
                conduct: Conduct {
                    name: "強攻撃".to_string(),
                    sp_cost: 0,
                    stamina_cost: cost as u32,
                    perks: vec![ConductPerk::Melee],
                    requirement: ConductRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    conduct_type: ConductType::Basic(ConductTypeBasic::Attack(
                        ConductTypeBasicAttack {
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
                            break_power: 20,
                        },
                    )),
                },
                weapon: None,
            },
            CommandKind::Heal => BattleConduct {
                actor_character_id: player_id,
                target_character_id: player_id,
                conduct: Conduct {
                    name: "回復".to_string(),
                    sp_cost: 0,
                    stamina_cost: cost as u32,
                    perks: vec![],
                    requirement: ConductRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    conduct_type: ConductType::Basic(ConductTypeBasic::Support(
                        ConductTypeBasicSupport::Recover(SupportRecover {
                            potencies: vec![SupportRecoverPotency::Hp(SupportRecoverPotencyHp {
                                hp_recover: 50,
                            })],
                        }),
                    )),
                },
                weapon: None,
            },
            CommandKind::Defend => BattleConduct {
                actor_character_id: player_id,
                target_character_id: player_id,
                conduct: Conduct {
                    name: "防御".to_string(),
                    sp_cost: 0,
                    stamina_cost: cost as u32,
                    perks: vec![],
                    requirement: ConductRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    conduct_type: ConductType::Basic(ConductTypeBasic::Support(
                        ConductTypeBasicSupport::StatusEffect(SuportStatusEffect {
                            status_effects: vec![StatusEffect {
                                potency: StatusEffectPotency::Resistance(StatusEffectResistance {
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
                                }),
                                duration: StatusEffectDuration::Turn(StatusEffectDurationTurn {
                                    turns: 1,
                                }),
                            }],
                        }),
                    )),
                },
                weapon: None,
            },
            CommandKind::Wait => BattleConduct {
                actor_character_id: player_id,
                target_character_id: player_id,
                conduct: Conduct {
                    name: "待機".to_string(),
                    sp_cost: 0,
                    stamina_cost: 0,
                    perks: vec![],
                    requirement: ConductRequirement {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        faith: 0,
                        arcane: 0,
                        agility: 0,
                    },
                    conduct_type: ConductType::Basic(ConductTypeBasic::Support(
                        ConductTypeBasicSupport::Recover(SupportRecover {
                            potencies: vec![SupportRecoverPotency::Stamina(
                                SupportRecoverPotencyStamina {
                                    stamina_recover: 60,
                                },
                            )],
                        }),
                    )),
                },
                weapon: None,
            },
        };

        let enemy_conduct = battle.decide_enemy_conduct(DecideEnemyConductRequest {
            enemy_character_id: enemy_id,
        });
        let order = battle.decide_order(BattleDecideOrderRequest {
            character_ids: vec![player_id, enemy_id],
        });

        let mut player_dealt_damage_hp: u32 = 0;
        for actor_id in order {
            let conduct_to_execute = if actor_id == player_id {
                &player_conduct
            } else {
                &enemy_conduct
            };
            let incident = battle.execute_conduct(BattleExecuteConductRequest {
                conduct: BattleConduct {
                    actor_character_id: conduct_to_execute.actor_character_id,
                    target_character_id: conduct_to_execute.target_character_id,
                    conduct: Conduct {
                        name: conduct_to_execute.conduct.name.clone(),
                        sp_cost: conduct_to_execute.conduct.sp_cost,
                        stamina_cost: conduct_to_execute.conduct.stamina_cost,
                        perks: conduct_to_execute.conduct.perks.clone(),
                        requirement: ConductRequirement {
                            strength: conduct_to_execute.conduct.requirement.strength,
                            dexterity: conduct_to_execute.conduct.requirement.dexterity,
                            intelligence: conduct_to_execute.conduct.requirement.intelligence,
                            faith: conduct_to_execute.conduct.requirement.faith,
                            arcane: conduct_to_execute.conduct.requirement.arcane,
                            agility: conduct_to_execute.conduct.requirement.agility,
                        },
                        conduct_type: conduct_to_execute.conduct.conduct_type.clone(),
                    },
                    weapon: None,
                },
            });

            match incident {
                BattleIncident::Conduct(c) => match c.outcome {
                    BattleIncidentConductOutcome::Failure(_) => {
                        log.0.push(format!("{}は不発", c.conduct.conduct.name))
                    }
                    BattleIncidentConductOutcome::Success(s) => {
                        for change in s.attacker.stats_changes.iter() {
                            match change {
                                BattleIncidentStats::DamageSp(d) => log
                                    .0
                                    .push(format!("SP -{} ({} → {})", d.damage, d.before, d.after)),
                                BattleIncidentStats::DamageStamina(d) => log.0.push(format!(
                                    "Stamina -{} ({} → {})",
                                    d.damage, d.before, d.after
                                )),
                                _ => {}
                            }
                        }
                        for def in s.defenders.iter() {
                            for change in def.stats_changes.iter() {
                                match change {
                                    BattleIncidentStats::DamageHp(d) => {
                                        if c.attacker_id == player_id {
                                            player_dealt_damage_hp = d.damage;
                                        }
                                        log.0.push(format!(
                                            "{} に{}ダメージ (HP {} → {})",
                                            if def.character_id == enemy_id {
                                                "敵"
                                            } else {
                                                "プレイヤー"
                                            },
                                            d.damage,
                                            d.before,
                                            d.after
                                        ));
                                    }
                                    BattleIncidentStats::RecoverHp(r) => log.0.push(format!(
                                        "{} のHPを{}回復 ({} → {})",
                                        if def.character_id == player_id {
                                            "プレイヤー"
                                        } else {
                                            "敵"
                                        },
                                        r.recover,
                                        r.before,
                                        r.after
                                    )),
                                    BattleIncidentStats::DamageBreak(d) => {
                                        log.0.push(format!("敵にブレイクダメージ {}", d.damage))
                                    }
                                    BattleIncidentStats::RecoverBreak(r) => {
                                        log.0.push(format!("敵のブレイク回復 {}", r.recover))
                                    }
                                    BattleIncidentStats::RecoverStamina(r) => log.0.push(format!(
                                        "Stamina +{} ({} → {})",
                                        r.recover, r.before, r.after
                                    )),
                                    _ => {}
                                }
                            }
                            if def.is_evaded {
                                log.0.push("回避した".to_string());
                            }
                            if def.is_defended {
                                log.0.push("防御した".to_string());
                            }
                        }
                    }
                },
                BattleIncident::AutoTrigger(_) => {}
            }
        }

        // TODO: 敵の行動
        // if e_hp.current > 0 {
        //     // 事前決定済みの敵行動を実行
        //     if e_bstate.remaining_turns > 0 {
        //         // ブレイク中は行動不能
        //         log.0.push("敵はブレイク中のため行動不能".to_string());
        //     } else if enemy_action_canceled_this_turn {
        //         // このターンの行動はキャンセル
        //         log.0.push("敵の行動はブレイクによりキャンセル".to_string());
        //     } else {
        //         let action = &mut planned.0;
        //         let step = action.current_step().unwrap();
        //         match step.specification {
        //             ActionStepSpecificationEnum::Attack(spec) => {
        //                 let mut incoming = (e_attack.0 as f32 * spec.power) as i32;
        //                 {
        //                     let mut d = def_guard.p0();
        //                     if d.0 {
        //                         incoming = 0;
        //                         d.0 = false; // 一度きり
        //                     }
        //                 }
        //                 p_hp.current = (p_hp.current - incoming).max(0);
        //                 log.0.push(format!(
        //                     "敵の行動: {} → {}ダメージ (プレイヤーHP {} / {})",
        //                     step.name, incoming, p_hp.current, p_hp.max
        //                 ));
        //             }
        //             ActionStepSpecificationEnum::Wait(_) => {
        //                 log.0.push(format!("敵の行動: {} (何もしない)", step.name));
        //             }
        //             ActionStepSpecificationEnum::Heal(spec) => {
        //                 // プレイヤーがこのターンに攻撃していた場合、敵の回復量は半減
        //                 let base_heal = spec.amount;
        //                 let heal_amount = if matches!(cmd, CommandKind::Attack | CommandKind::Skill)
        //                 {
        //                     base_heal / 2
        //                 } else {
        //                     base_heal
        //                 };
        //                 let before = e_hp.current;
        //                 e_hp.current = (e_hp.current + heal_amount).min(e_hp.max);
        //                 let healed = e_hp.current - before;
        //                 log.0.push(format!(
        //                     "敵の行動: {} → HPを{}回復 (敵HP {} / {})",
        //                     step.name, healed, e_hp.current, e_hp.max
        //                 ));
        //             }
        //         }
        //         action.next();
        //     }
        // }

        // TODO:
        // 次ターンの敵行動を事前決定（敵が生きている場合）
        // if e_hp.current > 0 && p_hp.current > 0 {
        //     if planned.0.is_finished() {
        //         // 現在の行動が完了している場合、新たに行動を決定

        //         let roll: f32 = rand::random::<f32>();
        //         // 敵HPが半分以下なら、回復とため開始を選択肢に含める
        //         let next = if e_hp.current * 2 <= e_hp.max {
        //             // 攻撃 / 待機 / 回復 / ため(準備)
        //             match () {
        //                 _ if roll < 0.1 => create_enemy_wait(),
        //                 _ if roll < 0.2 => create_enemy_heal(),
        //                 _ if roll < 0.3 => create_enemy_attack(),
        //                 _ if roll < 0.5 => create_enemy_claw_combo_strong(),
        //                 _ if roll < 0.7 => create_enemy_claw_strong(),
        //                 _ if roll < 0.8 => create_enemy_stomp(),
        //                 _ => create_enemy_fire_breath(),
        //             }
        //         } else {
        //             match () {
        //                 _ if roll < 0.3 => create_enemy_wait(),
        //                 _ if roll < 0.6 => create_enemy_attack(),
        //                 _ if roll < 0.8 => create_enemy_claw_combo(),
        //                 _ if roll < 0.9 => create_enemy_claw_strong(),
        //                 _ => create_enemy_stomp(),
        //             }
        //         };

        //         // TODO: 毎回生成してるのやめる
        //         planned.0 = ActionProcess::from(&Arc::new(next));
        //     }
        //     log.0.push(format!(
        //         "次ターン敵行動予定: {}",
        //         planned.0.current_step().unwrap().name
        //     ));
        // }

        turn.0 += 1;
        *phase = BattlePhase::AwaitCommand;
    };

    // 今回は1件だけ処理（各ターン1コマンドのルール）
    resolve_command(commands_to_process[0]);
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
    let player = battle.players.first().unwrap();
    let player_hp = player.base.current_stats.max_hp - player.base.current_stats.hp_damage;
    let enemy = battle.enemies.first().unwrap();
    let enemy_hp = enemy.base.current_stats.max_hp - enemy.base.current_stats.hp_damage;

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
    pending: Res<PendingSelections>,
    queue: Res<CommandQueue>,
    // mut ui_q: Query<&mut Children, With<UiRoot>>,
    planned: Res<EnemyPlannedAction>,
    battle_resource: Res<BattleResource>,
    mut ui_staus_q: Query<&mut Text, (With<UiStatus>, Without<UiPhase>, Without<UiLog>)>,
    // プレイヤーステータス（右上）の更新用: テキスト群（HP、スタミナ、モメンタム）
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

    let player = battle.players.first().unwrap();
    let p_hp = &player.base.current_stats.max_hp - player.base.current_stats.hp_damage;
    let p_stamina =
        &player.base.current_stats.max_stamina - player.base.current_stats.stamina_damage;
    let enemy = battle.enemies.first().unwrap();
    let e_hp = &enemy.base.current_stats.max_hp - enemy.base.current_stats.hp_damage;

    ui_status_text.0 = format!(
        "プレイヤーHP: {} / {}\nスタミナ: {} / {}\n100\n\n敵HP: {} / {}\n敵ブレイク値: {} / 100\n敵状態: {}\n\n",
        p_hp,
        player.base.current_stats.max_hp,
        p_stamina,
        player.base.current_stats.max_stamina,
        e_hp,
        enemy.base.current_stats.max_hp,
        enemy.current_enemy_only_stats.break_damage,
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
    // let atk_break_add = if buffs.attack > 0 { 25 } else { 15 };
    // let atk_enh_suffix = if buffs.attack > 0 { " (強化中)" } else { "" };
    // eff_atk_text.0 = format!(
    //     "攻撃 力:{} 消費:{}{} / ブレイク+{}\n",
    //     atk_power, atk_cost, atk_enh_suffix, atk_break_add
    // );
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

    let enemy_action_str = if let Some(step) = planned.0.current_step() {
        step.name
    } else {
        "不明"
    };
    // 選択中コマンド表示用の文字列
    let selected_str = if pending.0.is_empty() {
        "(なし)".to_string()
    } else {
        pending
            .0
            .iter()
            .map(|c| match c {
                CommandKind::Attack => "攻撃",
                CommandKind::Skill => "強攻撃",
                CommandKind::Heal => "回復",
                CommandKind::Defend => "防御",
                CommandKind::Wait => "待機",
            })
            .collect::<Vec<_>>()
            .join(", ")
    };
    let phase_str = match *phase {
        BattlePhase::AwaitCommand => format!(
            "コマンド入力待ち \nコマンドを選択してください(最大3つ)\n A=攻撃 S=強攻撃 H=回復 D=防御 W=待機\n Backspace=直前取り消し / Esc=全クリア\n Enter=決定\n [選択中] {selected_str}"
        ),
        BattlePhase::ConfirmQueued => {
            let next_name = if let Some(next) = queue.0.front() {
                match next {
                    CommandKind::Attack => "攻撃",
                    CommandKind::Skill => "強攻撃",
                    CommandKind::Heal => "回復",
                    CommandKind::Defend => "防御",
                    CommandKind::Wait => "待機",
                }
            } else {
                "(なし)"
            };
            format!(
                "連続コマンド確認\n次の予約: {}\n Y=実行 / N=選択しなおし(以降リセット)",
                next_name
            )
        }
        BattlePhase::InBattle => "処理中".to_string(),
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

// コマンド入力表示（右端パネル）の表示制御と内容更新
fn ui_update_command_system(
    phase: Res<BattlePhase>,
    planned: Res<EnemyPlannedAction>,
    pending: Res<PendingSelections>,
    queue: Res<CommandQueue>,
    mut cmd_panel_q: Query<(&mut Visibility, &Children), With<UiCommand>>,
    mut texts: Query<&mut Text>,
) {
    let Ok((mut vis, children)) = cmd_panel_q.single_mut() else {
        return;
    };
    match *phase {
        BattlePhase::AwaitCommand => {
            *vis = Visibility::Visible;
            for child in children.iter() {
                if let Ok(mut t) = texts.get_mut(child) {
                    let selected_str = if pending.0.is_empty() {
                        "(なし)".to_string()
                    } else {
                        pending
                            .0
                            .iter()
                            .map(|c| match c {
                                CommandKind::Attack => "攻撃",
                                CommandKind::Skill => "強攻撃",
                                CommandKind::Heal => "回復",
                                CommandKind::Defend => "防御",
                                CommandKind::Wait => "待機",
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    };
                    t.0 = format!(
                        "[コマンド入力] \nA=攻撃 S=強攻撃 H=回復 D=防御 W=待機\nZ=攻撃強化 X=強攻撃強化 C=回復強化 V=防御強化\nBackspace=直前取り消し Esc=全クリア Enter=決定 \n選択中: {selected_str}"
                    );
                }
            }
        }
        BattlePhase::ConfirmQueued => {
            *vis = Visibility::Visible;
            for child in children.iter() {
                if let Ok(mut t) = texts.get_mut(child) {
                    let next_name = if let Some(next) = queue.0.front() {
                        match next {
                            CommandKind::Attack => "攻撃",
                            CommandKind::Skill => "強攻撃",
                            CommandKind::Heal => "回復",
                            CommandKind::Defend => "防御",
                            CommandKind::Wait => "待機",
                        }
                    } else {
                        "(なし)"
                    };
                    t.0 = format!(
                        "[連続コマンド確認]\n次の予約: {}\nY=実行 / N=選び直し(以降の予約はリセット)",
                        next_name
                    );
                }
            }
        }
        _ => {
            *vis = Visibility::Hidden;
        }
    }
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

// 右上プレイヤーステータスの更新（HP/スタミナテキスト＆ゲージ、モメンタムテキスト）
fn ui_update_player_status_system(
    battle_resource: Res<BattleResource>,
    mut hp_text_q: Query<&mut Text, (With<UiHpText>, Without<UiStaText>, Without<UiMomentumText>)>,
    mut sta_text_q: Query<&mut Text, (With<UiStaText>, Without<UiHpText>, Without<UiMomentumText>)>,
    mut momentum_text_q: Query<
        &mut Text,
        (With<UiMomentumText>, Without<UiHpText>, Without<UiStaText>),
    >,
    mut buffs_text_q: Query<
        &mut Text,
        (
            With<UiBuffsText>,
            Without<UiHpText>,
            Without<UiStaText>,
            Without<UiMomentumText>,
        ),
    >,
    mut gauge_params: ParamSet<(
        Query<&mut Node, With<UiHpGaugeFill>>,
        Query<&mut Node, With<UiStaGaugeFill>>,
    )>,
) {
    let battle = &battle_resource.0;

    let player = battle.players.first().unwrap();
    let p_hp = &player.base.current_stats.max_hp - player.base.current_stats.hp_damage;
    let p_sta = &player.base.current_stats.max_stamina - player.base.current_stats.stamina_damage;

    // コンテナ内の最初のTextを簡潔表示用に更新
    if let Ok(mut hp_text) = hp_text_q.single_mut() {
        hp_text.0 = format!("HP: {} / {}", p_hp, player.base.current_stats.max_hp);
    }
    if let Ok(mut sta_text) = sta_text_q.single_mut() {
        sta_text.0 = format!(
            "スタミナ: {} / {}",
            p_sta, player.base.current_stats.max_stamina
        );
    }
    if let Ok(mut buffs_text) = buffs_text_q.single_mut() {
        // 表示: 強化中のものと残りターン。未強化は「なし」。
        let mut parts: Vec<String> = Vec::new();
        if parts.is_empty() {
            buffs_text.0 = "強化: なし".to_string();
        } else {
            buffs_text.0 = format!("強化: {}", parts.join(" "));
        }
    }

    // ゲージ幅更新
    if let Ok(mut hp_node) = gauge_params.p0().single_mut() {
        let ratio = if player.base.current_stats.max_hp > 0 {
            (p_hp as f32 / player.base.current_stats.max_hp as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        hp_node.width = percent((ratio * 100.0).round());
    }
    if let Ok(mut sta_node) = gauge_params.p1().single_mut() {
        let ratio = if player.base.current_stats.max_stamina > 0 {
            (p_sta as f32 / player.base.current_stats.max_stamina as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        sta_node.width = percent((ratio * 100.0).round());
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
    let e_hp = &enemy.base.current_stats.max_hp - enemy.base.current_stats.hp_damage;
    let e_break = enemy.current_enemy_only_stats.break_damage;

    if let Ok(mut hp_node) = gauge_params.p0().single_mut() {
        let ratio = if enemy.base.current_stats.max_hp > 0 {
            (e_hp as f32 / enemy.base.current_stats.max_hp as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        hp_node.width = percent((ratio * 100.0).round());
    }
    if let Ok(mut br_node) = gauge_params.p1().single_mut() {
        let ratio = (e_break as f32 / 100.0).clamp(0.0, 1.0);
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
        let enemy_action_str = if let Some(step) = planned.0.current_step() {
            step.name
        } else {
            "不明"
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
