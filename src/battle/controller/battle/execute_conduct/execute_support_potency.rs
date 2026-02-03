use super::*;

pub fn execute_support_potency(
    support: &ArtPotencySupport,
    target: &mut BattleCharacter,
) -> Vec<BattleCharacterIncidentConcrete> {
    // 支援処理
    match support {
        // 支援状態変化
        ArtPotencySupport::StatusCondition(status_condition) => {
            support_status_effect(&status_condition.status_conditions, target)
        }
        // 支援回復
        ArtPotencySupport::Recover(recover) => support_recover(recover, target),
    }
}

fn support_status_effect(
    status_conditions: &Vec<StatusCondition>,
    target: &mut BattleCharacter,
) -> Vec<BattleCharacterIncidentConcrete> {
    let mut incidents = vec![];
    // 支援行動処理
    for status_condition in status_conditions {
        // 状態変化付与処理
        let battle_status_condition = create_battle_status_condition(status_condition);
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
