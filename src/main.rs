use std::sync::Arc;

use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_input_system,
                battle_end_check_system,
                ui_update_system,
                boss_slain_banner_system,
            ),
        )
        .run();
}

// ================== Components & Resources ==================
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Enemy;
// 敵のブレイク値（0以上）
#[derive(Component)]
struct BreakValue {
    current: i32,
}
// 敵のブレイク状態（残りターン数）
#[derive(Component)]
struct BreakState {
    remaining_turns: u32, // 0なら非ブレイク
}
// ブレイク自然回復の現在量（ターンごとに倍増: 1,2,4,...）
#[derive(Component)]
struct BreakRegen {
    amount: i32, // 最小1
}
#[derive(Component)]
struct Hp {
    current: i32,
    max: i32,
}
#[derive(Component)]
struct Attack(i32);

#[derive(Component)]
struct Stamina {
    current: i32,
    max: i32,
}

#[derive(Resource, PartialEq, Eq)]
enum BattlePhase {
    AwaitCommand,
    InBattle,
    Finished,
}
#[derive(Resource)]
struct Turn(u32);

#[derive(Resource)]
struct CombatLog(Vec<String>);

// モメンタム（最大100）
#[derive(Resource, Default)]
struct Momentum {
    current: i32,
}

// コマンド強化の残りターン
#[derive(Resource, Default)]
struct CommandBuffs {
    attack: u32,
    skill: u32,
    heal: u32,
    defend: u32,
}

// 次の敵攻撃を無効化する防御フラグ
#[derive(Resource, Default)]
struct DefendNextAttack(bool);

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
    EnhanceAttack,
    EnhanceSkill,
    EnhanceHeal,
    EnhanceDefend,
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

// ================== Boss Slain Banner ==================
#[derive(Component)]
struct BossSlainText; // ボス撃破表示用

#[derive(Component)]
struct BossSlainBanner {
    elapsed: f32,
    phase: BannerPhase,
}

enum BannerPhase {
    FadeIn,
    Hold,
    FadeOut,
}

