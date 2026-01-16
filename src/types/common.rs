// 属性
pub enum Attribute {
    Slash,     // 斬撃
    Strike,    // 打撃
    Thrust,    // 刺突
    Impact,    // 衝撃
    Magic,     // 魔力
    Fire,      // 炎
    Lightning, // 雷
    Chaos,     // 混濁
}

// 攻撃力
#[derive(Clone)]
pub struct AttackPower {
    pub slash: u32,     // 斬撃
    pub strike: u32,    // 打撃
    pub thrust: u32,    // 刺突
    pub impact: u32,    // 衝撃
    pub magic: u32,     // 魔力
    pub fire: u32,      // 炎
    pub lightning: u32, // 雷
    pub chaos: u32,     // 混濁
}
impl AttackPower {
    pub fn default() -> Self {
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
    pub fn add_attribute(&mut self, attribute: &Attribute, value: u32) {
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
    pub fn multiply(&mut self, factor: f32) {
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

#[derive(Clone)]
pub struct AttackPowerScaling {
    pub slash: f32,     // 斬撃
    pub strike: f32,    // 打撃
    pub thrust: f32,    // 刺突
    pub impact: f32,    // 衝撃
    pub magic: f32,     // 魔力
    pub fire: f32,      // 炎
    pub lightning: f32, // 雷
    pub chaos: f32,     // 混濁
}
impl AttackPowerScaling {
    pub fn default() -> Self {
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
    pub fn add_attribute(&mut self, attribute: &Attribute, value: f32) {
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

// 能力補正
#[derive(Clone)]
pub struct AbilityScaling {
    pub strength: f32,     // 筋力
    pub dexterity: f32,    // 技量
    pub intelligence: f32, // 知力
    pub faith: f32,        // 信仰
    pub arcane: f32,       // 神秘
    pub agility: f32,      // 敏捷性
}

#[derive(Clone)]
pub struct DefensePower {
    pub slash: u32,     // 斬撃
    pub strike: u32,    // 打撃
    pub thrust: u32,    // 刺突
    pub impact: u32,    // 衝撃
    pub magic: u32,     // 魔力
    pub fire: u32,      // 炎
    pub lightning: u32, // 雷
    pub chaos: u32,     // 混濁
}
