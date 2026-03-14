use bevy::prelude::*;
use crate::battle::{BattleController, DecideEnemyConductRequest};
use super::super::resources::*;
use super::super::super::events::*;

pub fn phase_transition_system(
    mut phase: ResMut<BattlePhase>,
    mut battle_resource: ResMut<BattleResource>,
    mut planned: ResMut<EnemyPlannedAction>,
    mut enemy_planned_ev: MessageWriter<EnemyActionPlannedEvent>,
) {
    match *phase {
        BattlePhase::DecideEnemyConduct => {
            // カルマドロー
            battle_resource.0.karma_draw_card();

            // 敵の行動決定
            let enemy_id = battle_resource.0.enemies.first().map(|e| e.character_id).unwrap_or(2);
            let conduct = battle_resource.0.decide_enemy_conduct(DecideEnemyConductRequest {
                enemy_character_id: enemy_id,
            });

            // 敵の行動名をUIへ通知
            enemy_planned_ev.write(EnemyActionPlannedEvent {
                action_name: conduct.art.name.clone(),
            });
            planned.0 = Some(conduct);

            *phase = BattlePhase::AwaitCommand;
        }
        BattlePhase::TurnEnd => {
            battle_resource.0.turn_end();
            *phase = BattlePhase::DecideEnemyConduct;
        }
        _ => {}
    }
}
