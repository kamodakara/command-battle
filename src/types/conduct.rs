use super::common::*;
use super::equipment::WeaponKind;
use super::status_ailment::StatusEffect;

// 戦闘行動
pub struct Conduct {
    pub name: String,                    // 名前
    pub sp_cost: u32,                    // SP消費
    pub stamina_cost: u32,               // スタミナ消費
    pub perks: Vec<ConductPerk>,         // 特性
    pub requirement: ConductRequirement, // 必要能力
    // pub target: ConductTarget,           // 攻撃対象
    pub conduct_type: ConductType, // 戦闘行動内容
}

// 戦闘行動特性
#[derive(PartialEq, Clone)]
pub enum ConductPerk {
    // 近距離
    Melee,
    // 遠距離
    Ranged,
    // 足元
    AtFeet,
}

pub enum ConductTarget {
    Single,     // 単体
    All,        // 全体
    SelfTarget, // 自身
}

// 戦闘行動必要能力
pub struct ConductRequirement {
    pub strength: u32,     // 筋力
    pub dexterity: u32,    // 技量
    pub intelligence: u32, // 知力
    pub faith: u32,        // 信仰
    pub arcane: u32,       // 神秘
    pub agility: u32,      // 敏捷性
}
// 戦闘行動内容
#[derive(Clone)]
pub enum ConductType {
    Basic(ConductTypeBasic),     // 基本
    Skill(ConductTypeSkill),     // 技
    Sorcery(ConductTypeSorcery), // 術
}
// 戦闘行動基本
#[derive(Clone)]
pub enum ConductTypeBasic {
    Attack(ConductTypeBasicAttack),   // 攻撃
    Support(ConductTypeBasicSupport), // 支援
}
#[derive(Clone)]
pub struct ConductTypeBasicAttack {
    pub attack_power: AttackPower, // 攻撃力
    pub break_power: u32,          // ブレイク攻撃力
}
#[derive(Clone)]
pub enum ConductTypeBasicSupport {
    StatusEffect(SuportStatusEffect), // 状態変化付与
    Recover(SupportRecover),          // HP回復量
}

#[derive(Clone)]
pub struct SuportStatusEffect {
    pub status_effects: Vec<StatusEffect>,
}
#[derive(Clone)]
pub struct SupportRecover {
    pub potencies: Vec<SupportRecoverPotency>, // 回復効果
}
#[derive(Clone)]
pub enum SupportRecoverPotency {
    Hp(SupportRecoverPotencyHp),           // HP回復量
    Sp(SupportRecoverPotencySp),           // SP回復量
    Stamina(SupportRecoverPotencyStamina), // スタミナ回復量
}
#[derive(Clone)]
pub struct SupportRecoverPotencyHp {
    pub hp_recover: u32, // HP回復量
}
#[derive(Clone)]
pub struct SupportRecoverPotencySp {
    pub sp_recover: u32, // SP回復量
}
#[derive(Clone)]
pub struct SupportRecoverPotencyStamina {
    pub stamina_recover: u32, // スタミナ回復量
}

// 戦闘行動技
#[derive(Clone)]
pub struct ConductTypeSkill {
    // 使用可能武器
    pub usable_weapon_kinds: Vec<WeaponKind>,
    pub potency: ConductTypeSkillPotency,
}
#[derive(Clone)]
pub enum ConductTypeSkillPotency {
    Attack(ConductTypeSkillPotencyAttack),   // 攻撃
    Support(ConductTypeSkillPotencySupport), // 支援
}
#[derive(Clone)]
pub struct ConductTypeSkillPotencyAttack {
    pub attack_power: AttackPower,                // 基礎攻撃力
    pub attack_power_scaling: AttackPowerScaling, // 攻撃力補正
    pub break_power: u32,                         // ブレイク攻撃力
    pub break_power_scaling: f32,                 // ブレイク攻撃力補正
}
#[derive(Clone)]
pub enum ConductTypeSkillPotencySupport {
    StatusEffect(SuportStatusEffect), // 状態変化付与
    Recover(SupportRecover),          // HP回復量
}

// 術
#[derive(Clone)]
pub enum ConductTypeSorcery {
    Attack(ConductTypeSorceryAttack),   // 攻撃
    Support(ConductTypeSorcerySupport), // 支援
}
#[derive(Clone)]
pub struct ConductTypeSorceryAttack {
    pub attack_power: AttackPower, // 基礎攻撃力
    pub break_power: u32,          // ブレイク攻撃力
}
#[derive(Clone)]
pub enum ConductTypeSorcerySupport {
    StatusEffect(SuportStatusEffect), // 状態変化付与
    Recover(SupportRecover),          // HP回復量
}
