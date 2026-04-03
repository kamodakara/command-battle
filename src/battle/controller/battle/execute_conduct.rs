mod conduct_effect;
mod execute_attack_potency;
mod execute_support_potency;

use execute_attack_potency::execute_attack_potency;
use execute_support_potency::execute_support_potency;

use super::*;

// 攻撃者データ
// 判定や計算に必要なデータをまとめる
// 行動前のデータ
#[derive(Debug)]
struct AttackerData {
    character_id: BattleCharacterId,
    conduct: BattleConduct,

    character_type: BattleCharacterType,
    hp: BattleCharacterHP,
    sp: BattleCharacterSP,
    stamina: BattleCharacterStamina,
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

    // フェーズ1: 攻撃者に関する処理（attacker への可変借用はここで完結させる）
    let attacker = battle
        .character_mut(&conduct.actor_character_id)
        .expect("Attacker not found");
    let (attacker_data, mut attacker_incident, mut attacker_incident_character) =
        prepare_attacker(attacker, conduct);

    // 行動成否判定
    if let Some(failure_reason) = determine_action_outcome_failure(&attacker_data) {
        record_combination_result(
            battle,
            &attacker_data.character_id,
            CombinationConductResult::Failed,
        );
        return make_failure_incident(attacker_data, failure_reason);
    }
    record_combination_result(
        battle,
        &attacker_data.character_id,
        CombinationConductResult::Success,
    );

    // リソース消費
    {
        let attacker = battle
            .character_mut(&attacker_data.character_id)
            .expect("Attacker not found");
        consume_resources(attacker, &attacker_data, &mut attacker_incident);
    }

    // フェーズ2: ターゲット処理（battle の借用を再取得）
    let target_ids = determine_targets(
        battle,
        &attacker_data.conduct.target,
        &attacker_data.art_rank.target,
    );
    let target_incidents = apply_target_effects(battle, &attacker_data, target_ids);

    // 攻撃アーツの使用者への追加効果
    let user_additional_incidents = apply_user_additional_effects(battle, &attacker_data);
    if !user_additional_incidents.is_empty() {
        let mut user_incident =
            BattleCharacterIncident::new(BattleCharacterIncidentReason::ConductEffect);
        user_incident.extend_concretes(user_additional_incidents);
        attacker_incident_character.add_incident(user_incident);
    }

    attacker_incident_character.add_incident(attacker_incident);
    BattleIncidentConduct {
        actor_character_id: attacker_data.character_id,
        target: attacker_data.conduct.target.clone(),
        conduct: attacker_data.conduct,
        outcome: BattleIncidentConductOutcome::Success(BattleIncidentConductOutcomeSuccess {
            attacker: attacker_incident_character,
            defenders: target_incidents,
        }),
    }
}

// 攻撃者データ準備
fn prepare_attacker(
    attacker: &mut BattleCharacter,
    conduct: BattleConduct,
) -> (
    AttackerData,
    BattleCharacterIncident,
    BattleIncidentCharacter,
) {
    let attacker_id = attacker.character_id;
    let attacker_incident_character = BattleIncidentCharacter::new(attacker_id);
    let mut attacker_incident =
        BattleCharacterIncident::new(BattleCharacterIncidentReason::ConductConsumption);

    let mut effects = attacker.current_effects();
    let ability = attacker.ability_with_effects(&effects);
    let weapon_performance = resolve_weapon_performance(attacker, &conduct, &ability);
    let sorcery_power = weapon_performance.final_sorcery_power();
    let rank = conduct.art.effective_rank(sorcery_power).clone();

    // TODO: コンビネーション処理はいったんコメントアウト
    // // コンビネーション技処理
    // process_combination_skills(
    //     attacker,
    //     &rank,
    //     &conduct,
    //     &weapon_performance,
    //     &mut effects,
    //     &mut attacker_incident,
    // );

    let attacker_data = AttackerData {
        character_id: attacker_id,
        conduct,
        character_type: attacker.character_type.clone(),
        hp: attacker.hp.clone(),
        sp: attacker.sp.clone(),
        stamina: attacker.stamina.clone(),
        status_ailment: attacker.status_ailment.clone(),
        status_conditions: attacker.status_conditions.clone(),
        final_ability: ability,
        weapon_performance,
        art_rank: rank,
        effects,
    };

    (
        attacker_data,
        attacker_incident,
        attacker_incident_character,
    )
}

