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
    pub conduct_type: ConductType,       // 戦闘行動内容
}

// 戦闘行動特性
#[derive(PartialEq)]
pub enum ConductPerk {
    // 近距離
    Melee,
    // 遠距離
    Ranged,
    // 足元
    AtFeet,
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
pub enum ConductType {
    Basic(ConductTypeBasic),     // 基本
    Skill(ConductTypeSkill),     // 技
    Sorcery(ConductTypeSorcery), // 術
}
// 戦闘行動基本
pub enum ConductTypeBasic {
    Attack(ConductTypeBasicAttack),   // 攻撃
    Support(ConductTypeBasicSupport), // 支援
}
pub struct ConductTypeBasicAttack {
    pub attack_power: AttackPower, // 攻撃力
    pub break_power: u32,          // ブレイク攻撃力
}
pub enum ConductTypeBasicSupport {
    StatusEffect(SuportStatusEffect), // 状態変化付与
}

pub struct SuportStatusEffect {
    pub status_effects: Vec<StatusEffect>,
}

// 戦闘行動技
pub struct ConductTypeSkill {
    // 使用可能武器
    pub usable_weapon_kinds: Vec<WeaponKind>,
    pub potency: ConductTypeSkillPotency,
}
pub enum ConductTypeSkillPotency {
    Attack(ConductTypeSkillPotencyAttack),   // 攻撃
    Support(ConductTypeSkillPotencySupport), // 支援
}
pub struct ConductTypeSkillPotencyAttack {
    pub attack_power: AttackPower,                // 基礎攻撃力
    pub attack_power_scaling: AttackPowerScaling, // 攻撃力補正
    pub break_power: u32,                         // ブレイク攻撃力
    pub break_power_scaling: f32,                 // ブレイク攻撃力補正
}
pub enum ConductTypeSkillPotencySupport {
    StatusEffect(SuportStatusEffect), // 状態変化付与
}

// 術
pub enum ConductTypeSorcery {
    Attack(ConductTypeSorceryAttack),   // 攻撃
    Support(ConductTypeSorcerySupport), // 支援
}
pub struct ConductTypeSorceryAttack {
    pub attack_power: AttackPower, // 基礎攻撃力
    pub break_power: u32,          // ブレイク攻撃力
}
pub enum ConductTypeSorcerySupport {
    StatusEffect(SuportStatusEffect), // 状態変化付与
}
