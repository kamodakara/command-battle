use std::rc::Rc;

use bevy::asset::ron::de;

// 敵
struct Enemy {
    ability: EnemyAbility, // 能力
    stats: EnemyStats,     // ステータス
    equipment: Equipment,  // 装備
}

// 敵能力
struct EnemyAbility {
    agility: u32,      // 敏捷性
    strength: u32,     // 筋力
    dexterity: u32,    // 技量
    intelligence: u32, // 知力
    faith: u32,        // 信仰
    arcane: u32,       // 神秘
}

struct EnemyStats {
    hp: u32,             // HP
    sp: u32,             // SP
    break_max: u32,      // ブレイク最大値
    break_recovery: u32, // ブレイク回復量
    break_turn: u32,     // ブレイクターン
}

struct Player {
    ability: PlayerAbility, // 能力
    stats: PlayerStats,     // ステータス
    equipment: Equipment,   // 装備
}
// プレイヤー能力
struct PlayerAbility {
    vitality: u32,     // 生命力
    spirit: u32,       // 精神力
    endurance: u32,    // 持久力
    agility: u32,      // 敏捷性
    strength: u32,     // 筋力
    dexterity: u32,    // 技量
    intelligence: u32, // 知力
    faith: u32,        // 信仰
    arcane: u32,       // 神秘
}

// 状態異常
struct StatusAilment {
    // TODO
}

// 状態変化
struct StatusCondition {
    // TODO
}

// プレイヤーステータス
struct PlayerStats {
    hp: u32,         // HP
    sp: u32,         // SP
    stamina: u32,    // スタミナ
    equip_load: u32, // 装備重量
}

// プレイヤー装備
struct Equipment {
    weapon1: Option<Weapon>, // 右手
    weapon2: Option<Weapon>, // 左手
    armor1: Option<Armor>,   // 防具
    armor2: Option<Armor>,   // 防具
    armor3: Option<Armor>,   // 防具
    armor4: Option<Armor>,   // 防具
    armor5: Option<Armor>,   // 防具
    armor6: Option<Armor>,   // 防具
    armor7: Option<Armor>,   // 防具
    armor8: Option<Armor>,   // 防具
}

// 属性
enum Attribute {
    Slash,     // 斬撃
    Strike,    // 打撃
    Thrust,    // 刺突
    Impact,    // 衝撃
    Magic,     // 魔力
    Fire,      // 炎
    Lightning, // 雷
    Chaos,     // 混濁
}

// 武器
struct Weapon {
    kind: WeaponKind,                              // 種類
    weight: u32,                                   // 重量
    ability_requirement: WeaponAbilityRequirement, // 必要能力
    attack_power: WeaponAttackPower,               // 攻撃性能
    sorcery_power: WeaponSorceryPower,             // 術力
    break_power: WeaponBreakPower,                 // ブレイク力
    guard: WeaponGuard,                            // 防御性能
}

enum WeaponKind {
    StraightSword, // 直剣
    Greatsword,    // 大剣
    Spear,         // 槍
    Axe,           // 斧
    Hammer,        // ハンマー
    Bow,           // 弓
    Crossbow,      // クロスボウ
    Staff,         // 杖
    Shield,        // 盾
}

// 武器必要能力
struct WeaponAbilityRequirement {
    strength: u32,     // 筋力
    dexterity: u32,    // 技量
    intelligence: u32, // 知力
    faith: u32,        // 信仰
    arcane: u32,       // 神秘
    agility: u32,      // 敏捷性
}

// 武器攻撃性能
struct WeaponAttackPower {
    base: AttackPower,                                // 基礎攻撃力
    ability_scaling: WeaponAttackPowerAbilityScaling, // 能力補正
}

// 能力補正
struct AbilityScaling {
    strength: f32,     // 筋力
    dexterity: f32,    // 技量
    intelligence: f32, // 知力
    faith: f32,        // 信仰
    arcane: f32,       // 神秘
    agility: f32,      // 敏捷性
}

// 武器攻撃力能力補正
struct WeaponAttackPowerAbilityScaling {
    slash: AbilityScaling,     // 斬撃
    strike: AbilityScaling,    // 打撃
    thrust: AbilityScaling,    // 刺突
    impact: AbilityScaling,    // 衝撃
    magic: AbilityScaling,     // 魔力
    fire: AbilityScaling,      // 炎
    lightning: AbilityScaling, // 雷
    chaos: AbilityScaling,     // 混濁
}

// 武器術力
struct WeaponSorceryPower {
    base: u32,               // 基礎力
    scaling: AbilityScaling, // 能力補正
}

// 武器ブレイク力
struct WeaponBreakPower {
    base_power: u32,         // 基礎力
    scaling: AbilityScaling, // 能力補正
}

// 武器防御性能
struct WeaponGuard {
    cut_rate: GuardCutRate, // カット率
    guard_strength: u32,    // ガード強度
}

// 武器防御カット率
// 攻撃力補正値が入る0.0〜1.0の範囲
// 0.0で100%カット
#[derive(Clone)]
struct GuardCutRate {
    slash: f32,     // 斬撃
    strike: f32,    // 打撃
    thrust: f32,    // 刺突
    impact: f32,    // 衝撃
    magic: f32,     // 魔力
    fire: f32,      // 炎
    lightning: f32, // 雷
    chaos: f32,     // 混濁
}

// 防具
struct Armor {
    kind: ArmorKind,
    // 種類
    weight: u32,                 // 重量
    defense: ArmorDefense,       // 防御力
    resistance: ArmorResistance, // 状態異常耐性値
}

// 防具種類
enum ArmorKind {
    Helmet,     // 頭装備
    ChestArmor, // 胴装備
    Gauntlets,  // 腕装備
    LegArmor,   // 脚装備
}

// 防具防御力
struct ArmorDefense {
    slash: u32,     // 斬撃
    strike: u32,    // 打撃
    thrust: u32,    // 刺突
    impact: u32,    // 衝撃
    magic: u32,     // 魔力
    fire: u32,      // 炎
    lightning: u32, // 雷
    chaos: u32,     // 混濁
}
// 防具状態異常耐性値
struct ArmorResistance {
    immunity: u32,   // 免疫
    robustness: u32, // 頑健
    sanity: u32,     // 正気
}

struct BattleAbility {
    agility: u32,      // 敏捷性
    strength: u32,     // 筋力
    dexterity: u32,    // 技量
    intelligence: u32, // 知力
    faith: u32,        // 信仰
    arcane: u32,       // 神秘
}

struct BattleStats {
    max_hp: u32,         // HP
    max_sp: u32,         // SP
    max_stamina: u32,    // スタミナ 敵は使用しない
    hp_damage: u32,      // 受けたHPダメージ
    sp_damage: u32,      // 受けたSPダメージ
    stamina_damage: u32, // 受けたスタミナダメージ
}
struct BattleEnemyOnlyStats {
    max_break: u32,      // ブレイク最大値
    break_recovery: u32, // ブレイク回復量
    break_max_turn: u32, // ブレイク最大ターン
    break_damage: u32,   // 受けたブレイクダメージ
    break_turns: u32,    // 現在のブレイク経過ターン
}

