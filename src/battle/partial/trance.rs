use super::*;

impl BattleTrance {
    // 現在有効なハート効果を取得する
    pub fn current_heart_effects(&self) -> Vec<HeartEffect> {
        if self.current_trance >= 700 {
            // レベル3
            self.heart.level3_effects.clone()
        } else if self.current_trance >= 400 {
            // レベル2
            self.heart.level2_effects.clone()
        } else if self.current_trance >= 100 {
            // レベル1
            self.heart.level1_effects.clone()
        } else {
            vec![]
        }
    }
}
