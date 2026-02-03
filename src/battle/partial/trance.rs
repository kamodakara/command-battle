use super::{BattleTrance, HeartEffect};

pub trait BattleTrancePartialTrait {
    fn trance_level(&self) -> u32;
    fn current_heart_effects(&self) -> Vec<HeartEffect>;
}

impl BattleTrancePartialTrait for BattleTrance {
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
