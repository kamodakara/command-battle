pub mod handlers;
pub mod renderers;
pub mod resources;
mod setup;

use bevy::prelude::*;

pub struct BattleUiPlugin;

impl Plugin for BattleUiPlugin {
    fn build(&self, app: &mut App) {
        let battle_state = in_state(crate::GameState::Battle);
        app.add_systems(OnEnter(crate::GameState::Battle), setup::setup_battle_ui)
            .add_systems(OnExit(crate::GameState::Battle), setup::cleanup_battle_ui)
            // incident イベント → UIリソース更新・BattleLogEvent発火
            .add_systems(
                Update,
                (
                    handlers::battle_incidents::handle_combination_resolved,
                    handlers::battle_incidents::handle_conduct_resolved,
                    handlers::battle_incidents::handle_turn_end_incidents,
                )
                    .run_if(battle_state.clone()),
            )
            // その他イベント → UIリソース更新
            .add_systems(
                Update,
                (
                    handlers::battle_log::on_battle_log,
                    handlers::battle_log::on_enemy_action_planned,
                    handlers::battle_log::tick_message_queue,
                )
                    .run_if(battle_state.clone()),
            )
            // 入力 → UIリソース更新・Logicへイベント発火
            .add_systems(
                Update,
                (
                    handlers::action_menu::action_menu_click_system,
                    handlers::input::back_button_system,
                    renderers::karma_cards::handle_karma_pile_buttons,
                    renderers::karma_cards::handle_karma_dialog_close,
                    renderers::combat_log::handle_combat_log_toggle,
                )
                    .run_if(battle_state.clone()),
            )
            // UIリソース → Bevyコンポーネント描画
            .add_systems(
                Update,
                (
                    renderers::player_status::render,
                    renderers::enemy_status::render,
                    renderers::action_menu::render,
                    renderers::combat_log::render_message,
                    renderers::combat_log::render_phase,
                    renderers::combat_log::render_combat_log_toggle_button,
                    renderers::karma_cards::render,
                    renderers::karma_cards::render_karma_pile_counts,
                    renderers::karma_cards::render_karma_dialog,
                    renderers::damage_popup::render,
                    renderers::skill_effect::render,
                    renderers::turn_action_board::render,
                )
                    .run_if(battle_state.clone()),
            )
            // 勝利演出（イベント起点だが描画処理を持つ）
            .add_systems(
                Update,
                (
                    renderers::victory_banner::on_battle_result,
                    renderers::victory_banner::animate_banner,
                )
                    .run_if(battle_state),
            );
    }
}
