mod conduct_effect;
mod execute_attack_potency;
mod execute_support_potency;

use std::sync::Arc;

use execute_attack_potency::execute_attack_potency;
use execute_support_potency::execute_support_potency;

use super::*;

// 攻撃者データ
// 判定や計算に必要なデータをまとめる
// 行動前のデータ
struct AttackerData {
    character_id: BattleCharacterId,
    conduct: BattleConduct,

    character_type: BattleCharacterType,
    hp: BattleCharacterHP,
    sp: BattleCharacterSP,
    stamina: BattleCharacterStamina,
    break_resistance: BattleCharacterBreak,
    status_ailment: BattleStatusAilment,
    status_conditions: Vec<BattleStatusCondition>,

    final_ability: Ability, // 効果適応済み能力
    weapon_performance: WeaponPerformance,
    art_rank: ArtRank, // 効果ランク

    effects: Vec<Effect>, // 様々な効果

                          // TODO: 他に必要なデータがあれば追加
}

struct TargetData<'a> {
    target: &'a mut BattleCharacter,
    effects: Vec<Effect>, // 様々な効果
}

pub struct BattleExecuteConductRequest {
    pub conduct: BattleConduct,
}

// 行動実行
pub fn execute_conduct(
    battle: &mut Battle,
    request: BattleExecuteConductRequest,
) -> BattleIncidentConduct {
    let conduct = request.conduct;

    // 行動者の決定
    let attacker = if let Some(character) = battle.character_mut(&conduct.actor_character_id) {
        character
    } else {
        panic!("Attacker not found");
    };
    let attacker_id = attacker.character_id;

    // 状態異常、カルマ等から効果を取得する
    let mut attacker_effects = attacker.current_effects();

    // 効果適応済み能力取得
    let attacker_ability = attacker.ability_with_effects(&attacker_effects);

    // 能力補正済みの武器性能取得
    let attacker_weapon_performance = if let Some(battle_weapon_id) = &conduct.battle_weapon_id {
        if let Some(weapon) = attacker.weapon(&battle_weapon_id) {
            weapon.weapon.performance(&attacker_ability)
        } else {
            // TODO: エラー処理
            panic!("Weapon not found");
        }
    } else {
        // 武器なし
        unarmed_weapon_performance()
    };

    let sorcery_power = attacker_weapon_performance.final_sorcery_power();
    // 効果ランク判定
    let rank = conduct.art.effective_rank(sorcery_power).clone();

    // コンビネーションログ追加、ランク決まって技の内容が確定した後に行う必要がある
    if let Some(combination_skill) = &mut attacker.combination_skill {
        let mut categories = vec![];
        // 攻撃
        if let ArtPotency::Attack(potency) = &rank.potency {
            categories.push(CombinationConductCategory::Attack);

            // 攻撃属性
            // 攻撃力算出
            // TODO: 同じことをダメージ計算時にやってるの1回だけにするようにするか検討
            let weapon_attack_power = attacker_weapon_performance.final_attack_power();
            let attack_power = potency.final_attack_power(&weapon_attack_power);
            if attack_power.slash > 0 {
                categories.push(CombinationConductCategory::AttackAttribute(
                    Attribute::Slash,
                ));
            }
            if attack_power.strike > 0 {
                categories.push(CombinationConductCategory::AttackAttribute(
                    Attribute::Strike,
                ));
            }
            if attack_power.thrust > 0 {
                categories.push(CombinationConductCategory::AttackAttribute(
                    Attribute::Thrust,
                ));
            }
            if attack_power.impact > 0 {
                categories.push(CombinationConductCategory::AttackAttribute(
                    Attribute::Impact,
                ));
            }
            if attack_power.magic > 0 {
                categories.push(CombinationConductCategory::AttackAttribute(
                    Attribute::Magic,
                ));
            }
            if attack_power.fire > 0 {
                categories.push(CombinationConductCategory::AttackAttribute(Attribute::Fire));
            }
            if attack_power.lightning > 0 {
                categories.push(CombinationConductCategory::AttackAttribute(
                    Attribute::Lightning,
                ));
            }
            if attack_power.chaos > 0 {
                categories.push(CombinationConductCategory::AttackAttribute(
                    Attribute::Chaos,
                ));
            }
        }
        // 支援
        if let ArtPotency::Support(_) = &rank.potency {
            categories.push(CombinationConductCategory::Support);
        }
        // アーツ、基礎
        if conduct.art.art_type == ArtType::Basic {
            categories.push(CombinationConductCategory::ArtBasic);
        }
        // アーツ、技
        if conduct.art.art_type == ArtType::Skill {
            categories.push(CombinationConductCategory::ArtSkill);
        }
        // アーツ、術
        if conduct.art.art_type == ArtType::Sorcery {
            categories.push(CombinationConductCategory::ArtSorcery);
        }

        // ガード
        if conduct.art.perks.contains(&ArtPerk::Guard) {
            combination_skill
                .add_current_conduct_categories(vec![CombinationConductCategory::Guard]);
        }

        combination_skill.add_current_conduct_categories(categories);

        // コンビネーション技判定
        // TODO: このタイミングの効果発動の場合、能力補正を適応できないのでどうするか要検討
        if combination_skill.can_activate_combination_skill() {
            // コンビネーション技発動
            match &combination_skill.combination_skill.effect {
                HeartCombinationEffect::AttackDamageModifier(modifier) => {
                    // 与ダメージ補正効果追加
                    attacker_effects.push(Effect::AttackDamageModifier(
                        EffectAttackDamageModifier {
                            modifier: modifier.modifier,
                        },
                    ));
                }
                HeartCombinationEffect::AttackBreakDamageModifier(modifier) => {
                    // 与ブレイクダメージ補正効果追加
                    attacker_effects.push(Effect::AttackBreakDamageModifier(
                        EffectAttackBreakDamageModifier {
                            modifier: modifier.modifier,
                        },
                    ));
                }
            }
        }
    }

    // 攻撃者データ準備
    let attacker_data = AttackerData {
        character_id: attacker_id,
        conduct,

        character_type: attacker.character_type.clone(),
        hp: attacker.hp.clone(),
        sp: attacker.sp.clone(),
        stamina: attacker.stamina.clone(),
        break_resistance: attacker.break_resistance.clone(),
        status_ailment: attacker.status_ailment.clone(),
        status_conditions: attacker.status_conditions.clone(),

        final_ability: attacker_ability,
        weapon_performance: attacker_weapon_performance,
        art_rank: rank,

        effects: attacker_effects,
    };

    // 行動成否判定
    if let Some(failure_reason) = determine_action_outcome_failure(&attacker_data) {
        // コンビネーションログに失敗追加
        if let Some(combination_skill) = &mut attacker.combination_skill {
            combination_skill.add_current_conduct_result(CombinationConductResult::Failed);
        }

        // TODO: 不発理由に応じた処理
        return BattleIncidentConduct {
            actor_character_id: attacker_id,
            target: attacker_data.conduct.target.clone(),
            conduct: attacker_data.conduct,
            outcome: BattleIncidentConductOutcome::Failure(BattleIncidentConductOutcomeFailure {
                reason: failure_reason,
            }),
        };
    } else {
        // コンビネーションログに成功追加
        if let Some(combination_skill) = &mut attacker.combination_skill {
            combination_skill.add_current_conduct_result(CombinationConductResult::Success);
        }
    }

    // 攻撃者インシデントの準備
    let mut attacker_incident_character = BattleIncidentCharacter::new(attacker_id);
    let mut attacker_incident =
        BattleCharacterIncident::new(BattleCharacterIncidentReason::ConductConsumption);

    // SP消費
    let sp_cost = attacker_data.conduct.art.sp_cost;
    let (before_sp, after_sp) = attacker.sp.damage(sp_cost);
    // インシデント
    attacker_incident.add_concrete(BattleCharacterIncidentConcrete::DamageSp(
        BattleIncidentDamageSp::new(sp_cost, before_sp, after_sp),
    ));

    // スタミナ消費
    if attacker.character_type == BattleCharacterType::Player {
        // プレイヤーの場合のみスタミナ消費処理
        let stamina_cost = attacker_data.conduct.art.stamina_cost;
        let (before_stamina, after_stamina) = attacker.stamina.damage(stamina_cost);
        // インシデント
        attacker_incident.add_concrete(BattleCharacterIncidentConcrete::DamageStamina(
            BattleIncidentDamageStamina::new(stamina_cost, before_stamina, after_stamina),
        ));
    }

    // ターゲットの決定
    let target_character_ids = determine_targets(
        battle,
        &attacker_data.conduct.target,
        &attacker_data.art_rank.target,
    );

    // ターゲットごとに効果処理
    let mut target_incidents = Vec::new();
    for target_id in target_character_ids.iter() {
        let target = if let Some(character) = battle.character_mut(&target_id) {
            character
        } else {
            // TODO: エラー処理
            panic!("Target not found");
        };

        let mut target_incident_character = BattleIncidentCharacter::new(target.character_id);

        // 回避判定
        let mut is_evaded = false;
        for se in target.status_conditions.iter() {
            match &se.potency {
                StatusConditionPotency::Evasion => {
                    // 回避効果処理
                    is_evaded = true;
                    break;
                }
                StatusConditionPotency::Airborne => {
                    // 空中効果処理
                    // 遠距離攻撃でない時は回避
                    if !attacker_data.conduct.art.perks.contains(&ArtPerk::Ranged) {
                        is_evaded = true;
                        break;
                    }
                }
                StatusConditionPotency::Floating => {
                    // 浮遊効果処理
                    // 足元攻撃は回避
                    if attacker_data.conduct.art.perks.contains(&ArtPerk::AtFeet) {
                        is_evaded = true;
                        break;
                    }
                }
                StatusConditionPotency::Ranged => {
                    // 遠距離効果処理
                    // 近距離の攻撃を回避
                    if !attacker_data.conduct.art.perks.contains(&ArtPerk::Ranged) {
                        is_evaded = true;
                        break;
                    }
                }
                _ => {
                    // その他
                }
            }
        }
        if is_evaded {
            // 回避された場合の処理
            // 防御者インシデント作成
            target_incidents.push(BattleIncidentConductOutcomeSuccessDefender {
                character: target_incident_character,
                is_evaded: true,
                is_defended: false,
                is_dead: false,
            });
            // 回避して、効果処理は行わない
            continue;
        }

        // 発生インシデント
        let mut target_character_incident =
            BattleCharacterIncident::new(BattleCharacterIncidentReason::ConductEffect);

        // 効果処理
        match &attacker_data.art_rank.potency {
            ArtPotency::Attack(art_attack) => {
                // 攻撃効果処理

                // 状態異常、カルマ等の効果を取得する
                let target_effects = target.current_effects();

                let target_data = TargetData {
                    target,
                    effects: target_effects,
                };
                let attack_incidents =
                    execute_attack_potency(art_attack, &attacker_data, target_data);

                // インシデント
                for incident in attack_incidents.into_iter() {
                    target_character_incident.add_concrete(incident);
                }
                target_incident_character.add_incident(target_character_incident);
                target_incidents.push(BattleIncidentConductOutcomeSuccessDefender {
                    character: target_incident_character,
                    is_evaded: false,
                    is_defended: false,
                    is_dead: target.hp.current_hp == 0, // TODO: 戦闘不能判定
                });
            }
            ArtPotency::Support(support) => {
                // 支援効果処理
                let support_incidents = execute_support_potency(support, target);
                for incident in support_incidents.into_iter() {
                    target_character_incident.add_concrete(incident);
                }
                target_incident_character.add_incident(target_character_incident);
                target_incidents.push(BattleIncidentConductOutcomeSuccessDefender {
                    character: target_incident_character,
                    is_evaded: false,
                    is_defended: false,
                    is_dead: false,
                });
            }
        }
    }

    attacker_incident_character.add_incident(attacker_incident);
    BattleIncidentConduct {
        actor_character_id: attacker_id,
        target: attacker_data.conduct.target.clone(),
        conduct: attacker_data.conduct,
        outcome: BattleIncidentConductOutcome::Success(BattleIncidentConductOutcomeSuccess {
            attacker: attacker_incident_character,
            defenders: target_incidents,
        }),
    }
}

