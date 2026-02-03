use super::*;

pub struct BattleDecideOrderRequest<'a> {
    pub conducts: Vec<&'a BattleConduct>, // 行動順を決定するキャラクターID一覧
}

// 行動順序決定
pub fn decide_order(battle: &Battle, request: BattleDecideOrderRequest) -> Vec<u32> {
    let mut order: Vec<(u32, i32, u32)> = Vec::new();
    for conduct in request.conducts {
        // TODO: 効果を考慮する
        let effects = vec![];

        if battle.player.character_id == conduct.actor_character_id {
            order.push((
                conduct.actor_character_id,
                conduct.art.priority,
                battle.player.ability_with_effects(&effects).agility,
            ));
        } else if let Some(enemy) = battle
            .enemies
            .iter()
            .find(|c| c.character_id == conduct.actor_character_id)
        {
            order.push((
                conduct.actor_character_id,
                conduct.art.priority,
                enemy.ability_with_effects(&effects).agility,
            ));
        }
    }

    // 優先度→敏捷性の降順でソート
    order.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)));

    order.into_iter().map(|(id, _, _)| id).collect()
}