// 戦闘中の状態変化
#[derive(Clone)]
struct BattleStatusEffect {
    potency: StatusEffectPotency,         // 状態変化効果
    duration: BattleStatusEffectDuration, // 継続時間
}
#[derive(Clone)]
enum BattleStatusEffectDuration {
    Permanent,                              // 永続
    Turn(BattleStatusEffectDurationTurn),   // ターン数
    Count(BattleStatusEffectDurationCount), // 回数
    UntilNextAction,                        // 次の行動まで
}
#[derive(Clone)]
struct BattleStatusEffectDurationTurn {
    // 効果ターン数
    turns: u32,
    // 経過ターン数
    elapsed_turns: u32,
}
#[derive(Clone)]
struct BattleStatusEffectDurationCount {
    // 効果回数
    count: u32,
    // 経過回数
    elapsed_count: u32,
}

fn create_battle_status_effect(status_effect: &StatusEffect) -> BattleStatusEffect {
    let duration = match &status_effect.duration {
        StatusEffectDuration::Permanent => BattleStatusEffectDuration::Permanent,
        StatusEffectDuration::Turn(d) => {
            BattleStatusEffectDuration::Turn(BattleStatusEffectDurationTurn {
                turns: d.turns,
                elapsed_turns: 0,
            })
        }
        StatusEffectDuration::Count(d) => {
            BattleStatusEffectDuration::Count(BattleStatusEffectDurationCount {
                count: d.count,
                elapsed_count: 0,
            })
        }
        StatusEffectDuration::UntilNextAction => BattleStatusEffectDuration::UntilNextAction,
    };
    BattleStatusEffect {
        potency: status_effect.potency.clone(),
        duration,
    }
}

struct StatusEffect {
    potency: StatusEffectPotency,   // 効果量
    duration: StatusEffectDuration, // 継続時間
}

// 状態変化持続時間
enum StatusEffectDuration {
    Permanent,
    Turn(StatusEffectDurationTurn),   // ターン数
    Count(StatusEffectDurationCount), // 回数
    UntilNextAction,                  // 次の行動まで
}
struct StatusEffectDurationTurn {
    // 効果ターン数
    turns: u32,
}

struct StatusEffectDurationCount {
    // 効果回数
    count: u32,
}

#[derive(Clone)]
enum StatusEffectPotency {
    Resistance(StatusEffectResistance), // 防御
    Break(StatusEffectBreak),           // ブレイク状態
    Evasion,                            // 回避
    Airborne,                           // 空中
    Floating,                           // 浮遊
    Melee,                              // 近距離
    Ranged,                             // 遠距離
}

// 戦闘中の状態変化 防御
#[derive(Clone)]
struct StatusEffectResistance {
    cut_rate: GuardCutRate,
}
// 戦闘中の状態変化 ブレイク状態
#[derive(Clone)]
struct StatusEffectBreak {
    // TODO: 詳細な効果
    // ダメージ計算に使う値
    // ブレイク回復については別途処理で行うのでここには不要かも
}

// 戦闘行動
struct Conduct {
    name: String,                    // 名前
    sp_cost: u32,                    // SP消費
    stamina_cost: u32,               // スタミナ消費
    perks: Vec<ConductPerk>,         // 特性
    requirement: ConductRequirement, // 必要能力
    conduct_type: ConductType,       // 戦闘行動内容
}

// 戦闘行動特性
#[derive(PartialEq)]
enum ConductPerk {
    // 近距離
    Melee,
    // 遠距離
    Ranged,
    // 足元
    AtFeet,
}

// 戦闘行動必要能力
struct ConductRequirement {
    strength: u32,     // 筋力
    dexterity: u32,    // 技量
    intelligence: u32, // 知力
    faith: u32,        // 信仰
    arcane: u32,       // 神秘
    agility: u32,      // 敏捷性
}
// 戦闘行動内容
enum ConductType {
    Basic(ConductTypeBasic),     // 基本
    Skill(ConductTypeSkill),     // 技
    Sorcery(ConductTypeSorcery), // 術
}
// 戦闘行動基本
enum ConductTypeBasic {
    Attack(ConductTypeBasicAttack),   // 攻撃
    Support(ConductTypeBasicSupport), // 支援
}
struct ConductTypeBasicAttack {
    attack_power: AttackPower, // 攻撃力
    break_power: u32,          // ブレイク攻撃力
}
enum ConductTypeBasicSupport {
    StatusEffect(SuportStatusEffect), // 状態変化付与
}

struct SuportStatusEffect {
    status_effects: Vec<StatusEffect>,
}

// 戦闘行動技
struct ConductTypeSkill {
    // 使用可能武器
    usable_weapon_kinds: Vec<WeaponKind>,
    potency: ConductTypeSkillPotency,
}
enum ConductTypeSkillPotency {
    Attack(ConductTypeSkillPotencyAttack),   // 攻撃
    Support(ConductTypeSkillPotencySupport), // 支援
}
struct ConductTypeSkillPotencyAttack {
    attack_power: AttackPower,                // 基礎攻撃力
    attack_power_scaling: AttackPowerScaling, // 攻撃力補正
    break_power: u32,                         // ブレイク攻撃力
    break_power_scaling: f32,                 // ブレイク攻撃力補正
}
enum ConductTypeSkillPotencySupport {
    StatusEffect(SuportStatusEffect), // 状態変化付与
}

// 術
enum ConductTypeSorcery {
    Attack(ConductTypeSorceryAttack),   // 攻撃
    Support(ConductTypeSorcerySupport), // 支援
}
struct ConductTypeSorceryAttack {
    attack_power: AttackPower, // 基礎攻撃力
    break_power: u32,          // ブレイク攻撃力
}
enum ConductTypeSorcerySupport {
    StatusEffect(SuportStatusEffect), // 状態変化付与
}

// 攻撃力
#[derive(Clone)]
struct AttackPower {
    slash: u32,     // 斬撃
    strike: u32,    // 打撃
    thrust: u32,    // 刺突
    impact: u32,    // 衝撃
    magic: u32,     // 魔力
    fire: u32,      // 炎
    lightning: u32, // 雷
    chaos: u32,     // 混濁
}
impl AttackPower {
    fn default() -> Self {
        AttackPower {
            slash: 0,
            strike: 0,
            thrust: 0,
            impact: 0,
            magic: 0,
            fire: 0,
            lightning: 0,
            chaos: 0,
        }
    }

    // 1つの属性に加算
    fn add_attribute(&mut self, attribute: &Attribute, value: u32) {
        match attribute {
            Attribute::Slash => self.slash += value,
            Attribute::Strike => self.strike += value,
            Attribute::Thrust => self.thrust += value,
            Attribute::Impact => self.impact += value,
            Attribute::Magic => self.magic += value,
            Attribute::Fire => self.fire += value,
            Attribute::Lightning => self.lightning += value,
            Attribute::Chaos => self.chaos += value,
        }
    }

    // 倍率をかける
    fn multiply(&mut self, factor: f32) {
        self.slash = (self.slash as f32 * factor) as u32;
        self.strike = (self.strike as f32 * factor) as u32;
        self.thrust = (self.thrust as f32 * factor) as u32;
        self.impact = (self.impact as f32 * factor) as u32;
        self.magic = (self.magic as f32 * factor) as u32;
        self.fire = (self.fire as f32 * factor) as u32;
        self.lightning = (self.lightning as f32 * factor) as u32;
        self.chaos = (self.chaos as f32 * factor) as u32;
    }
}

