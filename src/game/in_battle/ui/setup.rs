use bevy::prelude::*;
use super::resources::*;
use super::handlers::input::BackToPreparationButton;
use super::renderers::{
    player_status::*,
    enemy_status::{UiEnemy, UiEnemyHpGaugeFill, UiEnemyBreakGaugeFill, UiEnemyBreakLabel, UiEnemyHintText},
    action_menu::{UiActionMenu, UiActionMenuContainer},
    combat_log::{UiMessage, UiCombatLogToggleButton, UiCombatLogToggleButtonText},
    karma_cards::{
        UiKarmaCardsContainer, UiKarmaDeckButton, UiKarmaDeckCount,
        UiKarmaDiscardButton, UiKarmaDiscardCount,
        UiKarmaDialog, UiKarmaDialogTitle, UiKarmaDialogContent, UiKarmaDialogCloseButton,
    },
    damage_popup::UiEnemyDamageText,
    turn_action_board::{UiTurnActionBoard, UiActionBoardPlayerText, UiActionBoardEnemyText},
};

/// バトル画面全体のマーカー（クリーンアップ用）
#[derive(Component)]
pub struct BattleScreen;

pub fn setup_battle_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(CombatLog(vec!["行動を選択してください".to_string()]));
    commands.insert_resource(MessageQueue::default());
    commands.insert_resource(EnemyDamagePopup::default());
    commands.insert_resource(KarmaCardsNeedsRedraw(true));
    commands.insert_resource(KarmaDialogState::default());
    commands.insert_resource(ActionMenuSelection::default());
    commands.insert_resource(ConsecutiveCommands::default());
    commands.insert_resource(TurnActionBoard::default());
    commands.insert_resource(CombatLogExpanded::default());

    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    // 右下：ターン行動ボード＋ログメッセージ（縦積みコンテナ）
    let label_color = TextColor(Color::from(LinearRgba { red: 0.55, green: 0.55, blue: 0.55, alpha: 1.0 }));
    commands
        .spawn((
            BattleScreen,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(12.0),
                bottom: Val::Px(12.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ZIndex(10),
        ))
        .with_children(|col| {
            // ターン行動ボード（上段・実行中のみ表示）
            col.spawn((
                UiTurnActionBoard,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    border: UiRect::all(Val::Px(1.0)),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::from(LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.7 })),
                BorderColor::all(Color::WHITE),
            ))
            .with_children(|panel| {
                // ヘッダー行
                panel
                    .spawn((Node {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },))
                    .with_children(|row| {
                        row.spawn((
                            Node { width: Val::Px(36.0), ..default() },
                            Text::new(""),
                            TextFont { font: font.clone(), font_size: 13.0, ..default() },
                            label_color,
                        ));
                        row.spawn((
                            Node { width: Val::Px(120.0), ..default() },
                            Text::new("プレイヤー"),
                            TextFont { font: font.clone(), font_size: 13.0, ..default() },
                            label_color,
                        ));
                        row.spawn((
                            Text::new("敵"),
                            TextFont { font: font.clone(), font_size: 13.0, ..default() },
                            label_color,
                        ));
                    });

                // 3スロット行
                for i in 0..3usize {
                    panel
                        .spawn((Node {
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        },))
                        .with_children(|row| {
                            row.spawn((
                                Node { width: Val::Px(36.0), ..default() },
                                Text::new(format!("{}回目", i + 1)),
                                TextFont { font: font.clone(), font_size: 13.0, ..default() },
                                label_color,
                            ));
                            row.spawn((
                                UiActionBoardPlayerText(i),
                                Node { width: Val::Px(120.0), ..default() },
                                Text::new("---"),
                                TextFont { font: font.clone(), font_size: 14.0, ..default() },
                                TextColor(Color::WHITE),
                            ));
                            row.spawn((
                                UiActionBoardEnemyText(i),
                                Text::new("？？？"),
                                TextFont { font: font.clone(), font_size: 14.0, ..default() },
                                TextColor(Color::from(LinearRgba {
                                    red: 0.70,
                                    green: 0.55,
                                    blue: 0.30,
                                    alpha: 1.0,
                                })),
                            ));
                        });
                }
            });

            // ログメッセージ（下段）
            col.spawn((Node {
                width: Val::Px(750.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },))
            .with_children(|log_col| {
                // トグルボタン
                log_col.spawn((
                    UiCombatLogToggleButton,
                    Button,
                    Node {
                        width: Val::Px(90.0),
                        padding: UiRect::axes(Val::Px(6.0), Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::FlexEnd,
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba { red: 0.2, green: 0.2, blue: 0.2, alpha: 0.9 })),
                    BorderColor::all(Color::WHITE),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        UiCombatLogToggleButtonText,
                        Text::new("▼ 開く"),
                        TextFont { font: font.clone(), font_size: 13.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
                // ログ本文
                log_col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        border: UiRect::all(Val::Px(1.0)),
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.6 })),
                    BorderColor::all(Color::WHITE),
                ))
                .with_children(|log_box| {
                    log_box.spawn((
                        UiMessage,
                        Text::new(""),
                        TextFont { font: font.clone(), font_size: 16.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
            });
        });

    // 敵UI（中央配置）
    let dragon = asset_server.load("images/dragon.png");
    commands
        .spawn((
            BattleScreen,
            UiEnemy,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ZIndex(0),
        ))
        .with_children(|center| {
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
                            // HPゲージ行
                            col.spawn((Node {
                                width: Val::Auto,
                                height: Val::Auto,
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(12.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },))
                                .with_children(|row| {
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
                                                width: Val::Percent(0.0),
                                                height: Val::Percent(100.0),
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

                            // ブレイク行
                            col.spawn((Node {
                                width: Val::Auto,
                                height: Val::Auto,
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(8.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },))
                                .with_children(|row| {
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
                                                width: Val::Percent(0.0),
                                                height: Val::Percent(100.0),
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

                            // ヒントテキスト（ブレイクゲージ下）
                            col.spawn((
                                UiEnemyHintText,
                                Text::new(""),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::from(LinearRgba {
                                    red: 0.95,
                                    green: 0.85,
                                    blue: 0.55,
                                    alpha: 1.0,
                                })),
                                Visibility::Hidden,
                            ));

                        });
                });
        });

    // 左上：ステータス＋カルマ（縦積みラッパー）
    commands
        .spawn((
            BattleScreen,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(16.0),
                top: Val::Px(16.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ZIndex(10),
        ))
        .with_children(|left_col| {

    // ── ステータスパネル
    left_col
        .spawn((
            UiPlayerStatus,
            Node {
                width: Val::Px(280.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            BorderColor::all(Color::WHITE),
        ))
        .with_children(|col| {
            col.spawn((
                UiHpText,
                Text::new("HP: --- / ---"),
                TextFont { font: font.clone(), font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
            ));
            col.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(12.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::from(LinearRgba { red: 0.15, green: 0.15, blue: 0.15, alpha: 1.0 })),
                BorderColor::all(Color::WHITE),
            ))
            .with_children(|g| {
                g.spawn((
                    UiHpGaugeFill,
                    Node { width: Val::Percent(0.0), height: Val::Percent(100.0), ..default() },
                    BackgroundColor(Color::from(LinearRgba { red: 0.80, green: 0.20, blue: 0.20, alpha: 1.0 })),
                ));
            });

            col.spawn((
                UiStaText,
                Text::new("スタミナ: --- / ---"),
                TextFont { font: font.clone(), font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
            ));
            col.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(12.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::from(LinearRgba { red: 0.15, green: 0.15, blue: 0.15, alpha: 1.0 })),
                BorderColor::all(Color::WHITE),
            ))
            .with_children(|g| {
                g.spawn((
                    UiStaGaugeFill,
                    Node { width: Val::Percent(0.0), height: Val::Percent(100.0), ..default() },
                    BackgroundColor(Color::from(LinearRgba { red: 0.20, green: 0.70, blue: 0.25, alpha: 1.0 })),
                ));
            });

            col.spawn((
                UiSpText,
                Text::new("SP: --- / ---"),
                TextFont { font: font.clone(), font_size: 16.0, ..default() },
                TextColor(Color::from(LinearRgba { red: 0.40, green: 0.60, blue: 1.00, alpha: 1.0 })),
            ));
            col.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(12.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::from(LinearRgba { red: 0.15, green: 0.15, blue: 0.15, alpha: 1.0 })),
                BorderColor::all(Color::from(LinearRgba { red: 0.40, green: 0.60, blue: 1.00, alpha: 1.0 })),
            ))
            .with_children(|g| {
                g.spawn((
                    UiSpGaugeFill,
                    Node { width: Val::Percent(0.0), height: Val::Percent(100.0), ..default() },
                    BackgroundColor(Color::from(LinearRgba { red: 0.30, green: 0.50, blue: 0.90, alpha: 1.0 })),
                ));
            });

            // // トランス
            // col.spawn((Node {
            //     flex_direction: FlexDirection::Row,
            //     justify_content: JustifyContent::SpaceBetween,
            //     ..default()
            // },))
            //     .with_children(|row| {
            //         row.spawn((
            //             UiTranceText,
            //             Text::new("トランス: --- / ---"),
            //             TextFont { font: font.clone(), font_size: 16.0, ..default() },
            //             TextColor(Color::from(LinearRgba { red: 0.90, green: 0.60, blue: 0.90, alpha: 1.0 })),
            //         ));
            //         row.spawn((
            //             UiTranceLevelText,
            //             Text::new("Lv.0"),
            //             TextFont { font: font.clone(), font_size: 16.0, ..default() },
            //             TextColor(Color::from(LinearRgba { red: 1.0, green: 0.85, blue: 0.30, alpha: 1.0 })),
            //         ));
            //     });
            // col.spawn((
            //     Node {
            //         width: Val::Percent(100.0),
            //         height: Val::Px(12.0),
            //         border: UiRect::all(Val::Px(1.0)),
            //         ..default()
            //     },
            //     BackgroundColor(Color::from(LinearRgba { red: 0.15, green: 0.15, blue: 0.15, alpha: 1.0 })),
            //     BorderColor::all(Color::from(LinearRgba { red: 0.80, green: 0.50, blue: 0.80, alpha: 1.0 })),
            // ))
            // .with_children(|g| {
            //     g.spawn((
            //         UiTranceGaugeFill,
            //         Node { width: Val::Percent(0.0), height: Val::Percent(100.0), ..default() },
            //         BackgroundColor(Color::from(LinearRgba { red: 0.75, green: 0.30, blue: 0.85, alpha: 1.0 })),
            //     ));
            // });
            // col.spawn((
            //     UiTranceEffectText,
            //     Text::new("効果: なし"),
            //     TextFont { font: font.clone(), font_size: 14.0, ..default() },
            //     TextColor(Color::from(LinearRgba { red: 0.70, green: 0.70, blue: 0.90, alpha: 1.0 })),
            // ));

            // 状態異常ゲージ（折り返しレイアウト・蓄積値0は非表示）
            const AILMENT_NAMES: [&str; 8] =
                ["毒", "眠気", "寒気", "出血", "火傷", "麻痺", "恐怖", "激昂"];
            const AILMENT_INIT_COLORS: [LinearRgba; 8] = [
                LinearRgba { red: 0.29, green: 0.05, blue: 0.34, alpha: 1.0 },
                LinearRgba { red: 0.09, green: 0.18, blue: 0.38, alpha: 1.0 },
                LinearRgba { red: 0.07, green: 0.34, blue: 0.41, alpha: 1.0 },
                LinearRgba { red: 0.41, green: 0.05, blue: 0.07, alpha: 1.0 },
                LinearRgba { red: 0.43, green: 0.23, blue: 0.05, alpha: 1.0 },
                LinearRgba { red: 0.41, green: 0.38, blue: 0.07, alpha: 1.0 },
                LinearRgba { red: 0.36, green: 0.07, blue: 0.27, alpha: 1.0 },
                LinearRgba { red: 0.41, green: 0.11, blue: 0.05, alpha: 1.0 },
            ];
            col.spawn((Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(8.0),
                row_gap: Val::Px(4.0),
                width: Val::Percent(100.0),
                ..default()
            },))
            .with_children(|wrap| {
                for idx in 0..8usize {
                    wrap.spawn((
                        UiStatusAilmentContainer(idx),
                        Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(4.0),
                            display: Display::None, // 初期非表示（蓄積値0）
                            ..default()
                        },
                    ))
                    .with_children(|item| {
                        item.spawn((
                            Node { width: Val::Px(36.0), ..default() },
                            Text::new(AILMENT_NAMES[idx]),
                            TextFont { font: font.clone(), font_size: 13.0, ..default() },
                            TextColor(Color::from(LinearRgba {
                                red: 0.60,
                                green: 0.60,
                                blue: 0.60,
                                alpha: 1.0,
                            })),
                        ));
                        item.spawn((
                            Node {
                                width: Val::Px(80.0),
                                height: Val::Px(8.0),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(Color::from(LinearRgba {
                                red: 0.12,
                                green: 0.12,
                                blue: 0.12,
                                alpha: 1.0,
                            })),
                            BorderColor::all(Color::from(LinearRgba {
                                red: 0.38,
                                green: 0.38,
                                blue: 0.38,
                                alpha: 1.0,
                            })),
                        ))
                        .with_children(|g| {
                            g.spawn((
                                UiStatusAilmentGaugeFill(idx),
                                Node {
                                    width: Val::Percent(0.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(Color::from(AILMENT_INIT_COLORS[idx])),
                            ));
                        });
                    });
                }
            });

        }); // ステータスパネル end

    // ── カルマエリアパネル
    left_col
        .spawn((
            Node {
                width: Val::Px(280.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::from(LinearRgba { red: 0.05, green: 0.04, blue: 0.08, alpha: 1.0 })),
            BorderColor::all(Color::from(LinearRgba { red: 0.65, green: 0.50, blue: 0.25, alpha: 1.0 })),
        ))
        .with_children(|col| {
            // ヘッダー行（[カルマ] + 山札/捨て札ボタン）
            col.spawn((Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                ..default()
            },))
            .with_children(|row| {
                row.spawn((
                    Text::new("[カルマ]"),
                    TextFont { font: font.clone(), font_size: 14.0, ..default() },
                    TextColor(Color::from(LinearRgba { red: 0.95, green: 0.80, blue: 0.40, alpha: 1.0 })),
                ));
                row.spawn((Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(6.0),
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|btns| {
                    // 山札ボタン
                    btns.spawn((
                        UiKarmaDeckButton,
                        Button,
                        Node {
                            padding: UiRect::axes(Val::Px(6.0), Val::Px(2.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::from(LinearRgba { red: 0.15, green: 0.12, blue: 0.20, alpha: 1.0 })),
                        BorderColor::all(Color::from(LinearRgba { red: 0.55, green: 0.45, blue: 0.25, alpha: 1.0 })),
                    ))
                    .with_children(|b| {
                        b.spawn((
                            UiKarmaDeckCount,
                            Text::new("山札: 0"),
                            TextFont { font: font.clone(), font_size: 12.0, ..default() },
                            TextColor(Color::from(LinearRgba { red: 0.90, green: 0.80, blue: 0.60, alpha: 1.0 })),
                        ));
                    });
                    // 捨て札ボタン
                    btns.spawn((
                        UiKarmaDiscardButton,
                        Button,
                        Node {
                            padding: UiRect::axes(Val::Px(6.0), Val::Px(2.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::from(LinearRgba { red: 0.15, green: 0.12, blue: 0.20, alpha: 1.0 })),
                        BorderColor::all(Color::from(LinearRgba { red: 0.55, green: 0.45, blue: 0.25, alpha: 1.0 })),
                    ))
                    .with_children(|b| {
                        b.spawn((
                            UiKarmaDiscardCount,
                            Text::new("捨て札: 0"),
                            TextFont { font: font.clone(), font_size: 12.0, ..default() },
                            TextColor(Color::from(LinearRgba { red: 0.90, green: 0.80, blue: 0.60, alpha: 1.0 })),
                        ));
                    });
                });
            });

            // フィールドカルマ
            col.spawn((
                Node { margin: UiRect::top(Val::Px(4.0)), ..default() },
                Text::new("[フィールドカルマ]"),
                TextFont { font: font.clone(), font_size: 13.0, ..default() },
                TextColor(Color::from(LinearRgba { red: 0.95, green: 0.80, blue: 0.40, alpha: 1.0 })),
            ));
            col.spawn((
                UiKarmaCardsContainer,
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..default()
                },
            ));
        }); // カルマエリア end

    }); // 左上ラッパー end

    // カルマダイアログ（全画面オーバーレイ）
    commands
        .spawn((
            BattleScreen,
            UiKarmaDialog,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::from(LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.75 })),
            ZIndex(200),
            Visibility::Hidden,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: Val::Px(420.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba { red: 0.06, green: 0.05, blue: 0.10, alpha: 0.98 })),
                    BorderColor::all(Color::from(LinearRgba { red: 0.70, green: 0.55, blue: 0.30, alpha: 1.0 })),
                ))
                .with_children(|dialog| {
                    // タイトル行（タイトル + 閉じるボタン）
                    dialog
                        .spawn((Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.0),
                            ..default()
                        },))
                        .with_children(|row| {
                            row.spawn((
                                UiKarmaDialogTitle,
                                Text::new(""),
                                TextFont { font: font.clone(), font_size: 16.0, ..default() },
                                TextColor(Color::from(LinearRgba { red: 0.95, green: 0.80, blue: 0.40, alpha: 1.0 })),
                            ));
                            row.spawn((
                                UiKarmaDialogCloseButton,
                                Button,
                                Node {
                                    padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::from(LinearRgba { red: 0.25, green: 0.15, blue: 0.15, alpha: 1.0 })),
                                BorderColor::all(Color::from(LinearRgba { red: 0.60, green: 0.30, blue: 0.30, alpha: 1.0 })),
                            ))
                            .with_children(|b| {
                                b.spawn((
                                    Text::new("閉じる"),
                                    TextFont { font: font.clone(), font_size: 13.0, ..default() },
                                    TextColor(Color::WHITE),
                                ));
                            });
                        });

                    // カードリストコンテンツ
                    dialog.spawn((
                        UiKarmaDialogContent,
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.0),
                            ..default()
                        },
                    ));
                });
        });

    // 行動選択メニュー
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
            BackgroundColor(Color::from(LinearRgba { red: 0.0, green: 0.0, blue: 0.1, alpha: 0.9 })),
            BorderColor::all(Color::WHITE),
            Visibility::Hidden,
            ZIndex(10),
        ))
        .with_children(|menu| {
            menu.spawn((
                Text::new("[行動選択]"),
                TextFont { font: font.clone(), font_size: 18.0, ..default() },
                TextColor(Color::WHITE),
            ));
            menu.spawn((
                UiActionMenuContainer,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
            ));
        });

    // 戻るボタン
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
            BackgroundColor(Color::from(LinearRgba { red: 0.3, green: 0.2, blue: 0.2, alpha: 0.9 })),
            BorderColor::all(Color::WHITE),
            ZIndex(20),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new("準備画面に戻る"),
                TextFont { font: font.clone(), font_size: 18.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn cleanup_battle_ui(mut commands: Commands, query: Query<Entity, With<BattleScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
