use bevy::prelude::*;
use super::super::resources::*;
use super::super::super::events::*;

pub fn on_battle_log(mut events: MessageReader<BattleLogEvent>, mut queue: ResMut<MessageQueue>) {
    for event in events.read() {
        queue.pending.push_back(event.0.clone());
    }
}

/// タイマーを進めて、キューから1件ずつ CombatLog へ移す
pub fn tick_message_queue(
    mut queue: ResMut<MessageQueue>,
    mut log: ResMut<CombatLog>,
    time: Res<Time>,
) {
    queue.timer -= time.delta_secs();
    if queue.timer <= 0.0 {
        if let Some(msg) = queue.pending.pop_front() {
            log.0.push(msg);
            queue.timer = 1.0;
        }
    }
}

pub fn on_enemy_action_planned(
    mut events: MessageReader<EnemyActionPlannedEvent>,
    mut display: ResMut<EnemyNextActionDisplay>,
    mut board: ResMut<TurnActionBoard>,
) {
    for event in events.read() {
        display.0 = event.action_names.clone();
        board.reset();
    }
}
