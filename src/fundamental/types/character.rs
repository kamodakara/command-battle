use super::art::Art;
use super::common::DefensePower;
use super::equipment::Equipment;

// 敵
pub struct Enemy {
    pub ability: Ability,     // 能力
    pub stats: EnemyStats,    // ステータス
    pub equipment: Equipment, // 装備
}

pub struct EnemyStats {
    pub hp: u32,             // HP
    pub sp: u32,             // SP
    pub break_max: u32,      // ブレイク最大値
    pub break_recovery: u32, // ブレイク回復量
    pub break_turn: u32,     // ブレイクターン
}

pub struct Player {
    pub ability: Ability,                                        // 能力
    pub stats: PlayerStats,                                      // ステータス
    pub base_defense_power: DefensePower,                        // 基礎防御力
    pub equipment: Equipment,                                    // 装備
    pub arts: Vec<Art>,                                          // アーツ
    pub base_status_ailment_resistance: StatusAilmentResistance, // 基礎状態異常耐性
}

// プレイヤー能力
#[derive(Clone, Debug)]
pub struct Ability {
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

#[derive(Clone, Debug)]
pub enum AbilityType {
    Vitality,     // 生命力
    Spirit,       // 精神力
    Endurance,    // 持久力
    Agility,      // 敏捷性
    Strength,     // 筋力
    Dexterity,    // 技量
    Intelligence, // 知力
    Faith,        // 信仰
    Arcane,       // 神秘
}

// プレイヤーステータス
pub struct PlayerStats {
    pub hp: u32,               // HP
    pub sp: u32,               // SP
    pub stamina: u32,          // スタミナ
    pub stamina_recovery: u32, // スタミナ回復量
    pub equip_load: u32,       // 装備重量
}

// 状態異常耐性
pub struct StatusAilmentResistance {
    pub poison: u32,    // 毒耐性
    pub sleep: u32,     // 眠気耐性
    pub chill: u32,     // 寒気耐性
    pub bleed: u32,     // 出血耐性
    pub burn: u32,      // 火傷耐性
    pub paralysis: u32, // 麻痺耐性
    pub fear: u32,      // 恐怖耐性
    pub rage: u32,      // 激昂耐性
}