// 武器性能解決
fn resolve_weapon_performance(
    attacker: &BattleCharacter,
    conduct: &BattleConduct,
    ability: &Ability,
) -> WeaponPerformance {
    match &conduct.battle_weapon_id {
        Some(weapon_id) => attacker
            .weapon(weapon_id)
            .expect("Weapon not found")
            .weapon
            .performance(ability),
        None => WeaponPerformance::unarmed_weapon_performance(),
    }
}

// コンビネーション技処理
fn process_combination_skills(
    attacker: &mut BattleCharacter,
    rank: &ArtRank,
    conduct: &BattleConduct,
    weapon_performance: &WeaponPerformance,
    effects: &mut Vec<Effect>,
    incident: &mut BattleCharacterIncident,
) {
    let Some(combination_skill) = &mut attacker.combination_skill else {
        return;
    };

    let categories = collect_conduct_categories(rank, conduct, weapon_performance);
    if conduct.art.perks.contains(&ArtPerk::Guard) {
        combination_skill.add_current_conduct_categories(vec![CombinationConductCategory::Guard]);
    }
    combination_skill.add_current_conduct_categories(categories);

    for cs in combination_skill.activate_combination_skills() {
        for effect in cs.effects.iter() {
            match effect {
                HeartCombinationEffect::AttackDamageModifier(m) => {
                    effects.push(Effect::AttackDamageModifier(EffectAttackDamageModifier {
                        modifier: m.modifier,
                    }));
                }
                HeartCombinationEffect::AttackBreakDamageModifier(m) => {
                    effects.push(Effect::AttackBreakDamageModifier(
                        EffectAttackBreakDamageModifier {
                            modifier: m.modifier,
                        },
                    ));
                }
            }
        }
        incident.add_concrete(BattleCharacterIncidentConcrete::CombinationSkillActivated(
            BattleIncidentCombinationSkillActivated {
                combination_skill_name: cs.name.clone(),
            },
        ));
    }
}

// コンビネーション行動カテゴリ収集
fn collect_conduct_categories(
    rank: &ArtRank,
    conduct: &BattleConduct,
    weapon_performance: &WeaponPerformance,
) -> Vec<CombinationConductCategory> {
    let mut categories = vec![];

    if let ArtPotency::Attack(potency) = &rank.potency {
        categories.push(CombinationConductCategory::Attack);
        let weapon_attack_power = weapon_performance.final_attack_power();
        let attack_power = potency.final_attack_power(&weapon_attack_power);
        // 攻撃属性ごとのカテゴリ追加
        // TODO: 同じことをダメージ計算時にやってるの1回だけにするようにするか検討
        let attr_powers = [
            (attack_power.slash, Attribute::Slash),
            (attack_power.strike, Attribute::Strike),
            (attack_power.thrust, Attribute::Thrust),
            (attack_power.impact, Attribute::Impact),
            (attack_power.magic, Attribute::Magic),
            (attack_power.fire, Attribute::Fire),
            (attack_power.lightning, Attribute::Lightning),
            (attack_power.chaos, Attribute::Chaos),
        ];
        for (power, attr) in attr_powers {
            if power > 0 {
                categories.push(CombinationConductCategory::AttackAttribute(attr));
            }
        }
    }
    if let ArtPotency::Support(_) = &rank.potency {
        categories.push(CombinationConductCategory::Support);
    }
    match conduct.art.art_type {
        ArtType::Basic => categories.push(CombinationConductCategory::ArtBasic),
        ArtType::Skill => categories.push(CombinationConductCategory::ArtSkill),
        ArtType::Sorcery => categories.push(CombinationConductCategory::ArtSorcery),
    }
    categories
}

// コンビネーション結果記録
fn record_combination_result(
    battle: &mut Battle,
    attacker_id: &BattleCharacterId,
    result: CombinationConductResult,
) {
    if let Some(attacker) = battle.character_mut(attacker_id) {
        if let Some(combination_skill) = &mut attacker.combination_skill {
            combination_skill.add_current_conduct_result(result);
        }
    }
}

// 行動失敗インシデント作成
fn make_failure_incident(
    attacker_data: AttackerData,
    failure_reason: BattleIncidentConductOutcomeFailureReason,
) -> BattleIncidentConduct {
    BattleIncidentConduct {
        actor_character_id: attacker_data.character_id,
        target: attacker_data.conduct.target.clone(),
        conduct: attacker_data.conduct,
        outcome: BattleIncidentConductOutcome::Failure(BattleIncidentConductOutcomeFailure {
            reason: failure_reason,
        }),
    }
}

