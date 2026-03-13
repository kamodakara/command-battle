use bevy::prelude::*;
use std::sync::Arc;
use super::super::components::*;
use super::super::resources::*;
use super::super::super::logic::resources::{
    BattlePhase, ConsecutiveCommands, PlayerBasicArts, PlayerEquippedWeapons,
};
use super::super::super::events::*;

pub fn action_menu_click_system(
    phase: Res<BattlePhase>,
    mut action_menu: ResMut<ActionMenuSelection>,
    consecutive: Res<ConsecutiveCommands>,
    equipped_weapons: Res<PlayerEquippedWeapons>,
    mut interaction_query: Query<
        (&Interaction, &ActionMenuItem),
        (Changed<Interaction>, With<Button>),
    >,
    mut art_selected_ev: EventWriter<PlayerArtSelectedEvent>,
    mut execute_ev: EventWriter<ExecuteQueuedEvent>,
    mut cancel_ev: EventWriter<CancelQueuedEvent>,
    mut remove_ev: EventWriter<RemoveLastQueuedEvent>,
) {
    if *phase != BattlePhase::AwaitCommand
        && *phase != BattlePhase::ConfirmQueued
        && *phase != BattlePhase::ConfirmAllCommands
    {
        return;
    }

    for (interaction, menu_item) in interaction_query.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match &menu_item.item_type {
            ActionMenuItemType::Category(category) => match category {
                ActionMenuCategory::Basic => {
                    action_menu.select_category_basic();
                }
                ActionMenuCategory::Weapon(idx) => {
                    action_menu.select_category_weapon(*idx);
                }
                ActionMenuCategory::Back => {
                    // ConsecutiveInput 状態でのBackはコマンド取り消し
                    if let ActionMenuState::ConsecutiveInput = action_menu.menu_state {
                        if !consecutive.commands.is_empty() {
                            remove_ev.write(RemoveLastQueuedEvent);
                        }
                    }
                    action_menu.menu_state = ActionMenuState::ConsecutiveInput;
                }
            },
            ActionMenuItemType::Art(art) => {
                match action_menu.menu_state.clone() {
                    ActionMenuState::ConsecutiveWeaponArts { weapon_idx } => {
                        let battle_weapon_id = equipped_weapons
                            .weapons
                            .get(weapon_idx)
                            .map(|w| w.battle_weapon_id.clone());
                        art_selected_ev.write(PlayerArtSelectedEvent {
                            art: Arc::clone(art),
                            weapon_index: Some(weapon_idx),
                            battle_weapon_id,
                        });
                        action_menu.input();
                    }
                    ActionMenuState::ConsecutiveBasicArts => {
                        art_selected_ev.write(PlayerArtSelectedEvent {
                            art: Arc::clone(art),
                            weapon_index: None,
                            battle_weapon_id: None,
                        });
                        action_menu.input();
                    }
                    _ => {}
                }
            }
            ActionMenuItemType::ConsecutiveAction(action_type) => match action_type {
                ConsecutiveActionType::Execute => {
                    execute_ev.write(ExecuteQueuedEvent { use_combination: true });
                }
                ConsecutiveActionType::Reenter => {
                    cancel_ev.write(CancelQueuedEvent);
                    action_menu.input();
                }
                ConsecutiveActionType::FinishInput => {
                    execute_ev.write(ExecuteQueuedEvent { use_combination: false });
                }
                ConsecutiveActionType::ConfirmAll => {
                    execute_ev.write(ExecuteQueuedEvent { use_combination: false });
                }
                ConsecutiveActionType::ReselectThird => {
                    remove_ev.write(RemoveLastQueuedEvent);
                    action_menu.input();
                }
            },
        }
    }
}

