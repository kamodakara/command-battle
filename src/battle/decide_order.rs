use super::*;
// 行動順序決定
pub fn decide_action_order(characters: Vec<&BattleCharacter>) -> Vec<u32> {
    let mut order: Vec<(u32, u32)> = characters
        .iter()
        .map(|c| (c.character_id(), c.current_ability().agility))
        .collect();

    // TODO: 行動によっても変動させる

    order.sort_by(|a, b| b.1.cmp(&a.1));

    order.into_iter().map(|(id, _)| id).collect()
}