// リソース消費（SP・スタミナ）
fn consume_resources(
    attacker: &mut BattleCharacter,
    attacker_data: &AttackerData,
    incident: &mut BattleCharacterIncident,
) {
    let sp_cost = attacker_data.conduct.art.sp_cost;
    let (before_sp, after_sp) = attacker.sp.damage(sp_cost);
    incident.add_concrete(BattleCharacterIncidentConcrete::DamageSp(
        BattleIncidentDamageSp::new(sp_cost, before_sp, after_sp),
    ));

    if attacker_data.character_type == BattleCharacterType::Player {
        // プレイヤーの場合のみスタミナ消費処理
        let stamina_cost = attacker_data.conduct.art.stamina_cost;
        let (before_stamina, after_stamina) = attacker.stamina.damage(stamina_cost);
        incident.add_concrete(BattleCharacterIncidentConcrete::DamageStamina(
            BattleIncidentDamageStamina::new(stamina_cost, before_stamina, after_stamina),
        ));
    }
}

// ターゲットごとの効果処理
fn apply_target_effects(
    battle: &mut Battle,
    attacker_data: &AttackerData,
    target_ids: Vec<BattleCharacterId>,
) -> Vec<BattleIncidentConductOutcomeSuccessDefender> {
    let mut incidents = Vec::new();
    for target_id in &target_ids {
        let target = battle.character_mut(target_id).expect("Target not found");
        incidents.push(apply_effect_to_target(target, attacker_data));
    }
    incidents
}

// 単一ターゲットへの効果適用
fn apply_effect_to_target(
    target: &mut BattleCharacter,
    attacker_data: &AttackerData,
) -> BattleIncidentConductOutcomeSuccessDefender {
    let mut target_incident_character = BattleIncidentCharacter::new(target.character_id);

    // 回避判定
    if !attacker_data.conduct.art.always_hits && is_evaded(target, attacker_data) {
        return BattleIncidentConductOutcomeSuccessDefender {
            character: target_incident_character,
            is_evaded: true,
            is_dead: false,
        };
    }

    let mut target_character_incident =
        BattleCharacterIncident::new(BattleCharacterIncidentReason::ConductEffect);

    // ターゲットのステータスや状態変化を取得
    let target_effects = target.current_effects();
    let target_data = TargetData {
        target,
        effects: target_effects,
    };

    match &attacker_data.art_rank.potency {
        ArtPotency::Attack(art_attack) => {
            let attack_incidents = execute_attack_potency(art_attack, attacker_data, target_data);
            // Death インシデントの有無で戦闘不能を判定する
            let is_dead = attack_incidents
                .iter()
                .any(|i| matches!(i, BattleCharacterIncidentConcrete::Death(_)));
            for incident in attack_incidents {
                target_character_incident.add_concrete(incident);
            }
            target_incident_character.add_incident(target_character_incident);
            BattleIncidentConductOutcomeSuccessDefender {
                character: target_incident_character,
                is_evaded: false,
                is_dead,
            }
        }
        ArtPotency::Support(art_support) => {
            for incident in execute_support_potency(art_support, attacker_data, target_data) {
                target_character_incident.add_concrete(incident);
            }
            target_incident_character.add_incident(target_character_incident);
            BattleIncidentConductOutcomeSuccessDefender {
                character: target_incident_character,
                is_evaded: false,
                is_dead: false,
            }
        }
    }
}

