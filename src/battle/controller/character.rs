use super::uses::*;

pub trait BattleCharacterController {
    fn combination(&mut self, base: u32);
    fn reset_combination(&mut self);
}

impl BattleCharacterController for BattleCharacter {
    fn combination(&mut self, base: u32) {
        let combination_level = if let Some(combination_skill) = &mut self.combination_skill {
            // 現ターンのコンビネーションログの初期化
            combination_skill.initialize_current_conduct();

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

    fn reset_combination(&mut self) {
        // コンビネーションの初期化
        if let Some(combination_skill) = &mut self.combination_skill {
            combination_skill.combination_logs.clear();
            combination_skill.current_combination_conduct_log = None;
        }
    }
}
