use super::common::DefensePower;
use super::equipment::Equipment;

// 敵
pub struct Enemy {
    pub ability: EnemyAbility, // 能力
    pub stats: EnemyStats,     // ステータス
    pub equipment: Equipment,  // 装備
}

// 敵能力
pub struct EnemyAbility {
    pub agility: u32,      // 敏捷性
    pub strength: u32,     // 筋力
    pub dexterity: u32,    // 技量
    pub intelligence: u32, // 知力
    pub faith: u32,        // 信仰
    pub arcane: u32,       // 神秘
}

pub struct EnemyStats {
    pub hp: u32,             // HP
    pub sp: u32,             // SP
    pub break_max: u32,      // ブレイク最大値
    pub break_recovery: u32, // ブレイク回復量
    pub break_turn: u32,     // ブレイクターン
}

pub struct Player {
    pub ability: PlayerAbility,           // 能力
    pub stats: PlayerStats,               // ステータス
    pub base_defense_power: DefensePower, // 基礎防御力
    pub equipment: Equipment,             // 装備
}
// プレイヤー能力
pub struct PlayerAbility {
    pub vitality: u32,     // 生命力
    pub spirit: u32,       // 精神力
    pub endurance: u32,    // 持久力
    pub agility: u32,      // 敏捷性
    pub strength: u32,     // 筋力
    pub dexterity: u32,    // 技量
    pub intelligence: u32, // 知力
    pub faith: u32,        // 信仰
    pub arcane: u32,       // 神秘
}

// プレイヤーステータス
pub struct PlayerStats {
    pub hp: u32,               // HP
    pub sp: u32,               // SP
    pub stamina: u32,          // スタミナ
    pub stamina_recovery: u32, // スタミナ回復量
    pub equip_load: u32,       // 装備重量
}

// TODO: プレイヤー基礎状態異常耐性