struct AttackPowerScaling {
    slash: f32,     // 斬撃
    strike: f32,    // 打撃
    thrust: f32,    // 刺突
    impact: f32,    // 衝撃
    magic: f32,     // 魔力
    fire: f32,      // 炎
    lightning: f32, // 雷
    chaos: f32,     // 混濁
}
impl AttackPowerScaling {
    fn default() -> Self {
        AttackPowerScaling {
            slash: 0.0,
            strike: 0.0,
            thrust: 0.0,
            impact: 0.0,
            magic: 0.0,
            fire: 0.0,
            lightning: 0.0,
            chaos: 0.0,
        }
    }

    // 1つの属性に加算
    fn add_attribute(&mut self, attribute: &Attribute, value: f32) {
        match attribute {
            Attribute::Slash => self.slash += value,
            Attribute::Strike => self.strike += value,
            Attribute::Thrust => self.thrust += value,
            Attribute::Impact => self.impact += value,
            Attribute::Magic => self.magic += value,
            Attribute::Fire => self.fire += value,
            Attribute::Lightning => self.lightning += value,
            Attribute::Chaos => self.chaos += value,
        }
    }
}

struct DefensePower {
    slash: u32,     // 斬撃
    strike: u32,    // 打撃
    thrust: u32,    // 刺突
    impact: u32,    // 衝撃
    magic: u32,     // 魔力
    fire: u32,      // 炎
    lightning: u32, // 雷
    chaos: u32,     // 混濁
}

// 戦闘者
struct BattleCharacter {
    id: u32,
    current_ability: BattleAbility,
    current_stats: BattleStats,
    defense_power: DefensePower,
    status_effects: Vec<BattleStatusEffect>,

    character_type: BattleCharacterType,
}
enum BattleCharacterType {
    Player(BattlePlayer),
    Enemy(BattleEnemy),
}

struct Battle {
    players: Vec<BattleCharacter>,
    enemies: Vec<BattleCharacter>,
}

// バトル中のプレイヤーの状態
struct BattlePlayer {
    original: Rc<Player>,
}
// バトル中の敵の状態
struct BattleEnemy {
    original: Rc<Enemy>,
    current_enemy_only_stats: BattleEnemyOnlyStats,
}

struct BattleWeapon {
    original: Rc<Weapon>,

    attack_power: AttackPower, // 攻撃性能
    sorcery_power: f32,        // 術力
    break_power: u32,          // ブレイク力
}

struct BattleConduct {
    actor_id: u32,
    target_id: u32,
    conduct: Conduct,
    weapon: Option<BattleWeapon>,
}

// 戦闘出来事
enum BattleIncident {
    Conduct(BattleIncidentConduct),
}
struct BattleIncidentConduct {
    attacker_id: u32,
    defender_id: u32,
    conduct: BattleConduct,
    // 成否
    outcome: BattleIncidentConductOutcome,
    // TODO: その他必要な情報
}
// 攻撃の成否
enum BattleIncidentConductOutcome {
    Success(BattleIncidentConductOutcomeSuccess), // 発動
    Failure(BattleIncidentConductOutcomeFailure), // 不発
}
// 行動成功
struct BattleIncidentConductOutcomeSuccess {
    // 行動者
    attacker: BattleIncidentConductOutcomeSuccessAttacker,
    // 被行動者
    defenders: Vec<BattleIncidentConductOutcomeSuccessDefender>,
}

struct BattleIncidentConductOutcomeSuccessAttacker {
    character_id: u32,
    stats_changes: Vec<BattleIncidentStats>,
}
struct BattleIncidentConductOutcomeSuccessDefender {
    character_id: u32,
    stats_changes: Vec<BattleIncidentStats>,
    status_effects: Vec<BattleIncidentStatusEffect>, // 状態変化
    is_evaded: bool,                                 // 回避したか
    // TODO: 回避した理由
    is_defended: bool, // 防御したか
}

enum BattleIncidentStats {
    DamageHp(BattleIncidentDamageHp),
    DamageSp(BattleIncidentDamageSp),
    DamageStamina(BattleIncidentDamageStamina),
    DamageBreak(BattleIncidentDamageBreak),
}

// HPダメージ
struct BattleIncidentDamageHp {
    damage: u32,
    before: u32,
    after: u32,
}
// SP減少
struct BattleIncidentDamageSp {
    damage: u32,
    before: u32,
    after: u32,
}
// スタミナ減少
struct BattleIncidentDamageStamina {
    damage: u32,
    before: u32,
    after: u32,
}
// ブレイク減少
struct BattleIncidentDamageBreak {
    damage: u32,
    before: u32,
    after: u32,
}

// 状態変化
struct BattleIncidentStatusEffect {
    // 状態変化内容
    status_effect: BattleStatusEffect,
    // 発生内容
    status_effect_handling: BattleIncidentStatusEffectHandling,
}
enum BattleIncidentStatusEffectHandling {
    Applied(BattleIncidentStatusEffectApplied), // 付与
    Removed(BattleIncidentStatusEffectRemoved), // 解除
}
//    status_effect: StatusEffect,
struct BattleIncidentStatusEffectApplied {
    // TODO: 付与理由
}
struct BattleIncidentStatusEffectRemoved {
    // TODO: 解除理由
}

// 行動失敗
struct BattleIncidentConductOutcomeFailure {
    reason: BattleIncidentConductOutcomeFailureReason,
}
struct BattleIncidentConductOutcomeFailureReason {
    // TODO: 詳細な理由分解
    insufficient_stamina: bool, // スタミナ不足
    insufficient_ability: bool, // 能力不足
    insufficient_sp: bool,      // SP不足
}

// 行動順序決定
fn decide_action_order(characters: Vec<&BattleCharacter>) -> Vec<u32> {
    let mut order: Vec<(u32, u32)> = characters
        .iter()
        .map(|c| (c.id, c.current_ability.agility))
        .collect();

    order.sort_by(|a, b| b.1.cmp(&a.1));

    order.into_iter().map(|(id, _)| id).collect()
}

// 行動成否判定
/// 不発の場合、理由を返す
/// 発動の場合、Noneを返す
fn determine_action_outcome_failure(
    conduct: &BattleConduct,
    attacker: &BattleCharacter,
) -> Option<BattleIncidentConductOutcomeFailureReason> {
    match &attacker.character_type {
        BattleCharacterType::Player(_) => {
            let current_status = &attacker.current_stats;
            // スタミナが足りないと不発
            if current_status.max_stamina - current_status.stamina_damage
                < conduct.conduct.stamina_cost
            {
                return Some(BattleIncidentConductOutcomeFailureReason {
                    insufficient_stamina: true,
                    insufficient_ability: false,
                    insufficient_sp: false,
                });
            }
        }
        BattleCharacterType::Enemy(_) => {
            // 敵のスタミナ管理は省略
        }
    };

    // 必要能力が足りないと不発
    let req = &conduct.conduct.requirement;
    let abil = &attacker.current_ability;
    if abil.strength < req.strength
        || abil.dexterity < req.dexterity
        || abil.intelligence < req.intelligence
        || abil.faith < req.faith
        || abil.arcane < req.arcane
        || abil.agility < req.agility
    {
        return Some(BattleIncidentConductOutcomeFailureReason {
            insufficient_stamina: false,
            insufficient_ability: true,
            insufficient_sp: false,
        });
    }

    // SPが足りないと不発
    let sp_cost = conduct.conduct.sp_cost;
    let current_sp = attacker.current_stats.max_sp - attacker.current_stats.sp_damage;
    if current_sp < sp_cost {
        return Some(BattleIncidentConductOutcomeFailureReason {
            insufficient_stamina: false,
            insufficient_ability: false,
            insufficient_sp: true,
        });
    }

    None
}

