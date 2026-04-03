mod decide_enemy_conduct;
mod decide_order;
mod execute_conduct;
mod karma_draw_card;
mod turn_end;

use super::uses::*;
use crate::data::KarmaCardRepository;

pub use decide_enemy_conduct::DecideEnemyConductRequest;
pub use decide_order::BattleDecideOrderRequest;
pub use execute_conduct::BattleExecuteConductRequest;

use decide_enemy_conduct::decide_enemy_conduct;
use decide_order::decide_order;
use execute_conduct::execute_conduct;
use karma_draw_card::karma_draw_card;
use turn_end::turn_end;

pub trait BattleController {
    // 行動順を決定する
    fn decide_order(&self, request: BattleDecideOrderRequest) -> Vec<u32>;
    fn execute_conduct(&mut self, request: BattleExecuteConductRequest) -> BattleIncidentConduct;
    fn decide_enemy_conduct(&mut self, request: DecideEnemyConductRequest) -> BattleConduct;
    fn turn_end(&mut self) -> Vec<BattleIncidentCharacter>;
    fn karma_draw_card(&mut self, card_repo: &KarmaCardRepository);
}

impl BattleController for Battle {
    // 行動順を決定する
    fn decide_order(&self, request: BattleDecideOrderRequest) -> Vec<u32> {
        decide_order(self, request)
    }

    fn execute_conduct(&mut self, request: BattleExecuteConductRequest) -> BattleIncidentConduct {
        execute_conduct(self, request)
    }

    fn decide_enemy_conduct(&mut self, request: DecideEnemyConductRequest) -> BattleConduct {
        decide_enemy_conduct(self, request)
    }

    fn turn_end(&mut self) -> Vec<BattleIncidentCharacter> {
        turn_end(self)
    }

    fn karma_draw_card(&mut self, card_repo: &KarmaCardRepository) {
        karma_draw_card(self, card_repo);
    }
}
