use super::*;

impl BattleCharacter {
    pub fn current_ability(&self) -> Ability {
        // TODO: ステータス変化や装備による補正を考慮する
        self.raw_ability.clone()
    }

    pub fn defense_power(&self) -> DefensePower {
        // 能力の防御力
        let ability = self.current_ability();
        let mut defense_power = create_player_defense_power(&ability);

        // 装備の防御力を加算
        if let Some(armor) = &self.raw_equipment.armor1 {
            defense_power.add(&armor.defense);
        }
        if let Some(armor) = &self.raw_equipment.armor2 {
            defense_power.add(&armor.defense);
        }
        if let Some(armor) = &self.raw_equipment.armor3 {
            defense_power.add(&armor.defense);
        }
        if let Some(armor) = &self.raw_equipment.armor4 {
            defense_power.add(&armor.defense);
        }
        if let Some(armor) = &self.raw_equipment.armor5 {
            defense_power.add(&armor.defense);
        }
        if let Some(armor) = &self.raw_equipment.armor6 {
            defense_power.add(&armor.defense);
        }
        if let Some(armor) = &self.raw_equipment.armor7 {
            defense_power.add(&armor.defense);
        }
        if let Some(armor) = &self.raw_equipment.armor8 {
            defense_power.add(&armor.defense);
        }

        defense_power
    }

    pub fn weapon_performance(&self, weapon_id: &BattleWeaponId) -> Option<WeaponPerformance> {
        let weapon = if let Some(w) = self.weapons.iter().find(|w| &w.id == weapon_id) {
            w
        } else {
            // TODO: エラー処理
            // Noneを返すだけでいいか要検討
            return None;
        };
        let ability = self.current_ability();

        Some(weapon.weapon.performance(&ability))
    }

    // 最大カルマ値を取得する
    pub fn max_karma(&self) -> u32 {
        let ability = self.current_ability();
        (ability.vitality as f32
            + ability.spirit as f32
            + (ability.intelligence as f32 * 1.5)
            + (ability.faith as f32 * 1.5)) as u32
    }
}

impl BattleCharacterHP {
    pub fn damage(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_hp;
        self.current_hp = self.current_hp.saturating_sub(amount);
        let after = self.current_hp;
        (before, after)
    }
    pub fn recover(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_hp;
        self.current_hp = std::cmp::min(self.current_hp + amount, self.max_hp);
        let after = self.current_hp;
        (before, after)
    }
}

impl BattleCharacterSP {
    pub fn damage(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_sp;
        self.current_sp = self.current_sp.saturating_sub(amount);
        let after = self.current_sp;
        (before, after)
    }
    pub fn recover(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_sp;
        self.current_sp = std::cmp::min(self.current_sp + amount, self.max_sp);
        let after = self.current_sp;
        (before, after)
    }
}

impl BattleCharacterStamina {
    pub fn damage(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_stamina;
        self.current_stamina = self.current_stamina.saturating_sub(amount);
        let after = self.current_stamina;
        (before, after)
    }
    pub fn recover(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_stamina;
        self.current_stamina = std::cmp::min(self.current_stamina + amount, self.max_stamina);
        let after = self.current_stamina;
        (before, after)
    }
}

// ブレイク耐性
impl BattleCharacterBreak {
    pub fn damage(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_break;
        self.current_break = self.current_break.saturating_sub(amount);
        let after = self.current_break;

        // ブレイクダメージを受けたのでターン数リセット
        self.break_not_damaged_turns = 0;

        (before, after)
    }
    pub fn recover(&mut self, amount: u32) -> (u32, u32) {
        let before = self.current_break;
        self.current_break = std::cmp::min(self.current_break + amount, self.max_break);
        let after = self.current_break;
        (before, after)
    }

    // ブレイクターン経過
    pub fn elapse_breaking_turn(&mut self) -> (u32, u32) {
        let before = self.remaining_breaking_turns;
        self.remaining_breaking_turns.saturating_sub(1);
        let after = self.remaining_breaking_turns;
        (before, after)
    }

    // ブレイク中、解除
    pub fn clear_breaking(&mut self) {
        self.is_breaking = false;
        self.remaining_breaking_turns = 0;
    }
}
