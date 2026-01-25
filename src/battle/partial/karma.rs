use super::*;

impl BattleKarma {
    pub fn current_effects(&self) -> Vec<KarmaEffect> {
        self.field_cards
            .iter()
            .map(|card| card.effects.clone())
            .flatten()
            .collect()
    }
}
