use bevy::prelude::*;
use crate::battle::BattleTranceController;
use crate::fundamental::*;
use super::super::components::*;
use super::super::resources::*;
use super::super::super::logic::resources::{BattlePhase, BattleResource};
use super::super::super::events::*;

// ─── イベント受信 → UIリソース更新 ───────────────────────────────────────────

pub fn on_battle_log(mut events: EventReader<BattleLogEvent>, mut log: ResMut<CombatLog>) {
    for event in events.read() {
        log.0.push(event.0.clone());
    }
}

pub fn on_enemy_damaged(
    mut events: EventReader<EnemyDamagedEvent>,
    mut popup: ResMut<EnemyDamagePopup>,
) {
    for event in events.read() {
        popup.amount = event.amount as i32;
        popup.timer = 1.0;
    }
}

pub fn on_enemy_action_planned(
    mut events: EventReader<EnemyActionPlannedEvent>,
    mut display: ResMut<EnemyNextActionDisplay>,
) {
    for event in events.read() {
        display.0 = event.action_name.clone();
    }
}

// ─── UI描画更新 ──────────────────────────────────────────────────────────────

pub fn ui_update_system(
    phase: Res<BattlePhase>,
    log: Res<CombatLog>,
    enemy_action: Res<EnemyNextActionDisplay>,
    battle_resource: Res<BattleResource>,
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
        p_hp, player.hp.max_hp, p_stamina, player.stamina.max_stamina,
        e_hp, enemy.hp.max_hp, e_break, e_break_max, "通常"
    );

    if let Ok((mut _eff_atk_text, mut eff_atk_color)) = ui_eff_atk_q.single_mut() {
        eff_atk_color.0 = Color::WHITE;
    }
    if let Ok((mut _eff_heal_text, mut eff_heal_color)) = ui_eff_heal_q.single_mut() {
        eff_heal_color.0 = Color::WHITE;
    }
    if let Ok((mut _eff_def_text, mut eff_def_color)) = ui_eff_def_q.single_mut() {
        eff_def_color.0 = Color::WHITE;
    }

    let phase_str = match *phase {
        BattlePhase::DecideEnemyConduct => {
            format!("敵の行動決定中... 次の行動: {}", enemy_action.0)
        }
        BattlePhase::AwaitCommand => "行動を選択してください".to_string(),
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

pub fn ui_update_command_system() {}

pub fn ui_update_message_system(
    log: Res<CombatLog>,
    mut msg_q: Query<&mut Text, With<UiMessage>>,
) {
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

pub fn ui_update_player_status_system(
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

    if let Ok(mut hp_text) = hp_text_q.single_mut() {
        hp_text.0 = format!("HP: {} / {}", p_hp, player.hp.max_hp);
    }
    if let Ok(mut sta_text) = sta_text_q.single_mut() {
        sta_text.0 = format!("スタミナ: {} / {}", p_sta, player.stamina.max_stamina);
    }
    if let Ok(mut sp_text) = sp_text_q.single_mut() {
        sp_text.0 = format!("SP: {} / {}", p_sp, player.sp.max_sp);
    }

    if let Some(trance) = &player.trance {
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
                let effect_strs: Vec<String> =
                    heart_effects.iter().map(|e| format_heart_effect(e)).collect();
                effect_text.0 = format!("効果: {}", effect_strs.join(", "));
            }
        }

        if let Ok(mut trance_node) = gauge_params.p3().single_mut() {
            let ratio = if max_trance > 0 {
                (current_trance as f32 / max_trance as f32).clamp(0.0, 1.0)
            } else {
                0.0
            };
            trance_node.width = Val::Percent((ratio * 100.0).round());
        }
    }

    if let Ok(mut hp_node) = gauge_params.p0().single_mut() {
        let ratio = if player.hp.max_hp > 0 {
            (p_hp as f32 / player.hp.max_hp as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        hp_node.width = Val::Percent((ratio * 100.0).round());
    }
    if let Ok(mut sta_node) = gauge_params.p1().single_mut() {
        let ratio = if player.stamina.max_stamina > 0 {
            (p_sta as f32 / player.stamina.max_stamina as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        sta_node.width = Val::Percent((ratio * 100.0).round());
    }
    if let Ok(mut sp_node) = gauge_params.p2().single_mut() {
        let ratio = if player.sp.max_sp > 0 {
            (p_sp as f32 / player.sp.max_sp as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        sp_node.width = Val::Percent((ratio * 100.0).round());
    }
}

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
        HeartEffect::MagicalAttackModifier(m) => {
            format!("魔攻+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
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

pub fn ui_update_karma_cards_system(
    mut commands: Commands,
    battle_resource: Res<BattleResource>,
    asset_server: Res<AssetServer>,
    container_q: Query<Entity, With<UiKarmaCardsContainer>>,
    children_q: Query<&Children>,
    mut _redraw_flag: ResMut<KarmaCardsNeedsRedraw>,
) {
    let battle = &battle_resource.0;
    let player = &battle.player;

    let Ok(container_entity) = container_q.single() else {
        return;
    };

    if let Ok(children) = children_q.get(container_entity) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    let font: Handle<Font> = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    if let Some(karma) = &player.karma {
        if karma.field_cards.is_empty() {
            commands.entity(container_entity).with_children(|parent| {
                parent.spawn((
                    Text::new("なし"),
                    TextFont { font: font.clone(), font_size: 12.0, ..default() },
                    TextColor(Color::from(LinearRgba {
                        red: 0.6,
                        green: 0.6,
                        blue: 0.6,
                        alpha: 1.0,
                    })),
                ));
            });
        } else {
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
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
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
                            card_box.spawn((
                                Text::new(&card.card.name),
                                TextFont { font: font.clone(), font_size: 12.0, ..default() },
                                TextColor(Color::from(LinearRgba {
                                    red: 1.0,
                                    green: 0.90,
                                    blue: 0.60,
                                    alpha: 1.0,
                                })),
                            ));
                            card_box.spawn((
                                Node { flex_grow: 1.0, ..default() },
                                Text::new(&effect_text),
                                TextFont { font: font.clone(), font_size: 11.0, ..default() },
                                TextColor(Color::from(LinearRgba {
                                    red: 0.80,
                                    green: 0.80,
                                    blue: 0.95,
                                    alpha: 1.0,
                                })),
                            ));
                            card_box.spawn((
                                Text::new(format!("{}T", card.remaining_turns)),
                                TextFont { font: font.clone(), font_size: 11.0, ..default() },
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
        commands.entity(container_entity).with_children(|parent| {
            parent.spawn((
                Text::new("-"),
                TextFont { font: font.clone(), font_size: 12.0, ..default() },
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

pub fn boss_slain_banner_system(
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

pub fn ui_update_skill_effect_system(
    _battle_resource: Res<BattleResource>,
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
    let skl_power = 25;
    let skl_cost = 25;
    let Ok((mut eff_skl_text, mut eff_skl_color)) = ui_eff_skl_q.single_mut() else {
        return;
    };
    eff_skl_text.0 = format!(
        "強攻撃 威力:{} 消費:{} / ブレイク+{}\n",
        skl_power, skl_cost, 25
    );
    eff_skl_color.0 = Color::WHITE;
}

pub fn ui_update_enemy_system(
    battle_resource: Res<BattleResource>,
    enemy_action: Res<EnemyNextActionDisplay>,
    mut gauge_params: ParamSet<(
        Query<&mut Node, With<UiEnemyHpGaugeFill>>,
        Query<&mut Node, With<UiEnemyBreakGaugeFill>>,
    )>,
    mut _br_label_q: Query<&mut Visibility, With<UiEnemyBreakLabel>>,
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
        hp_node.width = Val::Percent((ratio * 100.0).round());
    }
    if let Ok(mut br_node) = gauge_params.p1().single_mut() {
        let ratio = (e_break as f32 / e_break_max as f32).clamp(0.0, 1.0);
        br_node.width = Val::Percent((ratio * 100.0).round());
    }
    if let Ok(mut t) = next_text_q.single_mut() {
        let display = if enemy_action.0.is_empty() {
            "不明".to_string()
        } else {
            enemy_action.0.clone()
        };
        t.0 = format!("次の行動: {}", display);
    }
}

pub fn ui_update_enemy_damage_popup_system(
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
