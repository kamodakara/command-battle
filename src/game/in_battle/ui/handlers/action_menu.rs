use bevy::prelude::*;
use std::sync::Arc;
use super::super::resources::*;
use super::super::renderers::action_menu::{ActionMenuItem, ActionMenuItemType, ActionMenuCategory, ConsecutiveActionType};
use super::super::super::logic::resources::{BattlePhase, PlayerEquippedWeapons};
use super::super::super::events::*;

pub fn action_menu_click_system(
    phase: Res<BattlePhase>,
    mut action_menu: ResMut<ActionMenuSelection>,
    mut consecutive: ResMut<ConsecutiveCommands>,
    equipped_weapons: Res<PlayerEquippedWeapons>,
    mut interaction_query: Query<
        (&Interaction, &ActionMenuItem),
        (Changed<Interaction>, With<Button>),
    >,
    mut execute_ev: EventWriter<ExecuteBattleCommandsEvent>,
) {
    if *phase != BattlePhase::AwaitCommand {
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
                    if let ActionMenuState::ConsecutiveInput = action_menu.menu_state {
                        consecutive.commands.pop();
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
                        consecutive.commands.push(ConsecutiveCommandEntry {
                            art: Arc::clone(art),
                            weapon_index: Some(weapon_idx),
                            battle_weapon_id,
                        });
                        if consecutive.commands.len() >= 3 {
                            action_menu.confirm_all();
                        } else {
                            action_menu.input();
                        }
                    }
                    ActionMenuState::ConsecutiveBasicArts => {
                        consecutive.commands.push(ConsecutiveCommandEntry {
                            art: Arc::clone(art),
                            weapon_index: None,
                            battle_weapon_id: None,
                        });
                        if consecutive.commands.len() >= 3 {
                            action_menu.confirm_all();
                        } else {
                            action_menu.input();
                        }
                    }
                    _ => {}
                }
            }
            ActionMenuItemType::ConsecutiveAction(action_type) => match action_type {
                ConsecutiveActionType::Execute => {
                    if let Some(cmd) = consecutive.commands.first().cloned() {
                        consecutive.commands.remove(0);
                        execute_ev.write(ExecuteBattleCommandsEvent {
                            command: cmd,
                            use_combination: true,
                        });
                        if !consecutive.commands.is_empty() {
                            action_menu.confirm();
                        } else {
                            action_menu.input();
                        }
                    }
                }
                ConsecutiveActionType::Reenter => {
                    consecutive.commands.clear();
                    action_menu.input();
                }
                ConsecutiveActionType::FinishInput => {
                    if let Some(cmd) = consecutive.commands.first().cloned() {
                        consecutive.commands.remove(0);
                        execute_ev.write(ExecuteBattleCommandsEvent {
                            command: cmd,
                            use_combination: false,
                        });
                        if !consecutive.commands.is_empty() {
                            action_menu.confirm();
                        } else {
                            action_menu.input();
                        }
                    }
                }
                ConsecutiveActionType::ConfirmAll => {
                    if let Some(cmd) = consecutive.commands.first().cloned() {
                        consecutive.commands.remove(0);
                        execute_ev.write(ExecuteBattleCommandsEvent {
                            command: cmd,
                            use_combination: false,
                        });
                        if !consecutive.commands.is_empty() {
                            action_menu.confirm();
                        } else {
                            action_menu.input();
                        }
                    }
                }
                ConsecutiveActionType::ReselectThird => {
                    consecutive.commands.pop();
                    action_menu.input();
                }
            },
        }
    }
}
