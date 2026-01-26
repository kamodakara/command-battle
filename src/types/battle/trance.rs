use super::*;

pub struct BattleCombination {
    logs: Vec<BattleCombinationConductLog>, // 実行行動の履歴、古い順
}
pub struct BattleCombinationConductLog {
    pub conduct_categories: Vec<CombinationConductCategory>,
}

pub struct BattleTrance {
    pub max_trance: u32, // 最大トランス量
    pub heart: Heart,

    pub current_trance: u32,                   // トランス量
    pub battle_combination: BattleCombination, // 戦闘中のコンビネーション
}
