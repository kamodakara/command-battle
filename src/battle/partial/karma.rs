use super::*;

impl BattleKarma {
    // 現在の場のカルマ効果を取得する
    pub fn field_effects(&self) -> Vec<KarmaEffect> {
        let effects: Vec<KarmaEffect> = self
            .field_cards
            .iter()
            .map(|card| card.card.effects.clone())
            .flatten()
            .collect();
        effects
    }
}
