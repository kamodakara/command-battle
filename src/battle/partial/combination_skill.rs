use super::*;

impl BattleCombinationSkill {
    // コンビネーション発動時にログを残すための初期化
    // プレイヤーは連続コマンド入力実行時、コンビネーションは発動時にこの関数を呼び出す
    pub fn initialize_current_conduct(&mut self) {
        self.current_combination_conduct_log = Some(BattleCombinationConductLog {
            categories: Vec::new(),
            results: Vec::new(),
        });
    }

    pub fn add_current_conduct_categories(&mut self, categories: Vec<CombinationConductCategory>) {
        if let Some(current_log) = &mut self.current_combination_conduct_log {
            // 現在の行動ログにカテゴリを追加
            current_log.categories.extend(categories);
        }

        // 現在の行動ログが存在しない場合は何もしない
    }

    pub fn add_current_conduct_result(&mut self, result: CombinationConductResult) {
        if let Some(current_log) = &mut self.current_combination_conduct_log {
            // 現在の行動ログに結果を追加
            current_log.results.push(result);
        }

        // 現在の行動ログが存在しない場合は何もしない
    }

    // 現在の行動ログを確定し、過去の行動ログに追加する
    pub fn finalize_current_conduct(&mut self) {
        if let Some(current_log) = self.current_combination_conduct_log.take() {
            self.combination_logs.push(current_log);
        }
        self.current_combination_conduct_log = None;
    }

    // コンビネーション技が発動可能か判定する
    pub fn can_activate_combination_skill(&self) -> bool {
        // 発動条件の判定ロジックを実装
        if let Some(current_log) = &self.current_combination_conduct_log {
            let condition = &self.combination_skill.condition;

            // 現在の行動の条件をチェック
            for required_category in &condition.current_requirements.categories {
                if !current_log.categories.contains(required_category) {
                    return false;
                }
            }
            for required_result in &condition.current_requirements.results {
                if !current_log.results.contains(required_result) {
                    return false;
                }
            }

            // 直前の行動の条件をチェック
            if let Some(previous_requirements) = &condition.previous_requirements {
                if let Some(previous_log) = self.combination_logs.last() {
                    for required_category in &previous_requirements.categories {
                        if !previous_log.categories.contains(required_category) {
                            return false;
                        }
                    }
                    for required_result in &previous_requirements.results {
                        if !previous_log.results.contains(required_result) {
                            return false;
                        }
                    }
                } else {
                    return false; // 直前の行動ログが存在しない場合
                }
            }

            // 二つ前の行動の条件をチェック
            if let Some(two_steps_before_requirements) = &condition.two_steps_before_requirements {
                if self.combination_logs.len() >= 2 {
                    let two_steps_before_log =
                        &self.combination_logs[self.combination_logs.len() - 2];
                    for required_category in &two_steps_before_requirements.categories {
                        if !two_steps_before_log.categories.contains(required_category) {
                            return false;
                        }
                    }
                    for required_result in &two_steps_before_requirements.results {
                        if !two_steps_before_log.results.contains(required_result) {
                            return false;
                        }
                    }
                } else {
                    return false; // 二つ前の行動ログが存在しない場合
                }
            }

            // すべての条件を満たしている場合、発動可能
            return true;
        }

        false
    }
}
