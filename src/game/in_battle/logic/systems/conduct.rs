use super::super::resources::*;
use super::super::super::events::*;
use crate::battle::{
    BattleCharacterController, BattleController, BattleDecideOrderRequest,
    BattleExecuteConductRequest,
};
use crate::fundamental::*;
use bevy::prelude::*;
use std::sync::Arc;

pub fn handle_execute_commands(
    mut events: EventReader<ExecuteBattleCommandsEvent>,
    mut phase: ResMut<BattlePhase>,
    mut turn: ResMut<Turn>,
    mut planned: ResMut<EnemyPlannedAction>,
    mut battle_resource: ResMut<BattleResource>,
    mut log_ev: EventWriter<BattleLogEvent>,
    mut combination_ev: EventWriter<BattleCombinationEvent>,
    mut conduct_ev: EventWriter<BattleConductResolvedEvent>,
    mut result_ev: EventWriter<BattleResultEvent>,
) {
    for event in events.read() {
        if *phase != BattlePhase::AwaitCommand {
            continue;
        }

        let cmd = &event.command;
        let battle = &mut battle_resource.0;
        let player_id = battle.player.character_id;
        let enemy_id = battle.enemies.first().map(|e| e.character_id).unwrap_or(2);

        log_ev.write(BattleLogEvent(format!(
            "ターン {} プレイヤーは{}を選択",
            turn.0, cmd.art.name
        )));

        // コンビネーション処理
        battle.player.initialize_current_conduct_log();
        if event.use_combination {
            let stamina_cost = cmd.art.stamina_cost;
            let incident = battle.player.combination(stamina_cost);
            combination_ev.write(BattleCombinationEvent {
                actor_character_id: player_id,
                incident: Arc::new(incident),
            });
        }

        // ターゲット決定
        let target = match &cmd.art.rank1.potency {
            ArtPotency::Attack(_) => BattleConductTargetType::EnemySingle(enemy_id),
            ArtPotency::Support(_) => BattleConductTargetType::Player,
        };

        let player_conduct = BattleConduct {
            actor_character_id: player_id,
            target,
            art: Arc::clone(&cmd.art),
            battle_weapon_id: cmd.battle_weapon_id.clone(),
        };

        let enemy_conduct = planned.0.clone().expect("敵の行動が未定");
        planned.0 = None;

        // 行動順決定
        let order = battle.decide_order(BattleDecideOrderRequest {
            conducts: vec![&player_conduct, &enemy_conduct],
        });

        // 行動実行 → 構造化イベントを発火
        for actor_id in order {
            let conduct_to_execute = if actor_id == player_id {
                player_conduct.clone()
            } else {
                enemy_conduct.clone()
            };

            let incident = battle.execute_conduct(BattleExecuteConductRequest {
                conduct: conduct_to_execute,
            });

            conduct_ev.write(BattleConductResolvedEvent {
                incident: Arc::new(incident),
                player_character_id: player_id,
                enemy_character_id: enemy_id,
            });
        }

        // バトル終了チェック
        let enemy_hp = battle.enemies.first().map(|e| e.hp.current_hp).unwrap_or(0);
        let player_hp = battle.player.hp.current_hp;

        if enemy_hp == 0 {
            *phase = BattlePhase::Finished;
            result_ev.write(BattleResultEvent::Victory);
        } else if player_hp == 0 {
            *phase = BattlePhase::Finished;
            result_ev.write(BattleResultEvent::Defeat);
        } else {
            turn.0 += 1;
            *phase = BattlePhase::TurnEnd;
        }
    }
}
