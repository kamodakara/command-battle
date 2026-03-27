use bevy::prelude::*;
use std::sync::Arc;
use crate::fundamental::Art;
use super::super::resources::*;
use super::super::super::logic::resources::{BattlePhase, PlayerBasicArts, PlayerEquippedWeapons};

// ─── コンポーネント定義 ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct UiActionMenu;

#[derive(Component)]
pub struct UiActionMenuContainer;

#[derive(Component)]
pub struct ActionMenuItem {
    pub item_type: ActionMenuItemType,
}

#[derive(Clone)]
pub enum ActionMenuItemType {
    Category(ActionMenuCategory),
    Art(Arc<Art>),
    ConsecutiveAction(ConsecutiveActionType),
}

#[derive(Clone, PartialEq, Eq)]
pub enum ActionMenuCategory {
    Basic,
    Weapon(usize),
    Back,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ConsecutiveActionType {
    Reenter,
    ConfirmAll,
    ReselectThird,
    NextAction,
}

// ─── 描画システム ────────────────────────────────────────────────────────────

pub fn render(
    phase: Res<BattlePhase>,
    action_menu: Res<ActionMenuSelection>,
    consecutive: Res<ConsecutiveCommands>,
    basic_arts: Res<PlayerBasicArts>,
    equipped_weapons: Res<PlayerEquippedWeapons>,
    asset_server: Res<AssetServer>,
    message_queue: Res<super::super::resources::MessageQueue>,
    mut commands: Commands,
    mut menu_vis_q: Query<&mut Visibility, With<UiActionMenu>>,
    container_q: Query<Entity, With<UiActionMenuContainer>>,
    menu_items_q: Query<Entity, With<ActionMenuItem>>,
) {
    let visible = *phase == BattlePhase::AwaitCommand && message_queue.pending.is_empty();

    // Hidden→Visible の遷移を検出し、非表示中に変更があった場合も再描画を強制する
    let mut just_became_visible = false;
    if let Ok(mut vis) = menu_vis_q.single_mut() {
        just_became_visible = visible && (*vis == Visibility::Hidden);
        *vis = if visible { Visibility::Visible } else { Visibility::Hidden };
    }

    if !visible {
        return;
    }

    let has_menu_items = menu_items_q.iter().next().is_some();
    if !just_became_visible && !action_menu.is_changed() && !consecutive.is_changed() && !phase.is_changed() && has_menu_items {
        return;
    }

    let Ok(container_entity) = container_q.single() else {
        return;
    };

    for entity in menu_items_q.iter() {
        commands.entity(entity).despawn();
    }

    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    match &action_menu.menu_state {
        ActionMenuState::AutoExecuting => {
            commands.entity(container_entity).with_children(|parent| {
                let remaining = consecutive.commands.len();
                let executed = 3 - remaining;
                spawn_label(parent, &font, &format!("【実行中 {}/3】", executed));
                spawn_label(parent, &font, "");
                spawn_button(
                    parent,
                    &font,
                    "次へ ▶",
                    ActionMenuItemType::ConsecutiveAction(ConsecutiveActionType::NextAction),
                );
            });
        }
        ActionMenuState::ConfirmAllCommands => {
            commands.entity(container_entity).with_children(|parent| {
                spawn_label(parent, &font, "【選択したコマンドの確認】");
                for (i, cmd) in consecutive.commands.iter().enumerate() {
                    spawn_label(parent, &font, &format!("{}ターン目: {}", i + 1, cmd.art.name));
                }
                spawn_label(parent, &font, "");
                spawn_label(parent, &font, "この内容でよろしいですか？");
                spawn_label(parent, &font, "");
                spawn_button(
                    parent,
                    &font,
                    "はい",
                    ActionMenuItemType::ConsecutiveAction(ConsecutiveActionType::ConfirmAll),
                );
                spawn_button(
                    parent,
                    &font,
                    "いいえ",
                    ActionMenuItemType::ConsecutiveAction(ConsecutiveActionType::ReselectThird),
                );
            });
        }
        ActionMenuState::ConsecutiveInput => {
            commands.entity(container_entity).with_children(|parent| {
                let turn = consecutive.commands.len() + 1;
                spawn_label(parent, &font, &format!("【連続コマンド入力 - {}ターン目】", turn));

                for (i, cmd) in consecutive.commands.iter().enumerate() {
                    spawn_label(parent, &font, &format!("  {}ターン目: {} ✓", i + 1, cmd.art.name));
                }
                spawn_label(parent, &font, "");

                spawn_button(
                    parent,
                    &font,
                    "基本",
                    ActionMenuItemType::Category(ActionMenuCategory::Basic),
                );
                for (idx, weapon) in equipped_weapons.weapons.iter().enumerate() {
                    spawn_button(
                        parent,
                        &font,
                        &weapon.weapon.name,
                        ActionMenuItemType::Category(ActionMenuCategory::Weapon(idx)),
                    );
                }

                if !consecutive.commands.is_empty() {
                    spawn_button(
                        parent,
                        &font,
                        "前の行動を取り消す",
                        ActionMenuItemType::Category(ActionMenuCategory::Back),
                    );
                }
            });
        }
        ActionMenuState::ConsecutiveBasicArts => {
            commands.entity(container_entity).with_children(|parent| {
                let turn = consecutive.commands.len() + 1;
                spawn_label(parent, &font, &format!("【連続コマンド - {}ターン目 - 基本】", turn));
                for (i, cmd) in consecutive.commands.iter().enumerate() {
                    spawn_label(parent, &font, &format!("  {}ターン目: {} ✓", i + 1, cmd.art.name));
                }
                spawn_label(parent, &font, "");
                spawn_button(
                    parent,
                    &font,
                    "← 戻る",
                    ActionMenuItemType::Category(ActionMenuCategory::Back),
                );
                for art in basic_arts.0.iter() {
                    spawn_button(
                        parent,
                        &font,
                        &format!("{} {}", art.name, format_cost(art.sp_cost, art.stamina_cost)),
                        ActionMenuItemType::Art(Arc::clone(art)),
                    );
                }
            });
        }
        ActionMenuState::ConsecutiveWeaponArts { weapon_idx } => {
            if let Some(weapon_data) = equipped_weapons.weapons.get(*weapon_idx) {
                commands.entity(container_entity).with_children(|parent| {
                    let turn = consecutive.commands.len() + 1;
                    spawn_label(
                        parent,
                        &font,
                        &format!("【連続コマンド - {}ターン目 - {}】", turn, weapon_data.weapon.name),
                    );
                    for (i, cmd) in consecutive.commands.iter().enumerate() {
                        spawn_label(parent, &font, &format!("  {}ターン目: {} ✓", i + 1, cmd.art.name));
                    }
                    spawn_label(parent, &font, "");
                    spawn_button(
                        parent,
                        &font,
                        "← 戻る",
                        ActionMenuItemType::Category(ActionMenuCategory::Back),
                    );

                    if !weapon_data.skills.is_empty() {
                        spawn_label(parent, &font, "【技】");
                        for art in weapon_data.skills.iter() {
                            spawn_button(
                                parent,
                                &font,
                                &format!("{} {}", art.name, format_cost(art.sp_cost, art.stamina_cost)),
                                ActionMenuItemType::Art(Arc::clone(art)),
                            );
                        }
                    }
                    if !weapon_data.sorceries.is_empty() {
                        spawn_label(parent, &font, "【術】");
                        for art in weapon_data.sorceries.iter() {
                            spawn_button(
                                parent,
                                &font,
                                &format!("{} {}", art.name, format_cost(art.sp_cost, art.stamina_cost)),
                                ActionMenuItemType::Art(Arc::clone(art)),
                            );
                        }
                    }
                    if weapon_data.skills.is_empty() && weapon_data.sorceries.is_empty() {
                        spawn_label(parent, &font, "(この武器には技・術がありません)");
                    }
                });
            }
        }
    }
}

fn spawn_button<'a>(
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
                width: Val::Percent(100.0),
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
                TextFont { font: font.clone(), font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
}

/// SP・スタミナコストを "(SP{} ST{})" 形式にフォーマットする。
fn format_cost(sp_cost: u32, stamina_cost: u32) -> String {
    format!("(SP{} ST{})", sp_cost, stamina_cost)
}

fn spawn_label<'a>(parent: &mut ChildSpawnerCommands<'a>, font: &Handle<Font>, label: &str) {
    parent
        .spawn((
            ActionMenuItem {
                item_type: ActionMenuItemType::Category(ActionMenuCategory::Back),
            },
            Node {
                width: Val::Percent(100.0),
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
                TextFont { font: font.clone(), font_size: 14.0, ..default() },
                TextColor(Color::from(LinearRgba {
                    red: 0.7,
                    green: 0.7,
                    blue: 0.9,
                    alpha: 1.0,
                })),
            ));
        });
}