// 行動実行
fn execute_conduct(battle: &mut Battle, conduct: BattleConduct) -> BattleIncident {
    enum PlayerOrEnemy<'a> {
        Player(&'a BattlePlayer),
        Enemy(&'a BattleEnemy),
    }

    // 行動者の決定
    let attacker =
        if let Some(player) = battle.players.iter_mut().find(|p| p.id == conduct.actor_id) {
            player
        } else if let Some(enemy) = battle.enemies.iter_mut().find(|e| e.id == conduct.actor_id) {
            enemy
        } else {
            panic!("Attacker not found");
        };
    let attacker_id = attacker.id;

    // 行動成否判定
    if let Some(failure_reason) = determine_action_outcome_failure(&conduct, attacker) {
        // TODO: 不発理由に応じた処理
        return BattleIncident::Conduct(BattleIncidentConduct {
            attacker_id,
            defender_id: conduct.target_id,
            conduct,
            outcome: BattleIncidentConductOutcome::Failure(BattleIncidentConductOutcomeFailure {
                reason: failure_reason,
            }),
        });
    }

    //
    let mut attacker_stats_changes = Vec::new();

    let before_sp = attacker.current_stats.max_sp - attacker.current_stats.sp_damage;
    let sp_damage = conduct.conduct.sp_cost;
    // SP消費
    attacker.current_stats.sp_damage += sp_damage;
    let after_sp = attacker.current_stats.max_sp - attacker.current_stats.sp_damage;
    // インシデント
    attacker_stats_changes.push(BattleIncidentStats::DamageSp(BattleIncidentDamageSp {
        damage: sp_damage,
        before: before_sp,
        after: after_sp,
    }));

    // スタミナ消費
    if let BattleCharacterType::Player(_) = attacker.character_type {
        // プレイヤーの場合のみスタミナ消費処理
        let before_stamina =
            attacker.current_stats.max_stamina - attacker.current_stats.stamina_damage;
        let stamina_damage = conduct.conduct.stamina_cost;
        attacker.current_stats.stamina_damage += conduct.conduct.stamina_cost;
        let after_stamina =
            attacker.current_stats.max_stamina - attacker.current_stats.stamina_damage;
        // インシデント
        attacker_stats_changes.push(BattleIncidentStats::DamageStamina(
            BattleIncidentDamageStamina {
                damage: stamina_damage,
                before: before_stamina,
                after: after_stamina,
            },
        ));
    }

    // 行動者インシデント
    let attacker_incident = BattleIncidentConductOutcomeSuccessAttacker {
        character_id: attacker_id,
        stats_changes: attacker_stats_changes,
    };

    // ターゲットの決定
    let target = if let Some(player) = battle
        .players
        .iter_mut()
        .find(|p| p.id == conduct.target_id)
    {
        player
    } else if let Some(enemy) = battle
        .enemies
        .iter_mut()
        .find(|e| e.id == conduct.target_id)
    {
        enemy
    } else {
        panic!("Defender not found");
    };
    // TODO: 複数ターゲットが存在した時のターゲットごとに効果処理
    let defender_incident = conduct_effect(&conduct, target);

    BattleIncident::Conduct(BattleIncidentConduct {
        attacker_id,
        defender_id: target.id,
        conduct,
        outcome: BattleIncidentConductOutcome::Success(BattleIncidentConductOutcomeSuccess {
            attacker: attacker_incident,
            defenders: vec![defender_incident],
        }),
    })
}

// 行動攻撃補正
fn calc_conduct_attack_modifier(
    base_attack_power: u32,
    conduct_attack_power: u32,
    conduct_attack_power_scaling: f32,
) -> u32 {
    conduct_attack_power + (base_attack_power as f32 * conduct_attack_power_scaling) as u32
}

// 攻撃力補正
fn calc_attack_power_modifier(
    base_attack_power: &AttackPower,
    modify_attack_power: &AttackPower,
    modify_attack_power_scaling: &AttackPowerScaling,
) -> AttackPower {
    AttackPower {
        slash: modify_attack_power.slash
            + (base_attack_power.slash as f32 * modify_attack_power_scaling.slash) as u32,
        strike: modify_attack_power.strike
            + (base_attack_power.strike as f32 * modify_attack_power_scaling.strike) as u32,
        thrust: modify_attack_power.thrust
            + (base_attack_power.thrust as f32 * modify_attack_power_scaling.thrust) as u32,
        impact: modify_attack_power.impact
            + (base_attack_power.impact as f32 * modify_attack_power_scaling.impact) as u32,
        magic: modify_attack_power.magic
            + (base_attack_power.magic as f32 * modify_attack_power_scaling.magic) as u32,
        fire: modify_attack_power.fire
            + (base_attack_power.fire as f32 * modify_attack_power_scaling.fire) as u32,
        lightning: modify_attack_power.lightning
            + (base_attack_power.lightning as f32 * modify_attack_power_scaling.lightning) as u32,
        chaos: modify_attack_power.chaos
            + (base_attack_power.chaos as f32 * modify_attack_power_scaling.chaos) as u32,
    }
}

fn calc_attack_power_cut_rate(
    attack_power: &AttackPower,
    guard_cut_rate: &GuardCutRate,
) -> AttackPower {
    AttackPower {
        slash: (attack_power.slash as f32 * guard_cut_rate.slash) as u32,
        strike: (attack_power.strike as f32 * guard_cut_rate.strike) as u32,
        thrust: (attack_power.thrust as f32 * guard_cut_rate.thrust) as u32,
        impact: (attack_power.impact as f32 * guard_cut_rate.impact) as u32,
        magic: (attack_power.magic as f32 * guard_cut_rate.magic) as u32,
        fire: (attack_power.fire as f32 * guard_cut_rate.fire) as u32,
        lightning: (attack_power.lightning as f32 * guard_cut_rate.lightning) as u32,
        chaos: (attack_power.chaos as f32 * guard_cut_rate.chaos) as u32,
    }
}

// ダメージ計算
fn calc_damage(attack_power: &AttackPower, defender: &DefensePower) -> u32 {
    let damage = (attack_power.slash / defender.slash)
        + (attack_power.strike / defender.strike)
        + (attack_power.thrust / defender.thrust)
        + (attack_power.impact / defender.impact)
        + (attack_power.magic / defender.magic)
        + (attack_power.fire / defender.fire)
        + (attack_power.lightning / defender.lightning)
        + (attack_power.chaos / defender.chaos);
    damage
}

fn support_status_effect(
    status_effects: &Vec<StatusEffect>,
    target: &mut BattleCharacter,
) -> Vec<BattleIncidentStatusEffect> {
    // 支援行動処理
    let mut status_effect_incidents: Vec<BattleIncidentStatusEffect> = Vec::new();
    for status_effect in status_effects {
        // 状態変化付与処理
        let battle_status_effect = create_battle_status_effect(status_effect);
        // 状態変化付与
        // TODO: 状態変化の重複処理
        target.status_effects.push(battle_status_effect.clone());
        status_effect_incidents.push(BattleIncidentStatusEffect {
            status_effect: battle_status_effect,
            status_effect_handling: BattleIncidentStatusEffectHandling::Applied(
                BattleIncidentStatusEffectApplied {},
            ),
        });
    }
    status_effect_incidents
}

fn conduct_effect(
    conduct: &BattleConduct,
    target: &mut BattleCharacter,
) -> BattleIncidentConductOutcomeSuccessDefender {
    // 回避判定
    for se in target.status_effects.iter() {
        match &se.potency {
            StatusEffectPotency::Evasion => {
                // 回避効果処理
                return BattleIncidentConductOutcomeSuccessDefender {
                    character_id: target.id,
                    stats_changes: Vec::new(),
                    status_effects: Vec::new(),
                    is_defended: false,
                    is_evaded: true,
                };
            }
            StatusEffectPotency::Airborne => {
                // 空中効果処理
                // 遠距離攻撃でない時は回避
                if !conduct.conduct.perks.contains(&ConductPerk::Ranged) {
                    return BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.id,
                        stats_changes: Vec::new(),
                        status_effects: Vec::new(),
                        is_defended: false,
                        is_evaded: true,
                    };
                }
            }
            StatusEffectPotency::Floating => {
                // 浮遊効果処理
                // 足元攻撃は回避
                if conduct.conduct.perks.contains(&ConductPerk::AtFeet) {
                    return BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.id,
                        stats_changes: Vec::new(),
                        status_effects: Vec::new(),
                        is_defended: false,
                        is_evaded: true,
                    };
                }
            }
            StatusEffectPotency::Ranged => {
                // 遠距離効果処理
                // 近距離の攻撃を回避
                if !conduct.conduct.perks.contains(&ConductPerk::Ranged) {
                    return BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.id,
                        stats_changes: Vec::new(),
                        status_effects: Vec::new(),
                        is_defended: false,
                        is_evaded: true,
                    };
                }
            }
            _ => {
                // その他
            }
        }
    }

    match &conduct.conduct.conduct_type {
        ConductType::Basic(basic) => {
            match basic {
                ConductTypeBasic::Attack(conduct_attack) => {
                    let mut stats_change_incidents = Vec::new();
                    let mut status_effect_incidents = Vec::new();

                    // ダメージ計算
                    let mut attak_power = conduct_attack.attack_power.clone();
                    let mut is_defended = false;
                    for se in target.status_effects.iter() {
                        match &se.potency {
                            StatusEffectPotency::Resistance(resistance) => {
                                // 防御効果処理
                                attak_power =
                                    calc_attack_power_cut_rate(&attak_power, &resistance.cut_rate);
                                is_defended = true;
                            }
                            _ => {
                                // その他
                            }
                        }
                    }

                    let defense_power = &target.defense_power;
                    let damage = calc_damage(&attak_power, defense_power);

                    let current_hp_damage = target.current_stats.hp_damage;
                    let mut next_hp_damage = current_hp_damage + damage;
                    if next_hp_damage > target.current_stats.max_hp {
                        next_hp_damage = target.current_stats.max_hp;
                    }
                    target.current_stats.hp_damage = next_hp_damage;
                    // HPダメージのインシデント
                    stats_change_incidents.push(BattleIncidentStats::DamageHp(
                        BattleIncidentDamageHp {
                            damage,
                            before: current_hp_damage,
                            after: next_hp_damage,
                        },
                    ));

                    // ブレイクダメージ処理
                    if let BattleCharacterType::Enemy(enemy) = &target.character_type {
                        // ブレイク中でない時
                        let mut is_break = false;
                        for se in target.status_effects.iter() {
                            if let StatusEffectPotency::Break(_) = &se.potency {
                                is_break = true
                            }
                        }

                        if !is_break {
                            // 敵のブレイクダメージ処理
                            let break_power = conduct_attack.break_power;
                            let current_break_damage = enemy.current_enemy_only_stats.break_damage;
                            let mut next_break_damage = current_break_damage + break_power;
                            if next_break_damage > enemy.current_enemy_only_stats.max_break {
                                next_break_damage = enemy.current_enemy_only_stats.max_break;

                                // ブレイク状態にする
                                // TODO: サポート技用の関数を使用していいか？
                                let new_status_effects = support_status_effect(
                                    &vec![StatusEffect {
                                        potency: StatusEffectPotency::Break(StatusEffectBreak {}),
                                        duration: StatusEffectDuration::Permanent,
                                    }],
                                    target,
                                );
                                status_effect_incidents.extend(new_status_effects);
                            }

                            // ブレイクダメージインシデント追加
                            stats_change_incidents.push(BattleIncidentStats::DamageBreak(
                                BattleIncidentDamageBreak {
                                    damage: break_power,
                                    before: current_break_damage,
                                    after: next_break_damage,
                                },
                            ));
                        }
                    }

                    BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.id,
                        stats_changes: stats_change_incidents,
                        status_effects: status_effect_incidents,
                        is_defended,
                        is_evaded: false,
                    }
                }
                ConductTypeBasic::Support(support) => {
                    // 支援行動処理
                    match &support {
                        ConductTypeBasicSupport::StatusEffect(status_effect) => {
                            let new_incidents =
                                support_status_effect(&status_effect.status_effects, target);

                            BattleIncidentConductOutcomeSuccessDefender {
                                character_id: target.id,
                                stats_changes: Vec::new(),
                                status_effects: new_incidents,
                                is_defended: false,
                                is_evaded: false,
                            }
                        }
                    }
                }
            }
        }
        ConductType::Skill(skill) => match &skill.potency {
            ConductTypeSkillPotency::Attack(skill) => {
                let mut stats_change_incidents = Vec::new();
                let mut status_effect_incidents = Vec::new();

                let weapon_attack_power = if let Some(weapon) = &conduct.weapon {
                    &weapon.attack_power
                } else {
                    &AttackPower::default()
                };
                let skill_attack_power = &skill.attack_power;
                let skill_attack_power_scaling = &skill.attack_power_scaling;
                let mut attack_power = calc_attack_power_modifier(
                    skill_attack_power,
                    weapon_attack_power,
                    skill_attack_power_scaling,
                );

                // 防御効果処理
                let mut is_defended = false;
                for se in target.status_effects.iter() {
                    match &se.potency {
                        StatusEffectPotency::Resistance(resistance) => {
                            // 防御効果処理
                            attack_power =
                                calc_attack_power_cut_rate(&attack_power, &resistance.cut_rate);
                            is_defended = true;
                        }
                        _ => {
                            // その他
                        }
                    }
                }

                let break_power =
                    calc_conduct_attack_modifier(0, skill.break_power, skill.break_power_scaling);

                // ダメージ
                let damage = calc_damage(&attack_power, &target.defense_power);
                let current_hp_damage = target.current_stats.hp_damage;
                let mut next_hp_damage = current_hp_damage + damage;
                if next_hp_damage > target.current_stats.max_hp {
                    next_hp_damage = target.current_stats.max_hp;
                }
                target.current_stats.hp_damage = next_hp_damage;
                // HPダメージのインシデント
                stats_change_incidents.push(BattleIncidentStats::DamageHp(
                    BattleIncidentDamageHp {
                        damage,
                        before: current_hp_damage,
                        after: next_hp_damage,
                    },
                ));

                // ブレイクダメージ処理
                if let BattleCharacterType::Enemy(enemy) = &target.character_type {
                    // ブレイク中でない時
                    let mut is_break = false;
                    for se in target.status_effects.iter() {
                        if let StatusEffectPotency::Break(_) = &se.potency {
                            is_break = true
                        }
                    }
                    if !is_break {
                        // 敵のブレイクダメージ処理
                        let current_break_damage = enemy.current_enemy_only_stats.break_damage;
                        let mut next_break_damage = current_break_damage + break_power;
                        if next_break_damage > enemy.current_enemy_only_stats.max_break {
                            next_break_damage = enemy.current_enemy_only_stats.max_break;

                            // ブレイク状態にする
                            let new_status_effects = support_status_effect(
                                &vec![StatusEffect {
                                    potency: StatusEffectPotency::Break(StatusEffectBreak {}),
                                    duration: StatusEffectDuration::Permanent,
                                }],
                                target,
                            );
                            status_effect_incidents.extend(new_status_effects);
                        }
                        // ブレイクダメージインシデント追加
                        stats_change_incidents.push(BattleIncidentStats::DamageBreak(
                            BattleIncidentDamageBreak {
                                damage: break_power,
                                before: current_break_damage,
                                after: next_break_damage,
                            },
                        ));
                    }
                }

                BattleIncidentConductOutcomeSuccessDefender {
                    character_id: target.id,
                    stats_changes: stats_change_incidents,
                    status_effects: Vec::new(),
                    is_defended,
                    is_evaded: false,
                }
            }
            ConductTypeSkillPotency::Support(support) => {
                // 支援行動処理
                match &support {
                    ConductTypeSkillPotencySupport::StatusEffect(status_effect) => {
                        let new_incidents =
                            support_status_effect(&status_effect.status_effects, target);

                        BattleIncidentConductOutcomeSuccessDefender {
                            character_id: target.id,
                            stats_changes: Vec::new(),
                            status_effects: new_incidents,
                            is_defended: false,
                            is_evaded: false,
                        }
                    }
                }
            }
        },
        ConductType::Sorcery(sorcery) => match &sorcery {
            ConductTypeSorcery::Attack(sorcery) => {
                let mut stats_change_incidents = Vec::new();
                let mut status_effect_incidents = Vec::new();

                let mut attack_power = sorcery.attack_power.clone();
                let sorcery_power = if let Some(weapon) = &conduct.weapon {
                    weapon.sorcery_power
                } else {
                    1.0
                };
                attack_power.multiply(sorcery_power);

                // 防御効果処理
                let mut is_defended = false;
                for se in target.status_effects.iter() {
                    match &se.potency {
                        StatusEffectPotency::Resistance(resistance) => {
                            // 防御効果処理
                            attack_power =
                                calc_attack_power_cut_rate(&attack_power, &resistance.cut_rate);
                            is_defended = true;
                        }
                        _ => {
                            // その他
                        }
                    }
                }

                // ダメージ計算
                let damage = calc_damage(&attack_power, &target.defense_power);

                // TODO: ブレイク状態のダメージ補正
                // TODO: 防御側が敵の場合、ブレイクダメージ処理

                let current_hp_damage = target.current_stats.hp_damage;
                let mut next_hp_damage = current_hp_damage + damage;
                if next_hp_damage > target.current_stats.max_hp {
                    next_hp_damage = target.current_stats.max_hp;
                }
                target.current_stats.hp_damage = next_hp_damage;
                // HPダメージのインシデント
                stats_change_incidents.push(BattleIncidentStats::DamageHp(
                    BattleIncidentDamageHp {
                        damage,
                        before: current_hp_damage,
                        after: next_hp_damage,
                    },
                ));

                // ブレイクダメージ処理
                if let BattleCharacterType::Enemy(enemy) = &target.character_type {
                    // ブレイク中でない時
                    let mut is_break = false;
                    for se in target.status_effects.iter() {
                        if let StatusEffectPotency::Break(_) = &se.potency {
                            is_break = true
                        }
                    }
                    if !is_break {
                        // 敵のブレイクダメージ処理
                        let break_power = sorcery.break_power;
                        let current_break_damage = enemy.current_enemy_only_stats.break_damage;
                        let mut next_break_damage = current_break_damage + break_power;
                        if next_break_damage > enemy.current_enemy_only_stats.max_break {
                            next_break_damage = enemy.current_enemy_only_stats.max_break;
                            // ブレイク状態にする
                            let new_status_effects = support_status_effect(
                                &vec![StatusEffect {
                                    potency: StatusEffectPotency::Break(StatusEffectBreak {}),
                                    duration: StatusEffectDuration::Permanent,
                                }],
                                target,
                            );
                            status_effect_incidents.extend(new_status_effects);
                        }
                        // ブレイクダメージインシデント追加
                        stats_change_incidents.push(BattleIncidentStats::DamageBreak(
                            BattleIncidentDamageBreak {
                                damage: break_power,
                                before: current_break_damage,
                                after: next_break_damage,
                            },
                        ));
                    }
                }

                BattleIncidentConductOutcomeSuccessDefender {
                    character_id: target.id,
                    stats_changes: stats_change_incidents,
                    status_effects: status_effect_incidents,
                    is_defended,
                    is_evaded: false,
                }
            }
            ConductTypeSorcery::Support(support) => match &support {
                ConductTypeSorcerySupport::StatusEffect(status_effect) => {
                    let new_incidents =
                        support_status_effect(&status_effect.status_effects, target);

                    BattleIncidentConductOutcomeSuccessDefender {
                        character_id: target.id,
                        stats_changes: Vec::new(),
                        status_effects: new_incidents,
                        is_defended: false,
                        is_evaded: false,
                    }
                }
            },
        },
    }
}

