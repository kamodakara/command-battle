use bevy::prelude::*;
use super::super::resources::*;
use super::super::super::events::*;

pub fn on_battle_log(mut events: MessageReader<BattleLogEvent>, mut log: ResMut<CombatLog>) {
    for event in events.read() {
        log.0.push(event.0.clone());
    }
}

pub fn on_enemy_action_planned(
    mut events: MessageReader<EnemyActionPlannedEvent>,
    mut display: ResMut<EnemyNextActionDisplay>,
) {
    for event in events.read() {
        display.0 = event.action_name.clone();
    }
}