// 回避判定
fn is_evaded(target: &BattleCharacter, attacker_data: &AttackerData) -> bool {
    let is_ranged = attacker_data.conduct.art.perks.contains(&ArtPerk::Ranged);
    let at_feet = attacker_data.conduct.art.perks.contains(&ArtPerk::AtFeet);

    target.status_conditions.iter().any(|se| match &se.potency {
        StatusConditionPotency::Evasion => true,
        StatusConditionPotency::Airborne => !is_ranged,
        StatusConditionPotency::Floating => at_feet,
        StatusConditionPotency::Ranged => !is_ranged,
        _ => false,
    })
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
    match conduct_target {
        BattleConductTargetType::Player => {
            vec![battle.player.character_id]
        }
        BattleConductTargetType::EnemySingle(character_id) => {
            // TODO: 指定された敵がすでに死亡していた場合、生存している別の敵をターゲットにする
            if let Some(character) = battle.character(character_id) {
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

// 攻撃アーツ 使用者への追加効果適用
fn apply_user_additional_effects(
    battle: &mut Battle,
    attacker_data: &AttackerData,
) -> Vec<BattleCharacterIncidentConcrete> {
    let ArtPotency::Attack(art_attack) = &attacker_data.art_rank.potency else {
        return vec![];
    };

    let mut incidents = vec![];
    for additional_effect in art_attack
        .additional_effects
        .iter()
        .filter(|e| e.target == AdditionalEffectTarget::User)
    {
        let attacker = battle
            .character_mut(&attacker_data.character_id)
            .expect("Attacker not found");
        let attacker_effects = attacker.current_effects();
        let target_data = TargetData {
            target: attacker,
            effects: attacker_effects,
        };
        incidents.extend(execute_support_potency(
            &additional_effect.content,
            attacker_data,
            target_data,
        ));
    }
    incidents
}

// 状態異常蓄積
pub(super) fn accumulate_status_ailment(
    target: &mut BattleCharacter,
    ailment: &StatusAilment,
    ailment_accumulation: u32, // 蓄積量
) -> Vec<BattleCharacterIncidentConcrete> {
    let mut incidents = vec![];

    if ailment == &StatusAilment::Breaking && target.status_ailment.breaking.is_ailment {
        // ブレイク状態の場合は蓄積しない
        return incidents;
    }

    let status = match ailment {
        StatusAilment::Poison => &mut target.status_ailment.poison,
        StatusAilment::Sleep => &mut target.status_ailment.sleep,
        StatusAilment::Chill => &mut target.status_ailment.chill,
        StatusAilment::Bleed => &mut target.status_ailment.bleed,
        StatusAilment::Burn => &mut target.status_ailment.burn,
        StatusAilment::Paralysis => &mut target.status_ailment.paralysis,
        StatusAilment::Fear => &mut target.status_ailment.fear,
        StatusAilment::Rage => &mut target.status_ailment.rage,
        StatusAilment::Breaking => &mut target.status_ailment.breaking,
    };

    let (before, after) = status.add_accumulation(ailment_accumulation);
    // 蓄積インシデント
    incidents.push(BattleCharacterIncidentConcrete::StatusAilmentAccumulation(
        BattleIncidentStatusAilmentAccumulation {
            status_ailment: ailment.clone(),
            accumulation: ailment_accumulation,
            before_accumulation: before,
            after_accumulation: after,
        },
    ));
    // 蓄積ターンリセット
    status.no_accumulation_turns = 0;

    if !status.is_ailment && after == status.max_accumulation {
        // 状態異常でない場合、蓄積
        status.is_ailment = true;

        // 効果発動
        let effects = ailment.on_ailment_effects();
        for effect in effects.iter() {
            match effect {
                BattleStatusAilmentOnAilmentEffect::HpPercentageDamage(effect) => {
                    // HP割合ダメージ
                    let damage = (target.hp.max_hp as f32 * effect.percentage) as u32;
                    let (before_hp, after_hp, is_dead) = target.hp.damage(damage);

                    // HPダメージインシデント
                    incidents.push(BattleCharacterIncidentConcrete::DamageHp(
                        BattleIncidentDamageHp::new(damage, before_hp, after_hp),
                    ));
                    if is_dead {
                        // 死亡インシデント
                        incidents.push(BattleCharacterIncidentConcrete::Death(
                            BattleIncidentDeath {},
                        ));
                    }
                }
                BattleStatusAilmentOnAilmentEffect::SpPercentageDamage(effect) => {
                    // SP割合ダメージ
                    let damage = (target.sp.max_sp as f32 * effect.percentage) as u32;
                    let (before_sp, after_sp) = target.sp.damage(damage);

                    // SPダメージインシデント
                    incidents.push(BattleCharacterIncidentConcrete::DamageSp(
                        BattleIncidentDamageSp::new(damage, before_sp, after_sp),
                    ));
                }
                _ => {
                    // その他
                }
            }
        }

        // 状態異常付与インシデント
        incidents.push(BattleCharacterIncidentConcrete::StatusAilmentApplied(
            BattleIncidentStatusAilmentApplied {
                status_ailment: ailment.clone(),
            },
        ));
    }

    incidents
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
        if let Effect::UnableToAct = effect {
            // ブレイク中行動不能
            return Some(BattleIncidentConductOutcomeFailureReason::IsBreak);
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