fn battle() {
    println!("Battle started!");
}

#[cfg(test)]
mod tests {
    use super::*;

    // ヘルパー: ダミーのプレイヤー原本
    fn dummy_player() -> Rc<Player> {
        Rc::new(Player {
            ability: PlayerAbility {
                vitality: 0,
                spirit: 0,
                endurance: 0,
                agility: 0,
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
            },
            stats: PlayerStats {
                hp: 100,
                sp: 10,
                stamina: 10,
                equip_load: 0,
            },
            equipment: Equipment {
                weapon1: None,
                weapon2: None,
                armor1: None,
                armor2: None,
                armor3: None,
                armor4: None,
                armor5: None,
                armor6: None,
                armor7: None,
                armor8: None,
            },
        })
    }

    // ヘルパー: 最低限の防御力(0除算を避けるため全て1)
    fn min_defense() -> DefensePower {
        DefensePower {
            slash: 1,
            strike: 1,
            thrust: 1,
            impact: 1,
            magic: 1,
            fire: 1,
            lightning: 1,
            chaos: 1,
        }
    }

    // ヘルパー: デフォルト攻撃力(全て0)
    fn zero_attack() -> AttackPower {
        AttackPower::default()
    }

    // conduct_effect: 回避(Evasion)で早期リターンすること
    #[test]
    fn test_conduct_effect_evades_with_evasion() {
        // ターゲット: プレイヤーで回避状態
        let mut target = BattleCharacter {
            id: 1,
            current_ability: BattleAbility {
                agility: 0,
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
            },
            current_stats: BattleStats {
                max_hp: 100,
                max_sp: 10,
                max_stamina: 10,
                hp_damage: 0,
                sp_damage: 0,
                stamina_damage: 0,
            },
            defense_power: min_defense(),
            status_effects: vec![BattleStatusEffect {
                potency: StatusEffectPotency::Evasion,
                duration: BattleStatusEffectDuration::Permanent,
            }],
            character_type: BattleCharacterType::Player(BattlePlayer {
                original: dummy_player(),
            }),
        };

        // 行動: 基本攻撃(近接)で十分
        let conduct = BattleConduct {
            actor_id: 100,
            target_id: 1,
            conduct: Conduct {
                name: "Basic Attack".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Basic(ConductTypeBasic::Attack(
                    ConductTypeBasicAttack {
                        attack_power: zero_attack(),
                        break_power: 0,
                    },
                )),
            },
            weapon: None,
        };

        let result = conduct_effect(&conduct, &mut target);

        assert!(result.is_evaded);
        assert!(!result.is_defended);
        assert!(result.stats_changes.is_empty());
        assert!(result.status_effects.is_empty());
    }