// ターゲット決定
// ターゲット範囲が変化している場合、それに応じてターゲットを変更
fn determine_targets(
    battle: &Battle,
    conduct_target: &BattleConductTargetType,
    art_target: &ArtTarget,
) -> Vec<BattleCharacterId> {
    let conduct_target = match conduct_target {
        BattleConductTargetType::Player => {
            // プレイヤー
            // そのまま
            &BattleConductTargetType::Player
        }
        BattleConductTargetType::EnemySingle(_) => {
            if art_target == &ArtTarget::All {
                &BattleConductTargetType::EnemyAll
            } else {
                conduct_target
            }
        }
        BattleConductTargetType::EnemyAll => {
            if art_target == &ArtTarget::Single {
                if let Some(character) = battle.enemies.first() {
                    let target_character_id = character.character_id;
                    &BattleConductTargetType::EnemySingle(target_character_id)
                } else {
                    // TODO: エラー処理
                    panic!("No enemy characters available");
                }
            } else {
                conduct_target
            }
        }
    };

    // ターゲットIDリスト取得
    let target_character_ids = match conduct_target {
        BattleConductTargetType::Player => {
            vec![battle.player.character_id]
        }
        BattleConductTargetType::EnemySingle(character_id) => {
            if let Some(character) = battle.character(&character_id) {
                vec![character.character_id]
            } else {
                // TODO: エラー処理
                panic!("Defender not found");
            }
        }
        BattleConductTargetType::EnemyAll => {
            // enemiesのcharacter_id全て
            battle.enemies.iter().map(|c| c.character_id).collect()
        }
    };

    target_character_ids
}

