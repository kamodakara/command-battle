mod decide_enemy_conduct;
mod decide_order;
mod execute_conduct;
mod karma_draw_card;
mod karma_for_turn_end;
mod partial;
mod recover_break;
mod recover_stamina;
mod turn_end;
mod update_status_condition_for_turn;

// use super::fundamental;
use super::fundamental::*;

pub use decide_enemy_conduct::DecideEnemyConductRequest;
pub use decide_order::BattleDecideOrderRequest;
pub use execute_conduct::BattleExecuteConductRequest;
pub use recover_break::RecoverBreakRequest;
pub use recover_stamina::RecoverStaminaRequest;
pub use update_status_condition_for_turn::UpdateStatusConditionRequest;

use decide_enemy_conduct::decide_enemy_conduct;
use decide_order::decide_order;
use execute_conduct::execute_conduct;
use recover_break::recover_break;
use recover_stamina::recover_stamina;
use update_status_condition_for_turn::update_status_condition_for_turn;

impl Battle {
    // 行動順を決定する
    pub fn decide_order(&self, request: BattleDecideOrderRequest) -> Vec<u32> {
        decide_order(self, request)
    }

    pub fn execute_conduct(
        &mut self,
        request: BattleExecuteConductRequest,
    ) -> BattleIncidentConduct {
        execute_conduct(self, request)
    }

    pub fn decide_enemy_conduct(&self, request: DecideEnemyConductRequest) -> BattleConduct {
        decide_enemy_conduct(self, request)
    }

    pub fn recover_break(&mut self, request: RecoverBreakRequest) -> BattleIncidentCharacter {
        recover_break(self, request)
    }

    pub fn recover_stamina(&mut self, request: RecoverStaminaRequest) -> BattleIncidentCharacter {
        recover_stamina(self, request)
    }

    pub fn update_status_condition_for_turn(
        &mut self,
        request: UpdateStatusConditionRequest,
    ) -> BattleIncidentCharacter {
        update_status_condition_for_turn(self, request)
    }
}