    // conduct_effect: 非回避ルート（基本攻撃・攻撃力0）は回避せず、HPダメージ0の適用結果を返す
    #[test]
    fn test_conduct_effect_basic_attack_zero_damage() {
        // ターゲット: プレイヤー(状態変化なし)
        let mut target = BattleCharacter {
            id: 2,
            current_ability: BattleAbility {
                agility: 0,
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
            },
            current_stats: BattleStats {
                max_hp: 100,
                max_sp: 10,
                max_stamina: 10,
                hp_damage: 0,
                sp_damage: 0,
                stamina_damage: 0,
            },
            defense_power: min_defense(),
            status_effects: vec![],
            character_type: BattleCharacterType::Player(BattlePlayer {
                original: dummy_player(),
            }),
        };

        // 行動: 基本攻撃(近接)
        let conduct = BattleConduct {
            actor_id: 100,
            target_id: 2,
            conduct: Conduct {
                name: "Basic Attack".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Basic(ConductTypeBasic::Attack(
                    ConductTypeBasicAttack {
                        attack_power: zero_attack(),
                        break_power: 0,
                    },
                )),
            },
            weapon: None,
        };

        let result = conduct_effect(&conduct, &mut target);

        // 回避・防御なし
        assert!(!result.is_evaded);
        assert!(!result.is_defended);
        // 状態変化なし
        assert!(result.status_effects.is_empty());
        // HPダメージ0が記録されていること
        assert_eq!(result.stats_changes.len(), 1);
        match &result.stats_changes[0] {
            BattleIncidentStats::DamageHp(d) => {
                assert_eq!(d.damage, 0);
                assert_eq!(d.before, 0);
                assert_eq!(d.after, 0);
            }
            _ => panic!("expected DamageHp incident"),
        }
        // 実際のターゲットのHPダメージも0のまま
        assert_eq!(target.current_stats.hp_damage, 0);
    }

