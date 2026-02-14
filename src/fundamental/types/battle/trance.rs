use super::*;

#[derive(Debug)]
pub struct BattleTrance {
    pub max_trance: u32, // 最大トランス量
    pub heart: Heart,

    pub current_trance: u32, // トランス量
}

#[derive(Debug)]
pub struct BattleCombinationSkill {
    pub combination_skills: Vec<CombinationSkill>,

    pub current_combination_conduct_log: Option<BattleCombinationConductLog>,
    pub combination_logs: Vec<BattleCombinationConductLog>,
}

#[derive(Debug)]
pub struct BattleCombinationConductLog {
    pub categories: Vec<CombinationConductCategory>,
    pub results: Vec<CombinationConductResult>,
    pub combination_activated: bool, // コンビネーションが発動したかどうか
}
