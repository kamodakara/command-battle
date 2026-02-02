mod decide_enemy_conduct;
mod decide_order;
mod execute_conduct;
mod karma_draw_card;
mod partial;
mod turn_end;

// use super::fundamental;
use super::fundamental::*;

pub use decide_enemy_conduct::DecideEnemyConductRequest;
pub use decide_order::BattleDecideOrderRequest;
pub use execute_conduct::BattleExecuteConductRequest;

use decide_enemy_conduct::decide_enemy_conduct;
use decide_order::decide_order;
use execute_conduct::execute_conduct;
use turn_end::turn_end;

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

    pub fn turn_end(&mut self) -> Vec<BattleIncidentCharacter> {
        turn_end(self)
    }
}
