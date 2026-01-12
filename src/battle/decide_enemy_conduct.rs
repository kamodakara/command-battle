use super::*;

// TODO: 実装
// 敵キャラクターの行動決定
// どういうデータを返すか

pub struct DecideEnemyConductRequest {
    pub enemy_character_id: BattleCharacterId,
}

pub fn decide_enemy_conduct(battle: &Battle, request: DecideEnemyConductRequest) -> BattleConduct {
    // TODO: 実装
    panic!("Not implemented yet");
}