pub fn action_menu_update_system(
    phase: Res<BattlePhase>,
    action_menu: Res<ActionMenuSelection>,
    consecutive: Res<ConsecutiveCommands>,
    basic_arts: Res<PlayerBasicArts>,
    equipped_weapons: Res<PlayerEquippedWeapons>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut menu_vis_q: Query<&mut Visibility, With<UiActionMenu>>,
    container_q: Query<Entity, With<UiActionMenuContainer>>,
    menu_items_q: Query<Entity, With<ActionMenuItem>>,
) {
    if let Ok(mut vis) = menu_vis_q.single_mut() {
        *vis = if *phase == BattlePhase::AwaitCommand
            || *phase == BattlePhase::ConfirmQueued
            || *phase == BattlePhase::ConfirmAllCommands
        {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if *phase != BattlePhase::AwaitCommand
        && *phase != BattlePhase::ConfirmQueued
        && *phase != BattlePhase::ConfirmAllCommands
    {
        return;
    }

    let has_menu_items = menu_items_q.iter().next().is_some();
    if !action_menu.is_changed() && !consecutive.is_changed() && has_menu_items {
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
        ActionMenuState::ConsecutiveConfirm => {
            commands.entity(container_entity).with_children(|parent| {
                spawn_menu_label(parent, &font, "【設定済み連続コマンド】");
                for (i, cmd) in consecutive.commands.iter().enumerate() {
                    let label = format!("{}ターン目: {}", i + 1, cmd.art.name);
                    spawn_menu_label(parent, &font, &label);
                }
                spawn_menu_label(parent, &font, "");
                spawn_menu_button(
                    parent,
                    &font,
                    "▶ 連続コマンドを実行",
                    ActionMenuItemType::ConsecutiveAction(ConsecutiveActionType::Execute),
                );
                spawn_menu_button(
                    parent,
                    &font,
                    "✕ コマンド入力しなおし",
                    ActionMenuItemType::ConsecutiveAction(ConsecutiveActionType::Reenter),
                );
            });
        }
        ActionMenuState::ConfirmAllCommands => {
            commands.entity(container_entity).with_children(|parent| {
                spawn_menu_label(parent, &font, "【選択したコマンドの確認】");
                for (i, cmd) in consecutive.commands.iter().enumerate() {
                    let label = format!("{}ターン目: {}", i + 1, cmd.art.name);
                    spawn_menu_label(parent, &font, &label);
                }
                spawn_menu_label(parent, &font, "");
                spawn_menu_label(parent, &font, "この内容でよろしいですか？");
                spawn_menu_label(parent, &font, "");
                spawn_menu_button(
                    parent,
                    &font,
                    "はい",
                    ActionMenuItemType::ConsecutiveAction(ConsecutiveActionType::ConfirmAll),
                );
                spawn_menu_button(
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
                let title = format!("【連続コマンド入力 - {}ターン目】", turn);
                spawn_menu_label(parent, &font, &title);

                for (i, cmd) in consecutive.commands.iter().enumerate() {
                    let label = format!("  {}ターン目: {} ✓", i + 1, cmd.art.name);
                    spawn_menu_label(parent, &font, &label);
                }
                spawn_menu_label(parent, &font, "");

                spawn_menu_button(
                    parent,
                    &font,
                    "基本",
                    ActionMenuItemType::Category(ActionMenuCategory::Basic),
                );

                for (idx, weapon) in equipped_weapons.weapons.iter().enumerate() {
                    spawn_menu_button(
                        parent,
                        &font,
                        &weapon.weapon.name,
                        ActionMenuItemType::Category(ActionMenuCategory::Weapon(idx)),
                    );
                }

                if !consecutive.commands.is_empty() {
                    let finish_label =
                        format!("入力完了（{}ターン分）", consecutive.commands.len());
                    spawn_menu_button(
                        parent,
                        &font,
                        &finish_label,
                        ActionMenuItemType::ConsecutiveAction(ConsecutiveActionType::FinishInput),
                    );
                    spawn_menu_button(
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
                let title = format!("【連続コマンド - {}ターン目 - 基本】", turn);
                spawn_menu_label(parent, &font, &title);

                for (i, cmd) in consecutive.commands.iter().enumerate() {
                    let label = format!("  {}ターン目: {} ✓", i + 1, cmd.art.name);
                    spawn_menu_label(parent, &font, &label);
                }
                spawn_menu_label(parent, &font, "");

                spawn_menu_button(
                    parent,
                    &font,
                    "← 戻る",
                    ActionMenuItemType::Category(ActionMenuCategory::Back),
                );

                for art in basic_arts.0.iter() {
                    let label = format!("{} (ST{})", art.name, art.stamina_cost);
                    spawn_menu_button(
                        parent,
                        &font,
                        &label,
                        ActionMenuItemType::Art(Arc::clone(art)),
                    );
                }
            });
        }
        ActionMenuState::ConsecutiveWeaponArts { weapon_idx } => {
            if let Some(weapon_data) = equipped_weapons.weapons.get(*weapon_idx) {
                commands.entity(container_entity).with_children(|parent| {
                    let turn = consecutive.commands.len() + 1;
                    let title = format!(
                        "【連続コマンド - {}ターン目 - {}】",
                        turn, weapon_data.weapon.name
                    );
                    spawn_menu_label(parent, &font, &title);

                    for (i, cmd) in consecutive.commands.iter().enumerate() {
                        let label = format!("  {}ターン目: {} ✓", i + 1, cmd.art.name);
                        spawn_menu_label(parent, &font, &label);
                    }
                    spawn_menu_label(parent, &font, "");

                    spawn_menu_button(
                        parent,
                        &font,
                        "← 戻る",
                        ActionMenuItemType::Category(ActionMenuCategory::Back),
                    );

                    if !weapon_data.skills.is_empty() {
                        spawn_menu_label(parent, &font, "【技】");
                        for art in weapon_data.skills.iter() {
                            let label = format!("{} (ST{})", art.name, art.stamina_cost);
                            spawn_menu_button(
                                parent,
                                &font,
                                &label,
                                ActionMenuItemType::Art(Arc::clone(art)),
                            );
                        }
                    }

                    if !weapon_data.sorceries.is_empty() {
                        spawn_menu_label(parent, &font, "【術】");
                        for art in weapon_data.sorceries.iter() {
                            let label =
                                format!("{} (SP{}/ST{})", art.name, art.sp_cost, art.stamina_cost);
                            spawn_menu_button(
                                parent,
                                &font,
                                &label,
                                ActionMenuItemType::Art(Arc::clone(art)),
                            );
                        }
                    }

                    if weapon_data.skills.is_empty() && weapon_data.sorceries.is_empty() {
                        spawn_menu_label(parent, &font, "(この武器には技・術がありません)");
                    }
                });
            }
        }
    }
}

fn spawn_menu_button<'a>(
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

fn spawn_menu_label<'a>(parent: &mut ChildSpawnerCommands<'a>, font: &Handle<Font>, label: &str) {
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
