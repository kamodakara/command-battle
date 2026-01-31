use super::*;

pub struct BattleTrance {
    pub max_trance: u32, // 最大トランス量
    pub heart: Heart,

    pub current_trance: u32, // トランス量
}

pub struct BattleCombinationSkill {
    pub combination_skill: CombinationSkill,

    pub current_combination_conduct_log: Option<BattleCombinationConductLog>,
    pub combination_logs: Vec<BattleCombinationConductLog>,
}

pub struct BattleCombinationConductLog {
    pub categories: Vec<CombinationConductCategory>,
    pub results: Vec<CombinationConductResult>,
}
