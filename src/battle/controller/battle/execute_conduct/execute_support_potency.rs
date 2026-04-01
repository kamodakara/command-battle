use super::*;

pub fn execute_support_potency(
    support: &ArtPotencySupport,
    attacker_data: &AttackerData,
    target_data: TargetData,
) -> Vec<BattleCharacterIncidentConcrete> {
    // 支援処理
    match support {
        ArtPotencySupport::None => vec![], // 行動しない
        // 支援状態変化
        ArtPotencySupport::StatusCondition(status_condition) => support_status_effect(
            &status_condition.status_conditions,
            attacker_data,
            target_data.target,
        ),
        // 支援回復
        ArtPotencySupport::Recover(recover) => support_recover(recover, target_data.target),
        // 状態異常蓄積
        ArtPotencySupport::StatusAilment(status_ailment) => super::accumulate_status_ailment(
            target_data.target,
            &status_ailment.kind,
            status_ailment.accumulation,
        ),
    }
}

fn support_status_effect(
    status_conditions: &Vec<StatusCondition>,
    attacker_data: &AttackerData,
    target: &mut BattleCharacter,
) -> Vec<BattleCharacterIncidentConcrete> {
    let mut incidents = vec![];
    // 支援行動処理
    for status_condition in status_conditions {
        // 状態変化付与処理
        let mut battle_status_condition = create_battle_status_condition(status_condition);
        // 防御状態の場合、使用する武器IDをコマンドの武器IDで上書きする
        if let StatusConditionPotency::Resistance(ref mut resistance) =
            battle_status_condition.potency
        {
            if let Some(weapon_id) = &attacker_data.conduct.battle_weapon_id {
                resistance.battle_weapon_id = weapon_id.clone();
            }
        }
        // 状態変化付与
        // TODO: 状態変化の重複処理
        target
            .status_conditions
            .push(battle_status_condition.clone());

        // インシデント
        incidents.push(BattleCharacterIncidentConcrete::StatusConditionApplied(
            BattleIncidentStatusConditionApplied {
                status_condition: battle_status_condition,
            },
        ))
    }
    incidents
}

fn support_recover(
    recover: &ArtPotencySupportRecover,
    target: &mut BattleCharacter,
) -> Vec<BattleCharacterIncidentConcrete> {
    let mut incidents = vec![];

    // 支援回復処理
    for potency in &recover.potencies {
        match potency {
            SupportRecoverPotency::Hp(hp_recover) => {
                let hp_rcv = hp_recover.hp_recover;
                let (before_hp, after_hp) = target.hp.recover(hp_rcv);

                // HP回復のインシデント
                incidents.push(BattleCharacterIncidentConcrete::RecoverHp(
                    BattleIncidentRecoverHp::new(hp_rcv, before_hp, after_hp),
                ));
            }
            SupportRecoverPotency::Sp(sp_recover) => {
                let sp_rcv = sp_recover.sp_recover;
                let (before_sp, after_sp) = target.sp.recover(sp_rcv);

                // SP回復のインシデント
                incidents.push(BattleCharacterIncidentConcrete::RecoverSp(
                    BattleIncidentRecoverSp::new(sp_rcv, before_sp, after_sp),
                ));
            }
            SupportRecoverPotency::Stamina(stamina_recover) => {
                // スタミナ回復処理はプレイヤーキャラクターのみ
                if target.character_type == BattleCharacterType::Player {
                    let stamina_rcv = stamina_recover.stamina_recover;
                    let (before_stamina, after_stamina) = target.stamina.recover(stamina_rcv);

                    // スタミナ回復のインシデント
                    incidents.push(BattleCharacterIncidentConcrete::RecoverStamina(
                        BattleIncidentRecoverStamina::new(
                            stamina_rcv,
                            before_stamina,
                            after_stamina,
                        ),
                    ));
                }
            }
        }
    }

    incidents
}
