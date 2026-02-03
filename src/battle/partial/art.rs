use super::*;

pub trait ArtPartialTrait {
    // 有効なランクを返す
    fn effective_rank(&self, sorcery_power: u32) -> &ArtRank;
}
impl ArtPartialTrait for Art {
    // 有効なランクを返す
    // 効果ランク判定
    // 術力のみの想定なので術力でランク判定
    fn effective_rank(&self, sorcery_power: u32) -> &ArtRank {
        if let Some(rank3) = &self.rank3
            && sorcery_power >= rank3.threshold
        {
            rank3 // ランク3適用
        } else if let Some(rank2) = &self.rank2
            && sorcery_power >= rank2.threshold
        {
            rank2 // ランク2適用
        } else {
            &self.rank1 // ランク1適用
        }
    }
}

pub trait ArtPotencyAttackPartialTrait {
    // 最終的な攻撃力を取得する
    fn final_attack_power(&self, weapon_attack_power: &AttackPower) -> AttackPower;
    // 最終的なブレイク力を取得する
    fn final_break_power(&self, weapon_break_power: u32) -> u32;
}
impl ArtPotencyAttackPartialTrait for ArtPotencyAttack {
    // 最終的な攻撃力を取得する
    fn final_attack_power(&self, weapon_attack_power: &AttackPower) -> AttackPower {
        let mut attack_power = self.attack_power.clone();
        let mut weapon_power = weapon_attack_power.clone();

        // 武器攻撃力補正をかける
        weapon_power.slash =
            (weapon_power.slash as f32 * self.weapon_attack_power_scaling.slash) as u32;
        weapon_power.strike =
            (weapon_power.strike as f32 * self.weapon_attack_power_scaling.strike) as u32;
        weapon_power.thrust =
            (weapon_power.thrust as f32 * self.weapon_attack_power_scaling.thrust) as u32;
        weapon_power.impact =
            (weapon_power.impact as f32 * self.weapon_attack_power_scaling.impact) as u32;
        weapon_power.magic =
            (weapon_power.magic as f32 * self.weapon_attack_power_scaling.magic) as u32;
        weapon_power.fire =
            (weapon_power.fire as f32 * self.weapon_attack_power_scaling.fire) as u32;
        weapon_power.lightning =
            (weapon_power.lightning as f32 * self.weapon_attack_power_scaling.lightning) as u32;
        weapon_power.chaos =
            (weapon_power.chaos as f32 * self.weapon_attack_power_scaling.chaos) as u32;

        // 武器攻撃力を加算
        attack_power.add(&weapon_power);

        attack_power
    }

    // 最終的なブレイク力を取得する
    fn final_break_power(&self, weapon_break_power: u32) -> u32 {
        let mut break_power = self.break_power;

        // 武器の破壊力補正をかける
        break_power += (weapon_break_power as f32 * self.weapon_break_power_scaling) as u32;

        break_power
    }
}