// ================== Setup ==================
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Hp {
            current: 100,
            max: 100,
        },
        Attack(10),
        Stamina {
            current: 100,
            max: 100,
        },
    ));
    commands.spawn((
        Enemy,
        Hp {
            current: 1500,
            max: 1500,
        },
        Attack(40),
        BreakValue { current: 0 },
        BreakState { remaining_turns: 0 },
        BreakRegen { amount: 1 },
    ));
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
    commands.insert_resource(DefendNextAttack::default());
    commands.insert_resource(CommandQueue::default());
    commands.insert_resource(PlayerChainState::default());
    commands.insert_resource(PendingSelections::default());
    commands.insert_resource(EnemyPlannedAction(first_action));
    commands.insert_resource(Momentum::default());
    commands.insert_resource(CommandBuffs::default());

    const MARGIN: Val = Val::Px(12.);
    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");
    // UI Text
    commands
        .spawn((
            UiRoot,
            Node {
                // fill the entire window
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                row_gap: MARGIN,
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Node {
                        // fill the entire window
                        width: Val::Px(500.0),
                        height: percent(100),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        row_gap: Val::Px(0.0),
                        margin: UiRect {
                            left: MARGIN,
                            ..default()
                        },
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        UiStatus,
                        Text::new("プレイヤーHP: ???\n敵HP: ???\n\n"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    builder.spawn((
                        Text::new(
                            "[コマンド説明]\n \
 攻撃:   基本 消費15 威力10 / 連撃時: 消費5 / ブレイク中ダメージ: 通常15・強化時25\n \
 強攻撃: 基本 消費30 威力 30 / 強化中: 威力45 ブレイク+40\n \
 回復:   基本 消費15 回復 50 / 強化中: 消費20 / 回復 60\n \
 防御:   基本 消費10 次の敵攻撃を無効化 / 強化中: 消費5\n \
 待機:   消費0 / スタミナ+50 (強化不可)\n \
 各強化: モメンタム50消費 2ターン持続\n\n",
                        ),
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    builder.spawn((
                        Text::new("[有効値]\n"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    // 有効値の各行（個別に色を切り替える）
                    builder.spawn((
                        UiEffAttack,
                        Text::new("攻撃 力: ?? 消費: ?? (連撃時半減)\n"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    builder.spawn((
                        UiEffSkill,
                        Text::new("強攻撃 威力: ?? 消費: ?? / ブレイク+??\n"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    builder.spawn((
                        UiEffHeal,
                        Text::new("回復 量: ?? 消費: ??\n"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    builder.spawn((
                        UiEffDefend,
                        Text::new("防御 消費: ??\n\n"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    builder.spawn((
                        UiPhase,
                        Text::new("フェーズ: 初期化中\n\n"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            builder.spawn((
                UiLog,
                Text::new("ログ:\n"),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
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
    mut player_q: Query<(&Attack, &mut Hp), (With<Player>, Without<Enemy>)>,
    mut player_sta_q: Query<&mut Stamina, With<Player>>,
    mut enemy_q: Query<
        (
            &Attack,
            &mut Hp,
            &mut BreakValue,
            &mut BreakState,
            &mut BreakRegen,
        ),
        (With<Enemy>, Without<Player>),
    >,
    mut log: ResMut<CombatLog>,
    mut defend_flag: ResMut<DefendNextAttack>,
    mut queue: ResMut<CommandQueue>,
    mut chain_state: ResMut<PlayerChainState>,
    mut pending: ResMut<PendingSelections>,
    mut planned: ResMut<EnemyPlannedAction>,
    mut momentum: ResMut<Momentum>,
    mut buffs: ResMut<CommandBuffs>,
) {
    if *phase == BattlePhase::Finished {
        return;
    }
    let Ok((p_attack, mut p_hp)) = player_q.single_mut() else {
        return;
    };
    let Ok(mut p_sta) = player_sta_q.single_mut() else {
        return;
    };
    let Ok((e_attack, mut e_hp, mut e_break, mut e_bstate, mut e_bregen)) = enemy_q.single_mut()
    else {
        return;
    };

    // フェーズが待機でない場合は何もしない
    if *phase != BattlePhase::AwaitCommand {
        return;
    }

    // 予約キューがあれば入力をスキップして先頭を実行
    let mut commands_to_process: Vec<CommandKind> = Vec::new();
    if let Some(next) = queue.0.pop_front() {
        commands_to_process.push(next);
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
        // 強化: Z=攻撃強化 X=強攻撃強化 C=回復強化 V=防御強化
        if keyboard.just_pressed(KeyCode::KeyZ) {
            if !at_limit {
                pending.0.push(CommandKind::EnhanceAttack);
                added.push("攻撃強化");
            } else {
                log.0
                    .push("これ以上選択を追加できません (最大3件)".to_string());
            }
        }
        if keyboard.just_pressed(KeyCode::KeyX) {
            if !at_limit {
                pending.0.push(CommandKind::EnhanceSkill);
                added.push("強攻撃強化");
            } else {
                log.0
                    .push("これ以上選択を追加できません (最大3件)".to_string());
            }
        }
        if keyboard.just_pressed(KeyCode::KeyC) {
            if !at_limit {
                pending.0.push(CommandKind::EnhanceHeal);
                added.push("回復強化");
            } else {
                log.0
                    .push("これ以上選択を追加できません (最大3件)".to_string());
            }
        }
        if keyboard.just_pressed(KeyCode::KeyV) {
            if !at_limit {
                pending.0.push(CommandKind::EnhanceDefend);
                added.push("防御強化");
            } else {
                log.0
                    .push("これ以上選択を追加できません (最大3件)".to_string());
            }
        }
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
                    CommandKind::EnhanceAttack => "攻撃強化",
                    CommandKind::EnhanceSkill => "強攻撃強化",
                    CommandKind::EnhanceHeal => "回復強化",
                    CommandKind::EnhanceDefend => "防御強化",
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
            // 連続入力によるモメンタム増加（2件で+5、3件以上で+10、最大100）
            let count = pending.0.len();
            if count >= 2 {
                let inc = if count >= 3 { 10 } else { 5 };
                let before = momentum.current;
                momentum.current = (momentum.current + inc).min(100);
                let gained = momentum.current - before;
                if gained > 0 {
                    log.0.push(format!(
                        "モメンタムが{}増加 ({} → {} / 100)",
                        gained, before, momentum.current
                    ));
                }
            }
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
                        CommandKind::EnhanceAttack => "攻撃強化",
                        CommandKind::EnhanceSkill => "強攻撃強化",
                        CommandKind::EnhanceHeal => "回復強化",
                        CommandKind::EnhanceDefend => "防御強化",
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
            CommandKind::EnhanceAttack => "攻撃強化",
            CommandKind::EnhanceSkill => "強攻撃強化",
            CommandKind::EnhanceHeal => "回復強化",
            CommandKind::EnhanceDefend => "防御強化",
        };
        log.0
            .push(format!("ターン {} プレイヤーは{}を選択", turn.0, name));
        // 連撃判定（直前が攻撃 かつ 今回が攻撃）
        let is_chain = chain_state.last_was_attack && matches!(cmd, CommandKind::Attack);

        // コストチェック（実行時にも確認）。不足なら行動失敗。
        let base_attack_cost = 15;
        let cost = match cmd {
            CommandKind::Attack => {
                if is_chain {
                    5
                } else {
                    base_attack_cost
                }
            }
            CommandKind::Skill => 30,
            CommandKind::Heal => {
                if buffs.heal > 0 {
                    20
                } else {
                    15
                }
            }
            CommandKind::Defend => {
                if buffs.defend > 0 {
                    5
                } else {
                    10
                }
            }
            CommandKind::Wait => 0,
            CommandKind::EnhanceAttack
            | CommandKind::EnhanceSkill
            | CommandKind::EnhanceHeal
            | CommandKind::EnhanceDefend => 0,
        };
        if p_sta.current < cost {
            log.0.push("スタミナ不足で行動できませんでした".to_string());
            // 実行失敗なので連撃を継続させない
            chain_state.last_was_attack = false;
        } else {
            p_sta.current -= cost;

            match cmd {
                CommandKind::EnhanceAttack => {
                    if buffs.attack > 0 {
                        log.0
                            .push("攻撃は既に強化中のため強化できません".to_string());
                    } else if momentum.current < 50 {
                        log.0
                            .push("モメンタム不足で強化できませんでした (必要50)".to_string());
                    } else {
                        momentum.current -= 50;
                        buffs.attack = 3;
                        log.0
                            .push("攻撃を強化した (3ターン持続, モメンタム-50)".to_string());
                    }
                }
                CommandKind::EnhanceSkill => {
                    if buffs.skill > 0 {
                        log.0
                            .push("強攻撃は既に強化中のため強化できません".to_string());
                    } else if momentum.current < 50 {
                        log.0
                            .push("モメンタム不足で強化できませんでした (必要50)".to_string());
                    } else {
                        momentum.current -= 50;
                        buffs.skill = 3;
                        log.0
                            .push("強攻撃を強化した (3ターン持続, モメンタム-50)".to_string());
                    }
                }
                CommandKind::EnhanceHeal => {
                    if buffs.heal > 0 {
                        log.0
                            .push("回復は既に強化中のため強化できません".to_string());
                    } else if momentum.current < 50 {
                        log.0
                            .push("モメンタム不足で強化できませんでした (必要50)".to_string());
                    } else {
                        momentum.current -= 50;
                        buffs.heal = 3;
                        log.0
                            .push("回復を強化した (3ターン持続, モメンタム-50)".to_string());
                    }
                }
                CommandKind::EnhanceDefend => {
                    if buffs.defend > 0 {
                        log.0
                            .push("防御は既に強化中のため強化できません".to_string());
                    } else if momentum.current < 50 {
                        log.0
                            .push("モメンタム不足で強化できませんでした (必要50)".to_string());
                    } else {
                        momentum.current -= 50;
                        buffs.defend = 3;
                        log.0
                            .push("防御を強化した (3ターン持続, モメンタム-50)".to_string());
                    }
                }
                CommandKind::Heal => {
                    let amount = if buffs.heal > 0 { 60 } else { 50 };
                    let before = p_hp.current;
                    p_hp.current = (p_hp.current + amount).min(p_hp.max);
                    let healed = p_hp.current - before;
                    log.0.push(format!(
                        "プレイヤーは{}回復 (HP {} / {})",
                        healed, p_hp.current, p_hp.max
                    ));
                }
                CommandKind::Defend => {
                    defend_flag.0 = true;
                    log.0
                        .push("プレイヤーは防御態勢に入った (次の敵攻撃は無効)".to_string());
                }
                CommandKind::Attack => {
                    let base = if buffs.attack > 0 { 25 } else { p_attack.0 };
                    let mut dmg = base;
                    let mut break_bonus = 0;
                    if e_bstate.remaining_turns > 0 {
                        dmg *= 2;
                        break_bonus = dmg - base;
                    }
                    e_hp.current = (e_hp.current - dmg).max(0);
                    if is_chain {
                        if break_bonus > 0 {
                            log.0.push(format!(
                                "連撃! 敵に{}ダメージ (基本{} + ブレイク補正{} = 合計{}, 消費スタミナ半減, 敵HP {} / {})",
                                dmg, base, break_bonus, dmg, e_hp.current, e_hp.max
                            ));
                        } else {
                            log.0.push(format!(
                                "連撃! 敵に{}ダメージ (消費スタミナ半減, 敵HP {} / {})",
                                dmg, e_hp.current, e_hp.max
                            ));
                        }
                    } else {
                        if break_bonus > 0 {
                            log.0.push(format!(
                                "敵に{}ダメージ (基本{} + ブレイク補正{} = 合計{}, 敵HP {} / {})",
                                dmg, base, break_bonus, dmg, e_hp.current, e_hp.max
                            ));
                        } else {
                            log.0.push(format!(
                                "敵に{}ダメージ (敵HP {} / {})",
                                dmg, e_hp.current, e_hp.max
                            ));
                        }
                    }
                    // ブレイク値加算（攻撃時の固定増加量: 通常15・強化時25）
                    let before_break = e_break.current;
                    let add_break = if buffs.attack > 0 { 25 } else { 15 };
                    e_break.current += add_break;
                    log.0.push(format!(
                        "ブレイク値 +{} ({} → {} / 100)",
                        add_break, before_break, e_break.current
                    ));
                    // ダメージを受けたので自然回復量をリセット
                    e_bregen.amount = 1;
                }
                CommandKind::Skill => {
                    let base = if buffs.skill > 0 { 45 } else { 30 };
                    let mut dmg = base;
                    let mut break_bonus = 0;
                    if e_bstate.remaining_turns > 0 {
                        dmg *= 2;
                        break_bonus = dmg - base;
                    }
                    e_hp.current = (e_hp.current - dmg).max(0);
                    if break_bonus > 0 {
                        log.0.push(format!(
                            "敵に{}ダメージ (基本{} + ブレイク補正{} = 合計{}, 敵HP {} / {})",
                            dmg, base, break_bonus, dmg, e_hp.current, e_hp.max
                        ));
                    } else {
                        log.0.push(format!(
                            "敵に{}ダメージ (敵HP {} / {})",
                            dmg, e_hp.current, e_hp.max
                        ));
                    }
                    let before_break = e_break.current;
                    let add_break = if buffs.skill > 0 { 40 } else { dmg };
                    e_break.current += add_break;
                    log.0.push(format!(
                        "ブレイク値 +{} ({} → {} / 100)",
                        add_break, before_break, e_break.current
                    ));
                    e_bregen.amount = 1;
                }
                CommandKind::Wait => {
                    let before = p_sta.current;
                    p_sta.current = (p_sta.current + 40).min(p_sta.max);
                    let recovered = p_sta.current - before;
                    log.0.push(format!(
                        "プレイヤーは待機してスタミナを{}回復 (Stamina {} / {})",
                        recovered, p_sta.current, p_sta.max
                    ));
                }
            }
            // 実行成功: 直前が攻撃だったかを更新
            chain_state.last_was_attack = matches!(cmd, CommandKind::Attack);
        }

        // プレイヤーの攻撃/強攻撃後にブレイク判定。閾値到達でこのターンの敵行動をキャンセルし、次ターンから3ターンブレイク。
        let mut enemy_action_canceled_this_turn = false;
        if e_break.current >= 100 && e_bstate.remaining_turns == 0 {
            enemy_action_canceled_this_turn = true;
            e_bstate.remaining_turns = 3; // 次ターンから3ターン行動不能
            log.0.push(
                "敵がブレイク状態に入る!（次のターンから3ターン行動不能・被ダメ2倍）".to_string(),
            );
        }

        if e_hp.current > 0 {
            // 事前決定済みの敵行動を実行
            if e_bstate.remaining_turns > 0 {
                // ブレイク中は行動不能
                log.0.push("敵はブレイク中のため行動不能".to_string());
            } else if enemy_action_canceled_this_turn {
                // このターンの行動はキャンセル
                log.0.push("敵の行動はブレイクによりキャンセル".to_string());
            } else {
                let action = &mut planned.0;
                let step = action.current_step().unwrap();
                match step.specification {
                    ActionStepSpecificationEnum::Attack(spec) => {
                        let mut incoming = (e_attack.0 as f32 * spec.power) as i32;
                        if defend_flag.0 {
                            incoming = 0;
                            defend_flag.0 = false; // 一度きり
                        }
                        p_hp.current = (p_hp.current - incoming).max(0);
                        log.0.push(format!(
                            "敵の行動: {} → {}ダメージ (プレイヤーHP {} / {})",
                            step.name, incoming, p_hp.current, p_hp.max
                        ));
                    }
                    ActionStepSpecificationEnum::Wait(_) => {
                        log.0.push(format!("敵の行動: {} (何もしない)", step.name));
                    }
                    ActionStepSpecificationEnum::Heal(spec) => {
                        // プレイヤーがこのターンに攻撃していた場合、敵の回復量は半減
                        let base_heal = spec.amount;
                        let heal_amount = if matches!(cmd, CommandKind::Attack | CommandKind::Skill)
                        {
                            base_heal / 2
                        } else {
                            base_heal
                        };
                        let before = e_hp.current;
                        e_hp.current = (e_hp.current + heal_amount).min(e_hp.max);
                        let healed = e_hp.current - before;
                        log.0.push(format!(
                            "敵の行動: {} → HPを{}回復 (敵HP {} / {})",
                            step.name, healed, e_hp.current, e_hp.max
                        ));
                    }
                }
                action.next();
            }
        }
        // 次ターンの敵行動を事前決定（敵が生きている場合）
        if e_hp.current > 0 && p_hp.current > 0 {
            if planned.0.is_finished() {
                // 現在の行動が完了している場合、新たに行動を決定

                let roll: f32 = rand::random::<f32>();
                // 敵HPが半分以下なら、回復とため開始を選択肢に含める
                let next = if e_hp.current * 2 <= e_hp.max {
                    // 攻撃 / 待機 / 回復 / ため(準備)
                    match () {
                        _ if roll < 0.1 => create_enemy_wait(),
                        _ if roll < 0.2 => create_enemy_heal(),
                        _ if roll < 0.3 => create_enemy_attack(),
                        _ if roll < 0.5 => create_enemy_claw_combo_strong(),
                        _ if roll < 0.7 => create_enemy_claw_strong(),
                        _ if roll < 0.8 => create_enemy_stomp(),
                        _ => create_enemy_fire_breath(),
                    }
                } else {
                    match () {
                        _ if roll < 0.3 => create_enemy_wait(),
                        _ if roll < 0.6 => create_enemy_attack(),
                        _ if roll < 0.8 => create_enemy_claw_combo(),
                        _ if roll < 0.9 => create_enemy_claw_strong(),
                        _ => create_enemy_stomp(),
                    }
                };

                // TODO: 毎回生成してるのやめる
                planned.0 = ActionProcess::from(&Arc::new(next));
            }
            log.0.push(format!(
                "次ターン敵行動予定: {}",
                planned.0.current_step().unwrap().name
            ));
        }
        // ターン終了時、ブレイク残りターンのデクリメント（ブレイク中のみ）。解除時にブレイク値リセット。
        if e_bstate.remaining_turns > 0 {
            e_bstate.remaining_turns = e_bstate.remaining_turns.saturating_sub(1);
            if e_bstate.remaining_turns == 0 {
                e_break.current = 0;
                log.0
                    .push("敵のブレイク状態が解除。ブレイク値を0にリセット".to_string());
                // 0になったので自然回復量もリセット
                e_bregen.amount = 1;
            }
        }
        // ターン終了時、攻撃/強攻撃が無ければ自然回復: 1,2,4,...と倍増。0到達またはダメージ受けで1へリセット。
        if !matches!(cmd, CommandKind::Attack | CommandKind::Skill) {
            let before = e_break.current;
            e_break.current = (e_break.current - e_bregen.amount).max(0);
            if e_break.current != before {
                log.0.push(format!(
                    "敵のブレイク値が自然回復: {} → {} (回復量 {})",
                    before, e_break.current, e_bregen.amount
                ));
            }
            if e_break.current == 0 {
                e_bregen.amount = 1;
            } else {
                e_bregen.amount = (e_bregen.amount * 2).max(1);
            }
        }
        // ターン終了時、強化の残りターンをデクリメント
        let prev = (buffs.attack, buffs.skill, buffs.heal, buffs.defend);
        if buffs.attack > 0 {
            buffs.attack -= 1;
            if buffs.attack == 0 && prev.0 > 0 {
                log.0.push("攻撃の強化が解除された".to_string());
            }
        }
        if buffs.skill > 0 {
            buffs.skill -= 1;
            if buffs.skill == 0 && prev.1 > 0 {
                log.0.push("強攻撃の強化が解除された".to_string());
            }
        }
        if buffs.heal > 0 {
            buffs.heal -= 1;
            if buffs.heal == 0 && prev.2 > 0 {
                log.0.push("回復の強化が解除された".to_string());
            }
        }
        if buffs.defend > 0 {
            buffs.defend -= 1;
            if buffs.defend == 0 && prev.3 > 0 {
                log.0.push("防御の強化が解除された".to_string());
            }
        }
        turn.0 += 1;
        *phase = BattlePhase::AwaitCommand;
    };

    // 今回は1件だけ処理（各ターン1コマンドのルール）
    resolve_command(commands_to_process[0]);
}

// ================== End Check ==================
fn battle_end_check_system(
    mut phase: ResMut<BattlePhase>,
    player_q: Query<&Hp, With<Player>>,
    enemy_q: Query<&Hp, With<Enemy>>,
    mut log: ResMut<CombatLog>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if *phase == BattlePhase::Finished {
        return;
    }
    let Ok(p_hp) = player_q.single() else {
        return;
    };
    let Ok(e_hp) = enemy_q.single() else {
        return;
    };
    if e_hp.current <= 0 {
        *phase = BattlePhase::Finished;
        log.0.push("勝利! 敵を倒しました".to_string());

        let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");
        commands
            .spawn((
                BossSlainBanner {
                    elapsed: 0.0,
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
            ))
            .with_children(|builder| {
                builder.spawn((
                    BossSlainText,
                    Text::new("DORAGON SLAIN"),
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
                ));
            });
    } else if p_hp.current <= 0 {
        *phase = BattlePhase::Finished;
        log.0.push("敗北... プレイヤーのHPが0です".to_string());
    }
}

fn ui_update_system(
    player_q: Query<&Hp, With<Player>>,
    player_sta_q: Query<&Stamina, With<Player>>,
    enemy_q: Query<(&Hp, &BreakValue, &BreakState), With<Enemy>>,
    phase: Res<BattlePhase>,
    log: Res<CombatLog>,
    momentum: Res<Momentum>,
    buffs: Res<CommandBuffs>,
    pending: Res<PendingSelections>,
    // mut ui_q: Query<&mut Children, With<UiRoot>>,
    // mut text_q: Query<&mut Text, With<Text>>,
    planned: Res<EnemyPlannedAction>,
    mut ui_staus_q: Query<&mut Text, (With<UiStatus>, Without<UiPhase>, Without<UiLog>)>,
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
    let Ok(p_hp) = player_q.single() else {
        return;
    };
    let Ok(p_sta) = player_sta_q.single() else {
        return;
    };
    let Ok((e_hp, e_break, e_bstate)) = enemy_q.single() else {
        return;
    };
    let Ok(mut ui_status_text) = ui_staus_q.single_mut() else {
        return;
    };
    let Ok(mut ui_phase_text) = ui_phase_q.single_mut() else {
        return;
    };
    let Ok(mut ui_log_text) = ui_log_q.single_mut() else {
        return;
    };

    // 強化反映後の有効値
    let atk_power = if buffs.attack > 0 { 25 } else { 10 };
    let skl_power = if buffs.skill > 0 { 45 } else { 30 };
    let heal_amount = if buffs.heal > 0 { 60 } else { 50 };
    let atk_cost = 15;
    let skl_cost = 30; // 強化時も消費は変えない指定
    let heal_cost = if buffs.heal > 0 { 20 } else { 15 };
    let def_cost = if buffs.defend > 0 { 5 } else { 10 };

    ui_status_text.0 = format!(
        "プレイヤーHP: {} / {}\nスタミナ: {} / {}\nモメンタム: {} / 100\n強化 残り(攻:{} 強:{} 回:{} 防:{})\n\n敵HP: {} / {}\n敵ブレイク値: {} / 100\n敵状態: {}\n\n",
        p_hp.current,
        p_hp.max,
        p_sta.current,
        p_sta.max,
        momentum.current,
        buffs.attack,
        buffs.skill,
        buffs.heal,
        buffs.defend,
        e_hp.current,
        e_hp.max,
        e_break.current,
        if e_bstate.remaining_turns > 0 {
            "ブレイク中"
        } else {
            "通常"
        },
    );

    // 有効値（コマンド別）テキスト更新＆色切り替え
    let Ok((mut eff_atk_text, mut eff_atk_color)) = ui_eff_atk_q.single_mut() else {
        return;
    };
    eff_atk_text.0 = format!("攻撃 力:{} 消費:{} (連撃時消費5)\n", atk_power, atk_cost);
    eff_atk_color.0 = if buffs.attack > 0 {
        Color::from(LinearRgba {
            red: 0.95,
            green: 0.85,
            blue: 0.35,
            alpha: 1.0,
        })
    } else {
        Color::WHITE
    };

    let Ok((mut eff_skl_text, mut eff_skl_color)) = ui_eff_skl_q.single_mut() else {
        return;
    };
    eff_skl_text.0 = format!(
        "強攻撃 威力:{} 消費:{} / ブレイク+{}\n",
        skl_power,
        skl_cost,
        if buffs.skill > 0 { 40 } else { skl_power }
    );
    eff_skl_color.0 = if buffs.skill > 0 {
        Color::from(LinearRgba {
            red: 0.95,
            green: 0.85,
            blue: 0.35,
            alpha: 1.0,
        })
    } else {
        Color::WHITE
    };

    let Ok((mut eff_heal_text, mut eff_heal_color)) = ui_eff_heal_q.single_mut() else {
        return;
    };
    eff_heal_text.0 = format!("回復 量:{} 消費:{}\n", heal_amount, heal_cost);
    eff_heal_color.0 = if buffs.heal > 0 {
        Color::from(LinearRgba {
            red: 0.95,
            green: 0.85,
            blue: 0.35,
            alpha: 1.0,
        })
    } else {
        Color::WHITE
    };

    let Ok((mut eff_def_text, mut eff_def_color)) = ui_eff_def_q.single_mut() else {
        return;
    };
    eff_def_text.0 = format!("防御 消費:{}\n\n", def_cost);
    eff_def_color.0 = if buffs.defend > 0 {
        Color::from(LinearRgba {
            red: 0.95,
            green: 0.85,
            blue: 0.35,
            alpha: 1.0,
        })
    } else {
        Color::WHITE
    };

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
                CommandKind::Skill => "スキル",
                CommandKind::Heal => "回復",
                CommandKind::Defend => "防御",
                CommandKind::Wait => "待機",
                CommandKind::EnhanceAttack => "攻撃強化",
                CommandKind::EnhanceSkill => "スキル強化",
                CommandKind::EnhanceHeal => "回復強化",
                CommandKind::EnhanceDefend => "防御強化",
            })
            .collect::<Vec<_>>()
            .join(", ")
    };
    let phase_str = match *phase {
        BattlePhase::AwaitCommand => format!(
            "コマンド入力待ち    敵の次の行動: {enemy_action_str}\nコマンドを選択してください(最大3つ)\n A=攻撃 S=強攻撃 H=回復 D=防御 W=待機\n Z=攻撃強化 / X=強攻撃強化 / C=回復強化 / V=防御強化\n Backspace=直前取り消し / Esc=全クリア\n Enter=決定\n [選択中] {selected_str}"
        ),
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

// （演出簡易版につきフェード等の更新システムは未実装）
fn boss_slain_banner_system(
    time: Res<Time>,
    mut commands: Commands,
    mut banner_q: Query<(Entity, &mut BossSlainBanner, &Children)>,
    mut text_colors: Query<&mut TextColor, With<BossSlainText>>,
) {
    const FADE_IN: f32 = 1.0;
    const HOLD: f32 = 1.5;
    const FADE_OUT: f32 = 1.0;

    for (entity, mut banner, children) in banner_q.iter_mut() {
        banner.elapsed += time.delta().as_secs_f32();
        match banner.phase {
            BannerPhase::FadeIn => {
                let alpha = (banner.elapsed / FADE_IN).clamp(0.0, 1.0);
                for i in 0..children.len() {
                    let child = children[i];
                    if let Ok(mut c) = text_colors.get_mut(child) {
                        c.0 = Color::from(LinearRgba {
                            red: 0.83,
                            green: 0.72,
                            blue: 0.20,
                            alpha,
                        });
                    }
                }
                if banner.elapsed >= FADE_IN {
                    banner.phase = BannerPhase::Hold;
                    banner.elapsed = 0.0;
                }
            }
            BannerPhase::Hold => {
                for i in 0..children.len() {
                    let child = children[i];
                    if let Ok(mut c) = text_colors.get_mut(child) {
                        c.0 = Color::from(LinearRgba {
                            red: 0.83,
                            green: 0.72,
                            blue: 0.20,
                            alpha: 1.0,
                        });
                    }
                }
                if banner.elapsed >= HOLD {
                    banner.phase = BannerPhase::FadeOut;
                    banner.elapsed = 0.0;
                }
            }
            BannerPhase::FadeOut => {
                let alpha = 1.0 - (banner.elapsed / FADE_OUT).clamp(0.0, 1.0);
                for i in 0..children.len() {
                    let child = children[i];
                    if let Ok(mut c) = text_colors.get_mut(child) {
                        c.0 = Color::from(LinearRgba {
                            red: 0.83,
                            green: 0.72,
                            blue: 0.20,
                            alpha,
                        });
                    }
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
