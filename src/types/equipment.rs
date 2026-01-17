use super::common::*;

// 装備
pub struct Equipment {
    pub weapon1: Option<Weapon>, // 右手
    pub weapon2: Option<Weapon>, // 左手
    pub armor1: Option<Armor>,   // 防具
    pub armor2: Option<Armor>,   // 防具
    pub armor3: Option<Armor>,   // 防具
    pub armor4: Option<Armor>,   // 防具
    pub armor5: Option<Armor>,   // 防具
    pub armor6: Option<Armor>,   // 防具
    pub armor7: Option<Armor>,   // 防具
    pub armor8: Option<Armor>,   // 防具
}

// 武器
#[derive(Clone)]
pub struct Weapon {
    pub kind: WeaponKind,                              // 種類
    pub weight: u32,                                   // 重量
    pub ability_requirement: WeaponAbilityRequirement, // 必要能力
    pub attack_power: WeaponAttackPower,               // 攻撃性能
    pub sorcery_power: WeaponSorceryPower,             // 術力
    pub break_power: WeaponBreakPower,                 // ブレイク力
    pub guard: WeaponGuard,                            // 防御性能
}

#[derive(Clone)]
pub enum WeaponKind {
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
#[derive(Clone)]
pub struct WeaponAbilityRequirement {
    pub strength: u32,     // 筋力
    pub dexterity: u32,    // 技量
    pub intelligence: u32, // 知力
    pub faith: u32,        // 信仰
    pub arcane: u32,       // 神秘
    pub agility: u32,      // 敏捷性
}

// 武器攻撃性能
#[derive(Clone)]
pub struct WeaponAttackPower {
    pub base: AttackPower,                                // 基礎攻撃力
    pub ability_scaling: WeaponAttackPowerAbilityScaling, // 能力補正
}

// 武器攻撃力能力補正
#[derive(Clone)]
pub struct WeaponAttackPowerAbilityScaling {
    pub slash: AbilityScaling,     // 斬撃
    pub strike: AbilityScaling,    // 打撃
    pub thrust: AbilityScaling,    // 刺突
    pub impact: AbilityScaling,    // 衝撃
    pub magic: AbilityScaling,     // 魔力
    pub fire: AbilityScaling,      // 炎
    pub lightning: AbilityScaling, // 雷
    pub chaos: AbilityScaling,     // 混濁
}

// 武器術力
#[derive(Clone)]
pub struct WeaponSorceryPower {
    pub base: u32,               // 基礎力
    pub scaling: AbilityScaling, // 能力補正
}

// 武器ブレイク力
#[derive(Clone)]
pub struct WeaponBreakPower {
    pub base_power: u32,         // 基礎力
    pub scaling: AbilityScaling, // 能力補正
}

// 武器防御性能
#[derive(Clone)]
pub struct WeaponGuard {
    pub cut_rate: GuardCutRate, // カット率
    pub guard_strength: u32,    // ガード強度
}

// 武器防御カット率
// 攻撃力補正値が入る0.0〜1.0の範囲
// 0.0で100%カット
#[derive(Clone)]
pub struct GuardCutRate {
    pub slash: f32,     // 斬撃
    pub strike: f32,    // 打撃
    pub thrust: f32,    // 刺突
    pub impact: f32,    // 衝撃
    pub magic: f32,     // 魔力
    pub fire: f32,      // 炎
    pub lightning: f32, // 雷
    pub chaos: f32,     // 混濁
}

// 防具
#[derive(Clone)]
pub struct Armor {
    pub kind: ArmorKind,             // 種類
    pub weight: u32,                 // 重量
    pub defense: ArmorDefense,       // 防御力
    pub resistance: ArmorResistance, // 状態異常耐性値
    pub slots: Vec<ArmorSlot>,       // 装備箇所
}

// 防具種類
#[derive(Clone)]
pub enum ArmorKind {
    Helmet,     // 頭装備
    ChestArmor, // 胴装備
    Gauntlets,  // 腕装備
    LegArmor,   // 脚装備
}

// 防具防御力
#[derive(Clone)]
pub struct ArmorDefense {
    pub slash: u32,     // 斬撃
    pub strike: u32,    // 打撃
    pub thrust: u32,    // 刺突
    pub impact: u32,    // 衝撃
    pub magic: u32,     // 魔力
    pub fire: u32,      // 炎
    pub lightning: u32, // 雷
    pub chaos: u32,     // 混濁
}
// 防具状態異常耐性値
#[derive(Clone)]
pub struct ArmorResistance {
    pub immunity: u32,   // 免疫
    pub robustness: u32, // 頑健
    pub sanity: u32,     // 正気
}

// 防具装備箇所
#[derive(Clone, PartialEq)]
pub enum ArmorSlot {
    Head,  // 頭装備
    Chest, // 胴装備
    Arms,  // 腕装備
    Legs,  // 脚装備
}
