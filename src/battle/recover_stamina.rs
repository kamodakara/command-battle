use super::*;

pub struct RecoverStaminaRequest {
    pub character_id: BattleCharacterId,
}

// スタミナ回復
pub fn recover_stamina(
    battle: &mut Battle,
    request: RecoverStaminaRequest,
) -> BattleIncidentCharacter {
    // if let Some(player) = battle
    //     .players
    //     .iter_mut()
    //     .find(|c| c.character_id == request.character_id)
    // {
    //     // スタミナ回復
    //     let stamina_recovery = player.stamina.stamina_recovery;
    //     let (before_stamina, after_stamina) = player.stamina.recover(stamina_recovery);

    //     let mut incident =
    //         BattleCharacterIncident::new(BattleCharacterIncidentReason::TurnEndRecovery);
    //     incident.add_concrete(BattleCharacterIncidentConcrete::RecoverStamina(
    //         BattleIncidentRecoverStamina::new(stamina_recovery, before_stamina, after_stamina),
    //     ));
    //     return BattleIncidentCharacter {
    //         character_id: request.character_id,
    //         incidents: vec![incident],
    //     };
    // } else if let Some(_enemy) = battle
    //     .enemies
    //     .iter_mut()
    //     .find(|c| c.character_id == request.character_id)
    // {
    //     // 敵キャラクターの場合は何もしない

    //     return BattleIncidentCharacter {
    //         character_id: request.character_id,
    //         incidents: vec![],
    //     };
    // }

    // キャラクターが見つからなかった場合も何もしない
    // TODO: エラー処理
    panic!("Character not found");
}