// 素手の攻撃性能取得
fn unarmed_weapon_performance() -> WeaponPerformance {
    // TODO: 仮
    WeaponPerformance {
        attack_power: AttackPower {
            slash: 0,
            strike: 10,
            thrust: 0,
            impact: 0,
            magic: 0,
            fire: 0,
            lightning: 0,
            chaos: 0,
        },
        ability_attack_power: AttackPower::default(),
        sorcery_power: 0,
        ability_sorcery_power: 0,
        break_power: 0,
        ability_break_power: 0,
        guard_strength: 10,
        penalty: None,
    }
}

fn create_battle_status_condition(status_condition: &StatusCondition) -> BattleStatusCondition {
    let duration = match &status_condition.duration {
        StatusConditionDuration::Permanent => BattleStatusConditionDuration::Permanent,
        StatusConditionDuration::Turn(d) => {
            BattleStatusConditionDuration::Turn(BattleStatusConditionDurationTurn {
                turns: d.turns,
                elapsed_turns: 0,
            })
        }
        StatusConditionDuration::Count(d) => {
            BattleStatusConditionDuration::Count(BattleStatusConditionDurationCount {
                count: d.count,
                elapsed_count: 0,
            })
        }
        StatusConditionDuration::UntilNextAction => BattleStatusConditionDuration::UntilNextAction,
    };
    BattleStatusCondition {
        potency: status_condition.potency.clone(),
        duration,
    }
}

