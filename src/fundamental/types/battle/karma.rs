use super::*;

#[derive(Debug)]
pub struct BattleKarma {
    // 山札
    pub draw_pile: Vec<KarmaCard>,
    // 捨て札
    pub discard_pile: Vec<KarmaCard>,

    // 場の札
    pub field_cards: Vec<BattleKarmaCard>,
}

#[derive(Debug)]
pub struct BattleKarmaCard {
    pub card: KarmaCard,
    pub remaining_turns: u32,
}
