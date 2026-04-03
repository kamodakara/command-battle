use bevy::prelude::*;
use crate::fundamental::*;
use crate::data::DataManager;
use super::super::resources::{KarmaCardsNeedsRedraw, KarmaDialogState};
use super::super::super::logic::resources::BattleResource;

// ─── コンポーネント定義 ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct UiKarmaCardsContainer;

#[derive(Component)]
pub struct UiKarmaDeckButton;

#[derive(Component)]
pub struct UiKarmaDeckCount;

#[derive(Component)]
pub struct UiKarmaDiscardButton;

#[derive(Component)]
pub struct UiKarmaDiscardCount;

#[derive(Component)]
pub struct UiKarmaDialog;

#[derive(Component)]
pub struct UiKarmaDialogTitle;

#[derive(Component)]
pub struct UiKarmaDialogContent;

#[derive(Component)]
pub struct UiKarmaDialogCloseButton;

// ─── フィールドカルマ描画 ─────────────────────────────────────────────────────

pub fn render(
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
                        red: 0.6, green: 0.6, blue: 0.6, alpha: 1.0,
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
                                red: 0.12, green: 0.10, blue: 0.18, alpha: 1.0,
                            })),
                            BorderColor::all(Color::from(LinearRgba {
                                red: 0.70, green: 0.55, blue: 0.30, alpha: 1.0,
                            })),
                        ))
                        .with_children(|card_box| {
                            card_box.spawn((
                                Text::new(&card.card.name),
                                TextFont { font: font.clone(), font_size: 12.0, ..default() },
                                TextColor(Color::from(LinearRgba {
                                    red: 1.0, green: 0.90, blue: 0.60, alpha: 1.0,
                                })),
                            ));
                            card_box.spawn((
                                Node { flex_grow: 1.0, ..default() },
                                Text::new(&effect_text),
                                TextFont { font: font.clone(), font_size: 11.0, ..default() },
                                TextColor(Color::from(LinearRgba {
                                    red: 0.80, green: 0.80, blue: 0.95, alpha: 1.0,
                                })),
                            ));
                            card_box.spawn((
                                Text::new(format!("{}T", card.remaining_turns)),
                                TextFont { font: font.clone(), font_size: 11.0, ..default() },
                                TextColor(Color::from(LinearRgba {
                                    red: 0.60, green: 0.85, blue: 0.60, alpha: 1.0,
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
                    red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0,
                })),
            ));
        });
    }
}

// ─── 山札・捨て札 枚数更新 ────────────────────────────────────────────────────

pub fn render_karma_pile_counts(
    battle_resource: Res<BattleResource>,
    mut deck_q: Query<
        &mut Text,
        (With<UiKarmaDeckCount>, Without<UiKarmaDiscardCount>),
    >,
    mut discard_q: Query<
        &mut Text,
        (With<UiKarmaDiscardCount>, Without<UiKarmaDeckCount>),
    >,
) {
    let Some(karma) = &battle_resource.0.player.karma else {
        return;
    };

    if let Ok(mut text) = deck_q.single_mut() {
        text.0 = format!("山札: {}枚", karma.draw_pile.len());
    }
    if let Ok(mut text) = discard_q.single_mut() {
        text.0 = format!("捨て札: {}枚", karma.discard_pile.len());
    }
}

// ─── ボタン入力処理 ───────────────────────────────────────────────────────────

pub fn handle_karma_pile_buttons(
    mut dialog_state: ResMut<KarmaDialogState>,
    deck_q: Query<&Interaction, (Changed<Interaction>, With<UiKarmaDeckButton>)>,
    discard_q: Query<&Interaction, (Changed<Interaction>, With<UiKarmaDiscardButton>)>,
) {
    for interaction in deck_q.iter() {
        if *interaction == Interaction::Pressed {
            *dialog_state = KarmaDialogState::DrawPile;
        }
    }
    for interaction in discard_q.iter() {
        if *interaction == Interaction::Pressed {
            *dialog_state = KarmaDialogState::DiscardPile;
        }
    }
}

