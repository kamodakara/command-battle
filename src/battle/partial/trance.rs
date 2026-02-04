use super::{BattleTrance, HeartEffect};

pub trait BattleTrancePartialTrait {
    fn add_trance(&mut self, amount: u32) -> (u32, u32);
    fn reduce_trance(&mut self, amount: u32) -> (u32, u32);
    fn trance_level(&self) -> u32;
    fn current_heart_effects(&self) -> Vec<HeartEffect>;
}

impl BattleTrancePartialTrait for BattleTrance {
    fn add_trance(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_trance;
        self.current_trance += amount;
        if self.current_trance > self.max_trance {
            self.current_trance = self.max_trance;
        }
        (before, self.current_trance)
    }
    // トランスの減少
    fn reduce_trance(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_trance;
        self.current_trance = self.current_trance.saturating_sub(amount);
        (before, self.current_trance)
    }

    fn trance_level(&self) -> u32 {
        if self.current_trance >= 700 {
            3
        } else if self.current_trance >= 400 {
            2
        } else if self.current_trance >= 100 {
            1
        } else {
            0
        }
    }

    // 現在有効なハート効果を取得する
    fn current_heart_effects(&self) -> Vec<HeartEffect> {
        let level = self.trance_level();
        if level == 3 {
            // レベル3
            self.heart.level3_effects.clone()
        } else if level == 2 {
            // レベル2
            self.heart.level2_effects.clone()
        } else if level == 1 {
            // レベル1
            self.heart.level1_effects.clone()
        } else {
            vec![]
        }
    }
}