// 行動成否判定
/// 不発の場合、理由を返す
/// 発動の場合、Noneを返す
fn determine_action_outcome_failure(
    attacker_data: &AttackerData,
) -> Option<BattleIncidentConductOutcomeFailureReason> {
    let conduct = &attacker_data.conduct;

    if attacker_data.character_type == BattleCharacterType::Player {
        // プレイヤーキャラクターの場合のみスタミナチェック

        // スタミナが足りないと不発
        if attacker_data.stamina.current_stamina < conduct.art.stamina_cost {
            return Some(BattleIncidentConductOutcomeFailureReason::InsufficientStamina);
        }
    }

    // 行動不能効果チェック
    for effect in attacker_data.effects.iter() {
        match effect {
            Effect::UnableToAct => {
                // TODO: インシデント調整
                // ブレイク中行動不能
                return Some(BattleIncidentConductOutcomeFailureReason::IsBreak);
            }
            _ => {}
        }
    }

    // 必要能力が足りないと不発
    let req = &conduct.art.requirement;
    let ability = &attacker_data.final_ability;
    if ability.strength < req.strength
        || ability.dexterity < req.dexterity
        || ability.intelligence < req.intelligence
        || ability.faith < req.faith
        || ability.arcane < req.arcane
        || ability.agility < req.agility
    {
        return Some(BattleIncidentConductOutcomeFailureReason::InsufficientAbility);
    }

    // SPが足りないと不発
    let sp_cost = conduct.art.sp_cost;
    if attacker_data.sp.current_sp < sp_cost {
        return Some(BattleIncidentConductOutcomeFailureReason::InsufficientSp);
    }

    None
}
