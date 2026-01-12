mod decide_enemy_conduct;
mod decide_order;
mod execute_conduct;
mod recover_break;
mod recover_stamina;

use super::types::*;

pub use decide_enemy_conduct::DecideEnemyConductRequest;
pub use decide_order::BattleDecideOrderRequest;
pub use execute_conduct::BattleExecuteConductRequest;
pub use recover_break::RecoverBreakRequest;

use decide_enemy_conduct::decide_enemy_conduct;
use decide_order::decide_order;
use execute_conduct::execute_conduct;
use recover_break::recover_break;
use recover_stamina::recover_stamina;

impl Battle {
    // 行動順を決定する
    pub fn decide_order(&self, request: BattleDecideOrderRequest) -> Vec<u32> {
        decide_order(self, request)
    }

    pub fn execute_conduct(&mut self, request: BattleExecuteConductRequest) -> BattleIncident {
        execute_conduct(self, request)
    }

    pub fn decide_enemy_conduct(&self, request: DecideEnemyConductRequest) -> BattleConduct {
        decide_enemy_conduct(self, request)
    }

    pub fn recover_break(&mut self, request: RecoverBreakRequest) -> BattleIncidentAutoTrigger {
        recover_break(self, request)
    }
}
