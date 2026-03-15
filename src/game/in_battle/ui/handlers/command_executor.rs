use bevy::prelude::*;
use super::super::resources::*;
use super::super::super::logic::resources::BattlePhase;
use super::super::super::events::ExecuteBattleCommandsEvent;

/// AutoExecuting状態になったら確定済みコマンドを全てLogicへ送信する
pub fn auto_execute_commands(
    mut action_menu: ResMut<ActionMenuSelection>,
    mut consecutive: ResMut<ConsecutiveCommands>,
    phase: Res<BattlePhase>,
    mut execute_ev: MessageWriter<ExecuteBattleCommandsEvent>,
) {
    if *phase != BattlePhase::AwaitCommand {
        return;
    }
    if action_menu.menu_state != ActionMenuState::AutoExecuting {
        return;
    }

    let commands: Vec<_> = consecutive.commands.drain(..).collect();
    for cmd in commands {
        execute_ev.write(ExecuteBattleCommandsEvent { command: cmd });
    }
    action_menu.input();
}
