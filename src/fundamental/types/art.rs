use serde::{Deserialize, Serialize};

use super::common::*;
use super::equipment::WeaponKind;
use super::status_ailment::{StatusAilment, StatusCondition};

// 技能
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Art {
    pub name: String,                   // 名前
    pub sp_cost: u32,                   // SP消費
    pub stamina_cost: u32,              // スタミナ消費
    pub perks: Vec<ArtPerk>,            // 特性
    pub requirement: ArtRequirement,    // 必要能力
    pub art_type: ArtType,              // 技能内容
    pub usable_weapon: ArtUsableWeapon, // 使用可能武器種、技のみ武器種が指定される想定、技以外は全武器種
    pub always_hits: bool,              // 必中、回復など用
    pub priority: i32,                  // 行動優先度、高い方ほど先に行動する
    // 技能ランク
    // 条件を満たすと上位ランクの効果が適用される(術のみ想定)
    pub rank1: ArtRank,         // 技能ランク
    pub rank2: Option<ArtRank>, // 技能ランク2、術のみ想定
    pub rank3: Option<ArtRank>, // 技能ランク3、術のみ想定
}

// 技能特性
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum ArtPerk {
    Melee,  // 近距離
    Ranged, // 遠距離
    AtFeet, // 足元
    Guard,  // ガード
}

// 必要能力
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtRequirement {
    pub strength: u32,     // 筋力
    pub dexterity: u32,    // 技量
    pub intelligence: u32, // 知力
    pub faith: u32,        // 信仰
    pub arcane: u32,       // 神秘
    pub agility: u32,      // 敏捷性
}

// 戦闘行動内容
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ArtType {
    Basic,   // 基本
    Skill,   // 技
    Sorcery, // 術
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ArtUsableWeapon {
    All,                       // 全ての武器種
    Specific(Vec<WeaponKind>), // 特定の武器種
}

// 技能ランク
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtRank {
    pub threshold: u32,      // 条件閾値、術のみ想定で術力で判定、レベル1は参照しない
    pub target: ArtTarget,   // 対象
    pub potency: ArtPotency, // 効果
}

// 技能対象
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ArtTarget {
    Single, // 単体
    All,    // 全体
}

// 技能効果
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ArtPotency {
    Attack(ArtPotencyAttack),   // 攻撃
    Support(ArtPotencySupport), // 支援
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtPotencyAttack {
    pub attack_power: AttackPower,                       // 攻撃力、基礎
    pub weapon_attack_power_scaling: AttackPowerScaling, // 武器攻撃力補正、技のみ想定
    pub break_power: u32,                                // ブレイク攻撃力
    pub weapon_break_power_scaling: f32,                 // ブレイク攻撃力補正、技のみ想定
    #[serde(default)]
    pub additional_effects: Vec<AdditionalEffect>, // 追加効果
}

// 追加効果の対象
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AdditionalEffectTarget {
    User,         // アーツ使用者
    AttackTarget, // 攻撃対象者
}

// 追加効果
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdditionalEffect {
    pub target: AdditionalEffectTarget, // 対象
    pub content: ArtPotencySupport,     // 効果内容
}

// 技能支援効果
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ArtPotencySupport {
    None,                                              // 行動しない
    StatusCondition(ArtPotencySupportStatusCondition), // 状態変化付与
    Recover(ArtPotencySupportRecover),                 // HP回復量
    StatusAilment(ArtPotencySupportStatusAilment),     // 状態異常蓄積
}
// 技能支援効果 状態異常蓄積
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtPotencySupportStatusAilment {
    pub kind: StatusAilment, // 状態異常種別
    pub accumulation: u32,   // 蓄積量
}
// 技能支援効果 状態変化付与
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtPotencySupportStatusCondition {
    pub status_conditions: Vec<StatusCondition>,
}
// 技能支援効果 回復
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtPotencySupportRecover {
    pub potencies: Vec<SupportRecoverPotency>, // 回復効果
}

// 支援回復効果
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SupportRecoverPotency {
    Hp(SupportRecoverPotencyHp),           // HP回復量
    Sp(SupportRecoverPotencySp),           // SP回復量
    Stamina(SupportRecoverPotencyStamina), // スタミナ回復量
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupportRecoverPotencyHp {
    pub hp_recover: u32, // HP回復量
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupportRecoverPotencySp {
    pub sp_recover: u32, // SP回復量
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupportRecoverPotencyStamina {
    pub stamina_recover: u32, // スタミナ回復量
}
