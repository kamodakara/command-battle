use bevy::prelude::*;
use crate::fundamental::*;
use super::super::resources::KarmaCardsNeedsRedraw;
use super::super::super::logic::resources::BattleResource;

// ─── コンポーネント定義 ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct UiKarmaCardsContainer;

// ─── 描画システム ────────────────────────────────────────────────────────────

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
