pub mod components;
pub mod resources;
mod setup;
pub mod systems;

use bevy::prelude::*;

pub struct BattleUiPlugin;

impl Plugin for BattleUiPlugin {
    fn build(&self, app: &mut App) {
        let battle_state = in_state(crate::GameState::Battle);
        app.add_systems(OnEnter(crate::GameState::Battle), setup::setup_battle_ui)
            .add_systems(OnExit(crate::GameState::Battle), setup::cleanup_battle_ui)
            // incidentイベント受信 → BattleLogEvent / UIリソース更新
            .add_systems(
                Update,
                (
                    systems::incidents::handle_combination_resolved,
                    systems::incidents::handle_conduct_resolved,
                )
                    .run_if(battle_state.clone()),
            )
            // その他イベント受信 → UIリソース更新
            .add_systems(
                Update,
                (
                    systems::update::on_battle_log,
                    systems::update::on_enemy_action_planned,
                )
                    .run_if(battle_state.clone()),
            )
            // UI描画更新
            .add_systems(
                Update,
                (
                    systems::update::ui_update_system,
                    systems::update::ui_update_enemy_system,
                    systems::update::ui_update_enemy_damage_popup_system,
                    systems::update::ui_update_player_status_system,
                    systems::update::ui_update_command_system,
                    systems::update::ui_update_message_system,
                    systems::update::ui_update_skill_effect_system,
                    systems::update::ui_update_karma_cards_system,
                    systems::update::boss_slain_banner_system,
                )
                    .run_if(battle_state.clone()),
            )
            // アクションメニュー・バトル終了・入力
            .add_systems(
                Update,
                (
                    systems::action_menu::action_menu_click_system,
                    systems::action_menu::action_menu_update_system,
                    systems::battle_end::battle_result_system,
                    systems::input::back_button_system,
                )
                    .run_if(battle_state),
            );
    }
}