pub fn handle_karma_dialog_close(
    mut dialog_state: ResMut<KarmaDialogState>,
    close_q: Query<&Interaction, (Changed<Interaction>, With<UiKarmaDialogCloseButton>)>,
) {
    for interaction in close_q.iter() {
        if *interaction == Interaction::Pressed {
            *dialog_state = KarmaDialogState::Closed;
        }
    }
}

// ─── ダイアログ描画 ───────────────────────────────────────────────────────────

pub fn render_karma_dialog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    battle_resource: Res<BattleResource>,
    dialog_state: Res<KarmaDialogState>,
    data_manager: Res<DataManager>,
    mut dialog_q: Query<&mut Visibility, With<UiKarmaDialog>>,
    mut title_q: Query<&mut Text, With<UiKarmaDialogTitle>>,
    content_q: Query<Entity, With<UiKarmaDialogContent>>,
    children_q: Query<&Children>,
) {
    if !dialog_state.is_changed() {
        return;
    }

    let Ok(mut dialog_visibility) = dialog_q.single_mut() else {
        return;
    };

    if *dialog_state == KarmaDialogState::Closed {
        *dialog_visibility = Visibility::Hidden;
        return;
    }

    *dialog_visibility = Visibility::Visible;

    // タイトル更新
    if let Ok(mut title) = title_q.single_mut() {
        title.0 = match *dialog_state {
            KarmaDialogState::DrawPile => "山札".to_string(),
            KarmaDialogState::DiscardPile => "捨て札".to_string(),
            KarmaDialogState::Closed => unreachable!(),
        };
    }

    // コンテンツ再構築
    let Ok(content_entity) = content_q.single() else {
        return;
    };
    if let Ok(children) = children_q.get(content_entity) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    let font: Handle<Font> = asset_server.load("fonts/x12y16pxMaruMonica.ttf");
    let Some(karma) = &battle_resource.0.player.karma else {
        return;
    };

    let deck_cards: &Vec<_> = match *dialog_state {
        KarmaDialogState::DrawPile => &karma.draw_pile,
        KarmaDialogState::DiscardPile => &karma.discard_pile,
        KarmaDialogState::Closed => unreachable!(),
    };

    if deck_cards.is_empty() {
        commands.entity(content_entity).with_children(|p| {
            p.spawn((
                Text::new("カードなし"),
                TextFont { font: font.clone(), font_size: 13.0, ..default() },
                TextColor(Color::from(LinearRgba {
                    red: 0.6, green: 0.6, blue: 0.6, alpha: 1.0,
                })),
            ));
        });
        return;
    }

    for deck_card in deck_cards {
        let Some(record) = data_manager.karma_card.find_by_id(deck_card.card_id.0) else {
            continue;
        };
        let card = &record.data;
        let effect_strs: Vec<String> =
            card.effects.iter().map(|e| format_karma_effect(e)).collect();
        let effect_text = if effect_strs.is_empty() {
            "効果なし".to_string()
        } else {
            effect_strs.join(", ")
        };
        let card_name = card.name.clone();

        commands.entity(content_entity).with_children(|p| {
            p.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(5.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(2.0),
                    ..default()
                },
                BackgroundColor(Color::from(LinearRgba {
                    red: 0.12, green: 0.10, blue: 0.18, alpha: 1.0,
                })),
                BorderColor::all(Color::from(LinearRgba {
                    red: 0.50, green: 0.40, blue: 0.20, alpha: 1.0,
                })),
            ))
            .with_children(|card_box| {
                card_box.spawn((
                    Text::new(card_name),
                    TextFont { font: font.clone(), font_size: 13.0, ..default() },
                    TextColor(Color::from(LinearRgba {
                        red: 1.0, green: 0.90, blue: 0.60, alpha: 1.0,
                    })),
                ));
                card_box.spawn((
                    Text::new(effect_text),
                    TextFont { font: font.clone(), font_size: 11.0, ..default() },
                    TextColor(Color::from(LinearRgba {
                        red: 0.80, green: 0.80, blue: 0.95, alpha: 1.0,
                    })),
                ));
            });
        });
    }
}

// ─── ヘルパー ─────────────────────────────────────────────────────────────────

pub fn format_karma_effect(effect: &KarmaEffect) -> String {
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
