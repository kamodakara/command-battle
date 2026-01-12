use super::*;

pub struct BattleDecideOrderRequest {
    pub character_ids: Vec<BattleCharacterId>, // 行動順を決定するキャラクターID一覧
}

// 行動順序決定
pub fn decide_order(battle: &Battle, request: BattleDecideOrderRequest) -> Vec<u32> {
    let mut order: Vec<(u32, u32)> = Vec::new();
    for id in request.character_ids {
        if let Some(character) = battle.players.iter().find(|c| c.character_id == id) {
            order.push((id, character.base.current_ability.agility));
        } else if let Some(character) = battle.enemies.iter().find(|c| c.character_id == id) {
            order.push((id, character.base.current_ability.agility));
        }
    }

    // TODO: 行動によっても変動させる

    order.sort_by(|a, b| b.1.cmp(&a.1));

    order.into_iter().map(|(id, _)| id).collect()
}
