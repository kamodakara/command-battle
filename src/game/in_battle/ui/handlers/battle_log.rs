use bevy::prelude::*;
use rand::Rng;
use super::super::resources::*;
use super::super::super::events::*;

pub fn on_battle_log(mut events: MessageReader<BattleLogEvent>, mut queue: ResMut<MessageQueue>) {
    for event in events.read() {
        queue.pending.push_back(event.0.clone());
    }
}

/// タイマーを進めて、キューから1件ずつ CombatLog へ移す
/// キューが空になったタイミングで行動ボードの保留リセットを適用する
pub fn tick_message_queue(
    mut queue: ResMut<MessageQueue>,
    mut log: ResMut<CombatLog>,
    mut board: ResMut<TurnActionBoard>,
    time: Res<Time>,
) {
    queue.timer -= time.delta_secs();
    if queue.timer <= 0.0 {
        if let Some(msg) = queue.pending.pop_front() {
            log.0.push(msg);
            queue.timer = 0.5;
        } else {
            board.apply_pending_reset_if_any();
        }
    }
}

pub fn on_enemy_action_planned(
    mut events: MessageReader<EnemyActionPlannedEvent>,
    mut board: ResMut<TurnActionBoard>,
) {
    for event in events.read() {
        // 即時リセットせず、メッセージ送り完了後に適用するよう予約する
        if !event.action_names.is_empty() {
            let hint_idx = rand::rng().random_range(0..event.action_names.len());
            board.schedule_reset(hint_idx, event.action_names[hint_idx].clone());
        }
    }
}
