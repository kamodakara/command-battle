use bevy::prelude::*;
use super::enemy_status::{UiEnemy, UiEnemyBreakLabel, UiEnemyHpGaugeFill, UiEnemyBreakGaugeFill, UiEnemyNextActionText};
use super::super::super::events::BattleResultEvent;

// ─── コンポーネント定義 ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct BossSlainText;

#[derive(Component)]
pub struct BossSlainBanner {
    pub elapsed: f32,
    pub phase: BannerPhase,
}

#[derive(Component)]
pub struct BossSlainBackdrop;
#[derive(Component)]
pub struct BossSlainBackdropCenter;
#[derive(Component)]
pub struct BossSlainBackdropRow(pub u8);

pub enum BannerPhase {
    FadeIn,
    Hold,
    FadeOut,
}

// ─── 勝利時スポーン ──────────────────────────────────────────────────────────

pub fn on_battle_result(
    mut events: MessageReader<BattleResultEvent>,
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

                let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");
                commands
                    .spawn((
                        BossSlainBanner { elapsed: -0.3, phase: BannerPhase::FadeIn },
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
                                            red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0,
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
                                        red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0,
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
                                            red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0,
                                        })),
                                    ));
                                }
                            });

                        builder.spawn((
                            BossSlainText,
                            Text::new("DRAGON SLAIN"),
                            TextFont { font: font.clone(), font_size: 96.0, ..default() },
                            TextColor(Color::from(LinearRgba {
                                red: 0.83, green: 0.72, blue: 0.20, alpha: 0.0,
                            })),
                            ZIndex(101),
                        ));
                    });
            }
            BattleResultEvent::Defeat => {}
        }
    }
}

// ─── バナーアニメーション ────────────────────────────────────────────────────

pub fn animate_banner(
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
                let a = (banner.elapsed / FADE_IN).clamp(0.0, 1.0);
                set_banner_alpha(&mut text_colors, &mut backdrop_colors, a);
                if banner.elapsed >= FADE_IN {
                    banner.phase = BannerPhase::Hold;
                    banner.elapsed = 0.0;
                }
            }
            BannerPhase::Hold => {
                set_banner_alpha(&mut text_colors, &mut backdrop_colors, 1.0);
                if banner.elapsed >= HOLD {
                    banner.phase = BannerPhase::FadeOut;
                    banner.elapsed = 0.0;
                }
            }
            BannerPhase::FadeOut => {
                let a = 1.0 - (banner.elapsed / FADE_OUT).clamp(0.0, 1.0);
                set_banner_alpha(&mut text_colors, &mut backdrop_colors, a);
                if banner.elapsed >= FADE_OUT {
                    for i in 0..children.len() {
                        commands.entity(children[i]).despawn();
                    }
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn set_banner_alpha(
    text_colors: &mut Query<&mut TextColor, With<BossSlainText>>,
    backdrop_colors: &mut ParamSet<(
        Query<&mut BackgroundColor, With<BossSlainBackdropRow>>,
        Query<&mut BackgroundColor, With<BossSlainBackdropCenter>>,
    )>,
    alpha: f32,
) {
    for mut c in text_colors.iter_mut() {
        c.0 = Color::from(LinearRgba { red: 0.83, green: 0.72, blue: 0.20, alpha });
    }
    for mut bc in backdrop_colors.p1().iter_mut() {
        bc.0 = Color::from(LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha });
    }
    for mut br in backdrop_colors.p0().iter_mut() {
        br.0 = Color::from(LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.9 * alpha });
    }
}
