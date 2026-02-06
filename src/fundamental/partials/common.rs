use super::*;

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

    // 合計攻撃力
    pub fn total_power(&self) -> u32 {
        self.slash
            + self.strike
            + self.thrust
            + self.impact
            + self.magic
            + self.fire
            + self.lightning
            + self.chaos
    }

    pub fn add(&mut self, other: &AttackPower) {
        self.slash += other.slash;
        self.strike += other.strike;
        self.thrust += other.thrust;
        self.impact += other.impact;
        self.magic += other.magic;
        self.fire += other.fire;
        self.lightning += other.lightning;
        self.chaos += other.chaos;
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

    pub fn multiply_attribute(&mut self, attribute: &Attribute, factor: f32) {
        match attribute {
            Attribute::Slash => {
                self.slash = (self.slash as f32 * factor) as u32;
            }
            Attribute::Strike => {
                self.strike = (self.strike as f32 * factor) as u32;
            }
            Attribute::Thrust => {
                self.thrust = (self.thrust as f32 * factor) as u32;
            }
            Attribute::Impact => {
                self.impact = (self.impact as f32 * factor) as u32;
            }
            Attribute::Magic => {
                self.magic = (self.magic as f32 * factor) as u32;
            }
            Attribute::Fire => {
                self.fire = (self.fire as f32 * factor) as u32;
            }
            Attribute::Lightning => {
                self.lightning = (self.lightning as f32 * factor) as u32;
            }
            Attribute::Chaos => {
                self.chaos = (self.chaos as f32 * factor) as u32;
            }
        }
    }
}
