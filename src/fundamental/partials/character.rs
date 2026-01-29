use super::*;

impl Ability {
    // 基礎防御性能を取得する
    pub fn base_defense_power(&self) -> DefensePower {
        DefensePower {
            slash: calc_def_from_ability(self, [1.2, 0.5, 1.0, 1.0, 1.5, 1.0, 0.5, 0.5, 0.5]),
            strike: calc_def_from_ability(self, [1.2, 0.5, 1.0, 1.0, 1.0, 1.5, 0.5, 0.5, 0.5]),
            thrust: calc_def_from_ability(self, [1.2, 0.5, 1.0, 1.0, 1.0, 1.5, 0.5, 0.5, 0.5]),
            impact: calc_def_from_ability(self, [1.2, 0.5, 1.0, 1.0, 1.5, 1.0, 0.5, 0.5, 0.5]),
            magic: calc_def_from_ability(self, [1.2, 1.0, 0.5, 1.0, 0.5, 0.5, 1.4, 1.0, 1.0]),
            fire: calc_def_from_ability(self, [1.2, 1.0, 0.5, 1.0, 0.5, 0.5, 1.4, 1.0, 1.5]),
            lightning: calc_def_from_ability(self, [1.2, 1.0, 0.5, 1.0, 0.5, 0.5, 1.4, 1.5, 1.0]),
            chaos: calc_def_from_ability(self, [1.2, 1.0, 0.5, 1.0, 0.5, 0.5, 0.5, 1.5, 1.5]),
        }
    }

    // プレイヤーステータスを算出する
    pub fn player_stats(&self) -> PlayerStats {
        PlayerStats {
            hp: 50 + self.vitality * 2,
            sp: 10 + (self.spirit as f32 * 1.5) as u32,
            stamina: 50 + (self.endurance as f32 * 1.5) as u32,
            stamina_recovery: 5
                + ((self.endurance as f32 * 0.5) + (self.vitality as f32 * 0.5)) as u32,
            equip_load: 30 + (self.endurance as f32 * 1.0) as u32,
        }
    }
}

fn calc_def_from_ability(ability: &Ability, coef: [f64; 9]) -> u32 {
    (ability.vitality as f64 * coef[0]
        + ability.spirit as f64 * coef[1]
        + ability.endurance as f64 * coef[2]
        + ability.agility as f64 * coef[3]
        + ability.strength as f64 * coef[4]
        + ability.dexterity as f64 * coef[5]
        + ability.intelligence as f64 * coef[6]
        + ability.faith as f64 * coef[7]
        + ability.arcane as f64 * coef[8]) as u32
}