    // 基本攻撃でHPダメージが適用されること
    #[test]
    fn test_conduct_effect_basic_attack_applies_damage() {
        let mut target = BattleCharacter {
            id: 10,
            current_ability: BattleAbility {
                agility: 0,
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
            },
            current_stats: BattleStats {
                max_hp: 100,
                max_sp: 10,
                max_stamina: 10,
                hp_damage: 0,
                sp_damage: 0,
                stamina_damage: 0,
            },
            defense_power: min_defense(),
            status_effects: vec![],
            character_type: BattleCharacterType::Player(BattlePlayer {
                original: dummy_player(),
            }),
        };

        let mut atk = zero_attack();
        atk.slash = 10; // 期待ダメージ10

        let conduct = BattleConduct {
            actor_id: 100,
            target_id: 10,
            conduct: Conduct {
                name: "Basic Attack".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Basic(ConductTypeBasic::Attack(
                    ConductTypeBasicAttack { attack_power: atk, break_power: 0 },
                )),
            },
            weapon: None,
        };

        let result = conduct_effect(&conduct, &mut target);

        assert!(!result.is_evaded);
        assert!(!result.is_defended);
        assert!(matches!(result.stats_changes.get(0), Some(BattleIncidentStats::DamageHp(_))));
        if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
            assert_eq!(d.damage, 10);
            assert_eq!(d.before, 0);
            assert_eq!(d.after, 10);
        } else {
            panic!("expected DamageHp incident");
        }
        assert_eq!(target.current_stats.hp_damage, 10);
    }

    // 技攻撃でHPダメージが適用されること
    #[test]
    fn test_conduct_effect_skill_attack_applies_damage() {
        let mut target = BattleCharacter {
            id: 11,
            current_ability: BattleAbility {
                agility: 0,
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
            },
            current_stats: BattleStats {
                max_hp: 100,
                max_sp: 10,
                max_stamina: 10,
                hp_damage: 0,
                sp_damage: 0,
                stamina_damage: 0,
            },
            defense_power: min_defense(),
            status_effects: vec![],
            character_type: BattleCharacterType::Player(BattlePlayer {
                original: dummy_player(),
            }),
        };

        let mut skill_ap = zero_attack();
        skill_ap.slash = 12; // スキル基礎攻撃力

        // 実装ロジック: 攻撃力 = 武器攻撃 + (スキル攻撃 * スケーリング)
        // 武器を持たせ、スキル側のスケーリングも1.0にして合算を検証
        let mut scaling = AttackPowerScaling::default();
        scaling.slash = 1.0;

        // ダミー武器（攻撃力 5 を付与）
        let weapon = BattleWeapon {
            original: Rc::new(Weapon {
                kind: WeaponKind::StraightSword,
                weight: 1,
                ability_requirement: WeaponAbilityRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                attack_power: WeaponAttackPower {
                    base: AttackPower::default(),
                    ability_scaling: WeaponAttackPowerAbilityScaling {
                        slash: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        strike: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        thrust: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        impact: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        magic: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        fire: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        lightning: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        chaos: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                    },
                },
                sorcery_power: WeaponSorceryPower { base: 1, scaling: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 } },
                break_power: WeaponBreakPower { base_power: 0, scaling: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 } },
                guard: WeaponGuard { cut_rate: GuardCutRate { slash: 1.0, strike: 1.0, thrust: 1.0, impact: 1.0, magic: 1.0, fire: 1.0, lightning: 1.0, chaos: 1.0 }, guard_strength: 0 },
            }),
            attack_power: AttackPower { slash: 5, strike: 0, thrust: 0, impact: 0, magic: 0, fire: 0, lightning: 0, chaos: 0 },
            sorcery_power: 1.0,
            break_power: 0,
        };

        let conduct = BattleConduct {
            actor_id: 100,
            target_id: 11,
            conduct: Conduct {
                name: "Skill Attack".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Skill(ConductTypeSkill {
                    usable_weapon_kinds: vec![],
                    potency: ConductTypeSkillPotency::Attack(ConductTypeSkillPotencyAttack {
                        attack_power: skill_ap,
                        attack_power_scaling: scaling,
                        break_power: 0,
                        break_power_scaling: 0.0,
                    }),
                }),
            },
            weapon: Some(weapon),
        };

        let result = conduct_effect(&conduct, &mut target);

        assert!(!result.is_evaded);
        assert!(!result.is_defended);
        if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
            // 期待値: weapon(5) + skill(12*1.0) = 17
            assert_eq!(d.damage, 17);
            assert_eq!(d.before, 0);
            assert_eq!(d.after, 17);
        } else {
            panic!("expected DamageHp incident");
        }
        assert_eq!(target.current_stats.hp_damage, 17);
    }

    // 術攻撃でHPダメージが適用されること
    #[test]
    fn test_conduct_effect_sorcery_attack_applies_damage() {
        let mut target = BattleCharacter {
            id: 12,
            current_ability: BattleAbility {
                agility: 0,
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
            },
            current_stats: BattleStats {
                max_hp: 100,
                max_sp: 10,
                max_stamina: 10,
                hp_damage: 0,
                sp_damage: 0,
                stamina_damage: 0,
            },
            defense_power: min_defense(),
            status_effects: vec![],
            character_type: BattleCharacterType::Player(BattlePlayer {
                original: dummy_player(),
            }),
        };

        let mut sorc_ap = zero_attack();
        sorc_ap.slash = 8; // 期待ダメージ8（weaponなし→術力1.0）

        let conduct = BattleConduct {
            actor_id: 100,
            target_id: 12,
            conduct: Conduct {
                name: "Sorcery Attack".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Sorcery(ConductTypeSorcery::Attack(
                    ConductTypeSorceryAttack { attack_power: sorc_ap, break_power: 0 },
                )),
            },
            weapon: None, // weaponなし→術力1.0
        };

        let result = conduct_effect(&conduct, &mut target);

        assert!(!result.is_evaded);
        assert!(!result.is_defended);
        if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
            assert_eq!(d.damage, 8);
            assert_eq!(d.before, 0);
            assert_eq!(d.after, 8);
        } else {
            panic!("expected DamageHp incident");
        }
        assert_eq!(target.current_stats.hp_damage, 8);
    }

    // 技: スケーリング0.0なら武器攻撃力のみが寄与すること
    #[test]
    fn test_conduct_effect_skill_attack_with_weapon_zero_scaling() {
        let mut target = BattleCharacter {
            id: 13,
            current_ability: BattleAbility {
                agility: 0,
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
            },
            current_stats: BattleStats {
                max_hp: 100,
                max_sp: 10,
                max_stamina: 10,
                hp_damage: 0,
                sp_damage: 0,
                stamina_damage: 0,
            },
            defense_power: min_defense(),
            status_effects: vec![],
            character_type: BattleCharacterType::Player(BattlePlayer {
                original: dummy_player(),
            }),
        };

        // skill基礎攻撃力（高めに設定するが、スケーリング0.0なので無視される）
        let mut skill_ap = zero_attack();
        skill_ap.slash = 20;

        let scaling = AttackPowerScaling::default(); // 全属性0.0

        // ダミー武器（攻撃力 7 を付与）
        let weapon = BattleWeapon {
            original: Rc::new(Weapon {
                kind: WeaponKind::StraightSword,
                weight: 1,
                ability_requirement: WeaponAbilityRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                attack_power: WeaponAttackPower {
                    base: AttackPower::default(),
                    ability_scaling: WeaponAttackPowerAbilityScaling {
                        slash: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        strike: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        thrust: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        impact: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        magic: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        fire: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        lightning: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        chaos: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                    },
                },
                sorcery_power: WeaponSorceryPower { base: 1, scaling: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 } },
                break_power: WeaponBreakPower { base_power: 0, scaling: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 } },
                guard: WeaponGuard { cut_rate: GuardCutRate { slash: 1.0, strike: 1.0, thrust: 1.0, impact: 1.0, magic: 1.0, fire: 1.0, lightning: 1.0, chaos: 1.0 }, guard_strength: 0 },
            }),
            attack_power: AttackPower { slash: 7, strike: 0, thrust: 0, impact: 0, magic: 0, fire: 0, lightning: 0, chaos: 0 },
            sorcery_power: 1.0,
            break_power: 0,
        };

        let conduct = BattleConduct {
            actor_id: 100,
            target_id: 13,
            conduct: Conduct {
                name: "Skill Attack Zero Scaling".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Skill(ConductTypeSkill {
                    usable_weapon_kinds: vec![],
                    potency: ConductTypeSkillPotency::Attack(ConductTypeSkillPotencyAttack {
                        attack_power: skill_ap,
                        attack_power_scaling: scaling, // 0.0
                        break_power: 0,
                        break_power_scaling: 0.0,
                    }),
                }),
            },
            weapon: Some(weapon),
        };

        let result = conduct_effect(&conduct, &mut target);
        if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
            assert_eq!(d.damage, 7); // 武器のみ寄与
            assert_eq!(d.before, 0);
            assert_eq!(d.after, 7);
        } else {
            panic!("expected DamageHp incident");
        }
        assert_eq!(target.current_stats.hp_damage, 7);
    }

    // 技: 複数属性の合算が正しく行われること
    #[test]
    fn test_conduct_effect_skill_attack_multi_attribute_sum() {
        let mut target = BattleCharacter {
            id: 14,
            current_ability: BattleAbility {
                agility: 0,
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                faith: 0,
                arcane: 0,
            },
            current_stats: BattleStats {
                max_hp: 100,
                max_sp: 10,
                max_stamina: 10,
                hp_damage: 0,
                sp_damage: 0,
                stamina_damage: 0,
            },
            defense_power: min_defense(),
            status_effects: vec![],
            character_type: BattleCharacterType::Player(BattlePlayer {
                original: dummy_player(),
            }),
        };

        // skill: slash=10, strike=6
        let mut skill_ap = zero_attack();
        skill_ap.slash = 10;
        skill_ap.strike = 6;

        let mut scaling = AttackPowerScaling::default();
        scaling.slash = 1.0;
        scaling.strike = 1.0;

        // weapon: slash=5, strike=4
        let weapon = BattleWeapon {
            original: Rc::new(Weapon {
                kind: WeaponKind::StraightSword,
                weight: 1,
                ability_requirement: WeaponAbilityRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                attack_power: WeaponAttackPower {
                    base: AttackPower::default(),
                    ability_scaling: WeaponAttackPowerAbilityScaling {
                        slash: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        strike: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        thrust: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        impact: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        magic: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        fire: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        lightning: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                        chaos: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 },
                    },
                },
                sorcery_power: WeaponSorceryPower { base: 1, scaling: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 } },
                break_power: WeaponBreakPower { base_power: 0, scaling: AbilityScaling { strength: 0.0, dexterity: 0.0, intelligence: 0.0, faith: 0.0, arcane: 0.0, agility: 0.0 } },
                guard: WeaponGuard { cut_rate: GuardCutRate { slash: 1.0, strike: 1.0, thrust: 1.0, impact: 1.0, magic: 1.0, fire: 1.0, lightning: 1.0, chaos: 1.0 }, guard_strength: 0 },
            }),
            attack_power: AttackPower { slash: 5, strike: 4, thrust: 0, impact: 0, magic: 0, fire: 0, lightning: 0, chaos: 0 },
            sorcery_power: 1.0,
            break_power: 0,
        };

        let conduct = BattleConduct {
            actor_id: 100,
            target_id: 14,
            conduct: Conduct {
                name: "Skill Attack Multi Attribute".to_string(),
                sp_cost: 0,
                stamina_cost: 0,
                perks: vec![ConductPerk::Melee],
                requirement: ConductRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                conduct_type: ConductType::Skill(ConductTypeSkill {
                    usable_weapon_kinds: vec![],
                    potency: ConductTypeSkillPotency::Attack(ConductTypeSkillPotencyAttack {
                        attack_power: skill_ap,
                        attack_power_scaling: scaling, // 1.0 on slash & strike
                        break_power: 0,
                        break_power_scaling: 0.0,
                    }),
                }),
            },
            weapon: Some(weapon),
        };

        let result = conduct_effect(&conduct, &mut target);
        if let BattleIncidentStats::DamageHp(d) = &result.stats_changes[0] {
            // 期待値: (slash 5 + 10) + (strike 4 + 6) = 25
            assert_eq!(d.damage, 25);
            assert_eq!(d.before, 0);
            assert_eq!(d.after, 25);
        } else {
            panic!("expected DamageHp incident");
        }
        assert_eq!(target.current_stats.hp_damage, 25);
    }
}
