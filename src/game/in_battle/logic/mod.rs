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
            .add_message::<BattleCombinationEvent>()
            .add_message::<BattleConductResolvedEvent>()
            .add_message::<BattleTurnEndEvent>()
            .add_message::<EnemyActionPlannedEvent>()
            .add_message::<BattleResultEvent>()
            // イベント登録（UI → Logic）
            .add_message::<ExecuteBattleCommandsEvent>()
            // セットアップ
            .add_systems(OnEnter(crate::GameState::Battle), setup::setup_battle_logic)
            // システム登録
            .add_systems(
                Update,
                (
                    systems::turn::phase_transition_system,
                    systems::conduct::handle_execute_commands,
                )
                    .run_if(in_state(crate::GameState::Battle)),
            );
    }
}
