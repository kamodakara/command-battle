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

// 次ターンに表示される事前決定済み敵行動
#[derive(Resource)]
struct EnemyPlannedAction(EnemyAction);

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

// ================== Setup ==================
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Hp {
            current: 100,
            max: 100,
        },
        Attack(20),
        Stamina {
            current: 100,
            max: 100,
        },
    ));
    commands.spawn((
        Enemy,
        Hp {
            current: 300,
            max: 300,
        },
        Attack(30),
        BreakValue { current: 0 },
        BreakState { remaining_turns: 0 },
        BreakRegen { amount: 1 },
    ));
    commands.insert_resource(BattlePhase::AwaitCommand);
    commands.insert_resource(Turn(1));
    // 初期ログと敵行動決定
    let mut rng = rand::thread_rng();
    let first_action = if rng.gen_bool(0.5) {
        EnemyAction::Attack
    } else {
        EnemyAction::Wait
    };
    commands.insert_resource(CombatLog(vec![
        format!(
            "初期敵行動: {}",
            match first_action {
                EnemyAction::Attack => "攻撃",
                EnemyAction::Wait => "待機",
                EnemyAction::Heal => "回復",
                EnemyAction::ChargeStart => "ため(準備)",
                EnemyAction::ChargeHit => "ため攻撃(発動)",
            }
        ),
        "コマンドを選択してください (A=攻撃 S=スキル H=回復 D=防御 W=待機, Enter=決定)".to_string(),
    ]));
    commands.insert_resource(DefendNextAttack::default());
    commands.insert_resource(CommandQueue::default());
    commands.insert_resource(PlayerChainState::default());
    commands.insert_resource(PendingSelections::default());
    commands.insert_resource(EnemyPlannedAction(first_action));

    const MARGIN: Val = Val::Px(12.);
    // UI Text
    commands
        .spawn((
            UiRoot,
            Node {
                // fill the entire window
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(MARGIN),
                row_gap: MARGIN,
                ..Default::default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new("プレイヤーHP: ???\n敵HP: ???\n\n"),
                TextFont {
                    font: asset_server.load("fonts/x12y16pxMaruMonica.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            builder.spawn((
                Text::new("フェーズ: 初期化中\n\n"),
                TextFont {
                    font: asset_server.load("fonts/x12y16pxMaruMonica.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
            builder.spawn((
                Text::new("ログ:\n"),
                TextFont {
                    font: asset_server.load("fonts/x12y16pxMaruMonica.ttf"),
                    font_size: 18.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
    println!(
        "ゲーム開始: A=攻撃 S=スキル(1.5倍) H=回復(+50) D=防御 W=待機(+スタミナ50回復) / Enter=決定"
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
                added.push("スキル");
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
        if !added.is_empty() {
            log.0.push(format!(
                "選択を追加: {} (現在{}件)",
                added.join(", "),
                pending.0.len()
            ));
        }

        // Enterで確定: 先頭を実行、2つ目以降を予約キューへ
        if keyboard.just_pressed(KeyCode::Enter) && !pending.0.is_empty() {
            // 先頭を今回実行（連撃判定は実行時に行う）
            commands_to_process.push(pending.0[0]);
            // 残りをキューへ（連撃判定は実行時に行う）
            for &cmd in pending.0.iter().skip(1) {
                queue.0.push_back(cmd);
            }
            // ログ出力
            if pending.0.len() > 1 {
                let names = pending
                    .0
                    .iter()
                    .skip(1)
                    .map(|c| match c {
                        CommandKind::Attack => "攻撃",
                        CommandKind::Skill => "スキル",
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
            log.0.push("選択を確定しました".to_string());
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
            CommandKind::Skill => "スキル",
            CommandKind::Heal => "回復",
            CommandKind::Defend => "防御",
            CommandKind::Wait => "待機",
        };
        log.0
            .push(format!("ターン {} プレイヤーは{}を選択", turn.0, name));
        // 連撃判定（直前が攻撃 かつ 今回が攻撃）
        let is_chain = chain_state.last_was_attack && matches!(cmd, CommandKind::Attack);

        // コストチェック（実行時にも確認）。不足なら行動失敗。
        let base_attack_cost = 20;
        let cost = match cmd {
            CommandKind::Attack => {
                if is_chain {
                    base_attack_cost / 2
                } else {
                    base_attack_cost
                }
            }
            CommandKind::Skill => 30,
            CommandKind::Heal => 25,
            CommandKind::Defend => 10,
            CommandKind::Wait => 0,
        };
        if p_sta.current < cost {
            log.0.push("スタミナ不足で行動できませんでした".to_string());
            // 実行失敗なので連撃を継続させない
            chain_state.last_was_attack = false;
        } else {
            p_sta.current -= cost;

            match cmd {
                CommandKind::Heal => {
                    let before = p_hp.current;
                    p_hp.current = (p_hp.current + 50).min(p_hp.max);
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
                    let mut dmg = p_attack.0;
                    // ブレイク中は受けるダメージ2倍
                    if e_bstate.remaining_turns > 0 {
                        dmg *= 2;
                    }
                    e_hp.current = (e_hp.current - dmg).max(0);
                    if is_chain {
                        log.0.push(format!(
                            "連撃! 敵に{}ダメージ (消費スタミナ半減, 敵HP {} / {})",
                            dmg, e_hp.current, e_hp.max
                        ));
                    } else {
                        log.0.push(format!(
                            "敵に{}ダメージ (敵HP {} / {})",
                            dmg, e_hp.current, e_hp.max
                        ));
                    }
                    // ブレイク値加算（与えたダメージ分）
                    e_break.current += dmg;
                    // ダメージを受けたので自然回復量をリセット
                    e_bregen.amount = 1;
                }
                CommandKind::Skill => {
                    let mut dmg = (p_attack.0 as f32 * 1.5).round() as i32;
                    if e_bstate.remaining_turns > 0 {
                        dmg *= 2;
                    }
                    e_hp.current = (e_hp.current - dmg).max(0);
                    log.0.push(format!(
                        "敵に{}ダメージ (敵HP {} / {})",
                        dmg, e_hp.current, e_hp.max
                    ));
                    e_break.current += dmg;
                    e_bregen.amount = 1;
                }
                CommandKind::Wait => {
                    let before = p_sta.current;
                    p_sta.current = (p_sta.current + 50).min(p_sta.max);
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

        // プレイヤーの攻撃/スキル後にブレイク判定。閾値到達でこのターンの敵行動をキャンセルし、次ターンから3ターンブレイク。
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
                match planned.0 {
                    EnemyAction::Attack => {
                        let mut incoming = e_attack.0;
                        if defend_flag.0 {
                            incoming = 0;
                            defend_flag.0 = false; // 一度きり
                        }
                        p_hp.current = (p_hp.current - incoming).max(0);
                        log.0.push(format!(
                            "敵の行動: 攻撃 → {}ダメージ (プレイヤーHP {} / {})",
                            incoming, p_hp.current, p_hp.max
                        ));
                    }
                    EnemyAction::Wait => {
                        log.0.push("敵の行動: 待機 (何もしない)".to_string());
                    }
                    EnemyAction::Heal => {
                        // プレイヤーがこのターンに攻撃していた場合、敵の回復量は半減
                        let base_heal = 50;
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
                            "敵の行動: 回復 → HPを{}回復 (敵HP {} / {})",
                            healed, e_hp.current, e_hp.max
                        ));
                    }
                    EnemyAction::ChargeStart => {
                        // ための準備（何もしない）。次ターンは確定でため攻撃。
                        log.0.push("敵の行動: ため (次ターンに強攻撃)".to_string());
                        planned.0 = EnemyAction::ChargeHit;
                    }
                    EnemyAction::ChargeHit => {
                        // ため攻撃：HP70ダメージ + スタミナ25ダメージ。防御なら無効。
                        let mut hp_damage = 70;
                        let mut sta_damage = 25;
                        if defend_flag.0 {
                            hp_damage = 0;
                            sta_damage = 0;
                            defend_flag.0 = false;
                            log.0.push("プレイヤーの防御でため攻撃は無効化".to_string());
                        }
                        // HPダメージ
                        p_hp.current = (p_hp.current - hp_damage).max(0);
                        // スタミナダメージ
                        p_sta.current = (p_sta.current - sta_damage).max(0);
                        log.0.push(format!(
                        "敵の行動: ため攻撃 → HP{}ダメージ / スタミナ{}ダメージ (HP {} / {}, Stamina {} / {})",
                        hp_damage, sta_damage, p_hp.current, p_hp.max, p_sta.current, p_sta.max
                    ));
                        // 次ターンは通常行動に戻る
                        planned.0 = EnemyAction::Attack;
                    }
                }
            }
        }
        // 次ターンの敵行動を事前決定（敵が生きている場合）
        if e_hp.current > 0 && p_hp.current > 0 {
            // 直前がChargeStartの場合は次はChargeHit（すでに設定済み）。それ以外で決定。
            if !matches!(planned.0, EnemyAction::ChargeHit) {
                // 敵HPが半分以下なら、回復とため開始を選択肢に含める
                let roll: f32 = rand::random::<f32>();
                if e_hp.current * 2 <= e_hp.max {
                    // 攻撃 / 待機 / 回復 / ため(準備)
                    planned.0 = match () {
                        _ if roll < 0.25 => EnemyAction::Attack,
                        _ if roll < 0.50 => EnemyAction::Wait,
                        _ if roll < 0.75 => EnemyAction::Heal,
                        _ => EnemyAction::ChargeStart,
                    };
                } else {
                    // 攻撃 / 待機 / ため(準備)
                    planned.0 = match () {
                        _ if roll < 0.4 => EnemyAction::Attack,
                        _ if roll < 0.8 => EnemyAction::Wait,
                        _ => EnemyAction::ChargeStart,
                    };
                }
            }
            log.0.push(format!(
                "次ターン敵行動予定: {}",
                match planned.0 {
                    EnemyAction::Attack => "攻撃",
                    EnemyAction::Wait => "待機",
                    EnemyAction::Heal => "回復",
                    EnemyAction::ChargeStart => "ため(準備)",
                    EnemyAction::ChargeHit => "ため攻撃(発動)",
                }
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
        // ターン終了時、攻撃/スキルが無ければ自然回復: 1,2,4,...と倍増。0到達またはダメージ受けで1へリセット。
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
    mut ui_q: Query<&mut Children, With<UiRoot>>,
    mut text_q: Query<&mut Text, With<Text>>,
    planned: Res<EnemyPlannedAction>,
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

    for children in ui_q.iter_mut() {
        let mut index = 0;
        for child in children.iter() {
            if text_q.get_mut(child).is_ok() {
                match index {
                    0 => {
                        let child = text_q.get_mut(child);
                        if let Ok(mut text) = child {
                            text.0 = format!(
                                "プレイヤーHP: {} / {}\nスタミナ: {} / {}\n敵HP: {} / {}\n敵ブレイク値: {} / 100\n敵状態: {}\n\n",
                                p_hp.current,
                                p_hp.max,
                                p_sta.current,
                                p_sta.max,
                                e_hp.current,
                                e_hp.max,
                                e_break.current,
                                if e_bstate.remaining_turns > 0 {
                                    "ブレイク中"
                                } else {
                                    "通常"
                                }
                            );
                        }
                    }
                    1 => {
                        let enemy_action_str = match planned.0 {
                            EnemyAction::Attack => "攻撃",
                            EnemyAction::Wait => "待機",
                            EnemyAction::Heal => "回復",
                            EnemyAction::ChargeStart => "ため(準備)",
                            EnemyAction::ChargeHit => "ため攻撃(発動)",
                        };
                        let help = "\n[コマンド説明]\n \
 A=攻撃: 消費20 / ダメージ=攻撃力(20)\n \
 S=スキル: 消費30 / ダメージ=攻撃力×1.5\n \
 H=回復: 消費25 / HP+50\n \
 D=防御: 消費10 / 次の敵攻撃を無効化\n \
 W=待機: 消費0 / スタミナ+50";
                        let phase_str = match *phase {
                            BattlePhase::AwaitCommand => format!(
                                "コマンド入力待ち\n 敵予定行動: {enemy_action_str}\n コマンドを選択してください (A=攻撃 S=スキル H=回復 D=防御 W=待機, Enter=決定){help}"
                            ),
                            BattlePhase::InBattle => "処理中".to_string(),
                            BattlePhase::Finished => "終了".to_string(),
                        };
                        let child = text_q.get_mut(child);
                        if let Ok(mut text) = child {
                            text.0 = format!("フェーズ: {phase_str}\n\n");
                        }
                    }
                    2 => {
                        let mut log_text = String::from("ログ:\n");
                        let start = if log.0.len() > 10 {
                            log.0.len() - 10
                        } else {
                            0
                        };
                        for line in &log.0[start..] {
                            log_text.push_str(line);
                            log_text.push('\n');
                        }
                        let child = text_q.get_mut(child);
                        if let Ok(mut text) = child {
                            text.0 = log_text;
                        }
                    }
                    _ => {}
                }
            }
            index += 1;
        }
    }
}
