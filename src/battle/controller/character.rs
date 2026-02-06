use super::uses::*;

pub trait BattleCharacterController {
    fn initialize_current_conduct_log(&mut self);
    fn combination(&mut self, base: u32);
}

impl BattleCharacterController for BattleCharacter {
    // 現在の行動ログを初期化する、毎ターンする
    fn initialize_current_conduct_log(&mut self) {
        if let Some(combination_skill) = &mut self.combination_skill {
            combination_skill.initialize_current_conduct();
        }
    }

    // コンビネーション発動
    fn combination(&mut self, base: u32) {
        let combination_level = if let Some(combination_skill) = &mut self.combination_skill {
            // コンビネーション発動時にログを残す
            combination_skill.mark_current_conduct_as_combination_activated();

            combination_skill.combination_logs.len() as u32 + 1
        } else {
            1
        };

        // 現在の能力値を取得
        let ability = self.ability_with_effects(&self.current_effects());

        // トランス値上昇
        // 上昇量：(消費スタミナ^2*神秘*コンビネーションレベル)/20
        if let Some(trance) = &mut self.trance {
            let increase = 1 + (base * base * ability.arcane * combination_level) / 50;
            let (_before, _after) = trance.add_trance(increase);

            // TODO: インシデント
        }
    }
}
