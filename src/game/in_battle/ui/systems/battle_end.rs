use bevy::prelude::*;
use super::super::components::*;
use super::super::super::events::BattleResultEvent;

pub fn battle_result_system(
    mut events: EventReader<BattleResultEvent>,
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
) {
    for event in events.read() {
        match event {
            BattleResultEvent::Victory => {
                // 敵UIを即時非表示
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
                    hp_node.width = Val::Percent(0.0);
                }
                if let Ok(mut br_node) = gauge_params_end.p1().single_mut() {
                    br_node.width = Val::Percent(0.0);
                }

                // 勝利バナーを表示
                let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");
                commands
                    .spawn((
                        BossSlainBanner {
                            elapsed: -0.3,
                            phase: BannerPhase::FadeIn,
                        },
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ZIndex(100),
                    ))
                    .with_children(|builder| {
                        builder
                            .spawn((
                                BossSlainBackdrop,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Auto,
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(0.0),
                                    right: Val::Px(0.0),
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
                                for i in (0..6u8).rev() {
                                    back.spawn((
                                        BossSlainBackdropRow(i),
                                        Node {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(12.0),
                                            ..default()
                                        },
                                        BackgroundColor(Color::from(LinearRgba {
                                            red: 0.0,
                                            green: 0.0,
                                            blue: 0.0,
                                            alpha: 0.0,
                                        })),
                                    ));
                                }
                                back.spawn((
                                    BossSlainBackdropCenter,
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(140.0),
                                        ..default()
                                    },
                                    BackgroundColor(Color::from(LinearRgba {
                                        red: 0.0,
                                        green: 0.0,
                                        blue: 0.0,
                                        alpha: 0.0,
                                    })),
                                ));
                                for i in 0..6u8 {
                                    back.spawn((
                                        BossSlainBackdropRow(10 + i),
                                        Node {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(12.0),
                                            ..default()
                                        },
                                        BackgroundColor(Color::from(LinearRgba {
                                            red: 0.0,
                                            green: 0.0,
                                            blue: 0.0,
                                            alpha: 0.0,
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
            }
            BattleResultEvent::Defeat => {
                // TODO: 敗北演出
            }
        }
    }
}
