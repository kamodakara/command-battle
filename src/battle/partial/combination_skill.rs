use std::vec;

use super::*;

impl BattleCombinationSkill {
    // 現在の行動ログを初期化する、毎ターンする
    pub fn initialize_current_conduct(&mut self) {
        self.current_combination_conduct_log = Some(BattleCombinationConductLog {
            categories: Vec::new(),
            results: Vec::new(),
            combination_activated: false,
        });
    }

    // 現在の行動がコンビネーション発動
    pub fn mark_current_conduct_as_combination_activated(&mut self) {
        if let Some(current_log) = &mut self.current_combination_conduct_log {
            current_log.combination_activated = true;
        }
    }

    // 行動実行直後に確定した行動のカテゴリを現在の行動ログに追加する
    pub fn add_current_conduct_categories(&mut self, categories: Vec<CombinationConductCategory>) {
        if let Some(current_log) = &mut self.current_combination_conduct_log {
            // 現在の行動ログにカテゴリを追加
            current_log.categories.extend(categories);
        }

        // 現在の行動ログが存在しない場合は何もしない
    }

    // 行動の結果を現在の行動ログに追加する
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

        // ログが4つを超えたら古いものを削除する
        while self.combination_logs.len() > 4 {
            self.combination_logs.remove(0);
        }
    }

    // コンビネーション技が発動可能か判定する
    // 発動したコンビネーション技を返す
    pub fn activate_combination_skills(&self) -> Vec<&CombinationSkill> {
        // 発動条件の判定ロジックを実装
        if let Some(current_log) = &self.current_combination_conduct_log {
            if !current_log.combination_activated {
                // コンビネーションが発動していない場合なので発動しない
                return vec![];
            }

            // コンビネーション技の発動条件をチェック
            let combination_skills = self
                .combination_skills
                .iter()
                .filter(|combination_skill| {
                    let condition = &combination_skill.condition;

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
                            // 直前の行動ログが存在しない場合
                            return false;
                        }
                    }

                    // 二つ前の行動の条件をチェック
                    if let Some(two_steps_before_requirements) =
                        &condition.two_steps_before_requirements
                    {
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
                            // 二つ前の行動ログが存在しない場合
                            return false;
                        }
                    }

                    // すべての条件を満たしている場合、発動可能
                    true
                })
                .collect();

            return combination_skills;
        }

        vec![]
    }
}
