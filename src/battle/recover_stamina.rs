use super::*;

pub struct RecoverStaminaRequest {
    pub character_id: BattleCharacterId,
}

// スタミナ回復
pub fn recover_stamina(
    battle: &mut Battle,
    request: RecoverStaminaRequest,
) -> BattleIncidentAutoTrigger {
    if let Some(player) = battle
        .players
        .iter_mut()
        .find(|c| c.character_id == request.character_id)
    {
        // スタミナ回復
        let stamina_recovery = player.base.current_stats.stamina_recovery;
        let (before_stamina, after_stamina) =
            player.base.current_stats.stamina_add(stamina_recovery);
        return BattleIncidentAutoTrigger {
            character_id: request.character_id,
            stats_changes: vec![BattleIncidentStats::RecoverStamina(
                BattleIncidentRecoverStamina {
                    recover: stamina_recovery,
                    before: before_stamina,
                    after: after_stamina,
                },
            )],
            status_effects: vec![],
        };
    } else if let Some(_enemy) = battle
        .enemies
        .iter_mut()
        .find(|c| c.character_id == request.character_id)
    {
        // 敵キャラクターの場合は何もしない

        return BattleIncidentAutoTrigger {
            character_id: request.character_id,
            stats_changes: vec![],
            status_effects: vec![],
        };
    }

    // キャラクターが見つからなかった場合も何もしない
    // TODO: エラー処理
    panic!("Character not found");
}
