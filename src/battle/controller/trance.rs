use super::uses::*;

pub trait BattleTranceController {
    fn trance_level(&self) -> u32;
    fn current_heart_effects(&self) -> Vec<HeartEffect>;
}
impl BattleTranceController for BattleTrance {
    fn trance_level(&self) -> u32 {
        BattleTrancePartialTrait::trance_level(self)
    }
    fn current_heart_effects(&self) -> Vec<HeartEffect> {
        BattleTrancePartialTrait::current_heart_effects(self)
    }
}
