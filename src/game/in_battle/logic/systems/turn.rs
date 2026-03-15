use bevy::prelude::*;
use crate::battle::{BattleController, DecideEnemyConductRequest};
use super::super::resources::*;
use super::super::super::events::*;

pub fn phase_transition_system(
    mut phase: ResMut<BattlePhase>,
    mut battle_resource: ResMut<BattleResource>,
    mut planned: ResMut<EnemyPlannedActions>,
    mut enemy_planned_ev: MessageWriter<EnemyActionPlannedEvent>,
) {
    match *phase {
        BattlePhase::DecideEnemyConduct => {
            // カルマドロー
            battle_resource.0.karma_draw_card();

            // 敵の行動を3回分決定
            let enemy_id = battle_resource.0.enemies.first().map(|e| e.character_id).unwrap_or(2);
            let mut conducts = Vec::new();
            let mut action_names = Vec::new();

            for _ in 0..3 {
                let conduct = battle_resource.0.decide_enemy_conduct(DecideEnemyConductRequest {
                    enemy_character_id: enemy_id,
                });
                action_names.push(conduct.art.name.clone());
                conducts.push(conduct);
            }

            enemy_planned_ev.write(EnemyActionPlannedEvent { action_names });
            planned.0 = conducts;

            *phase = BattlePhase::AwaitCommand;
        }
        BattlePhase::TurnEnd => {
            battle_resource.0.turn_end();
            *phase = BattlePhase::DecideEnemyConduct;
        }
        _ => {}
    }
}
