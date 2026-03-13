pub mod resources;
mod battle_factory;
mod setup;
pub mod systems;

use bevy::prelude::*;
use super::events::*;

pub struct BattleLogicPlugin;

impl Plugin for BattleLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            // イベント登録（Logic → UI）
            .add_message::<BattleLogEvent>()
            .add_message::<EnemyDamagedEvent>()
            .add_message::<EnemyActionPlannedEvent>()
            .add_message::<BattleResultEvent>()
            // イベント登録（UI → Logic）
            .add_message::<PlayerArtSelectedEvent>()
            .add_message::<ExecuteQueuedEvent>()
            .add_message::<CancelQueuedEvent>()
            .add_message::<RemoveLastQueuedEvent>()
            // セットアップ
            .add_systems(OnEnter(crate::GameState::Battle), setup::setup_battle_logic)
            // システム登録
            .add_systems(
                Update,
                (
                    systems::turn::phase_transition_system,
                    systems::conduct::handle_player_art_selected,
                    systems::conduct::handle_execute_queued,
                    systems::conduct::handle_cancel_queued,
                    systems::conduct::handle_remove_last_queued,
                )
                    .run_if(in_state(crate::GameState::Battle)),
            );
    }
}
