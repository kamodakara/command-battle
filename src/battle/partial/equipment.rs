use super::*;

impl GuardCutRate {
    // ガードカット後の攻撃力を取得する
    pub fn apply_guard_cut(&self, attack_power: &AttackPower) -> AttackPower {
        let mut adjusted_attack_power = attack_power.clone();

        adjusted_attack_power.slash =
            (adjusted_attack_power.slash as f32 * (1.0 - self.slash)) as u32;
        adjusted_attack_power.strike =
            (adjusted_attack_power.strike as f32 * (1.0 - self.strike)) as u32;
        adjusted_attack_power.thrust =
            (adjusted_attack_power.thrust as f32 * (1.0 - self.thrust)) as u32;
        adjusted_attack_power.impact =
            (adjusted_attack_power.impact as f32 * (1.0 - self.impact)) as u32;
        adjusted_attack_power.magic =
            (adjusted_attack_power.magic as f32 * (1.0 - self.magic)) as u32;
        adjusted_attack_power.fire = (adjusted_attack_power.fire as f32 * (1.0 - self.fire)) as u32;
        adjusted_attack_power.lightning =
            (adjusted_attack_power.lightning as f32 * (1.0 - self.lightning)) as u32;
        adjusted_attack_power.chaos =
            (adjusted_attack_power.chaos as f32 * (1.0 - self.chaos)) as u32;

        adjusted_attack_power
    }
}
