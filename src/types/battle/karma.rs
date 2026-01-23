use super::*;

struct BattleKarma {
    // 山札
    pub draw_pile: Vec<KarmaCard>,
    // 捨て札
    pub discard_pile: Vec<KarmaCard>,

    // 場の札
    pub field_cards: Vec<KarmaCard>,
}

struct BattleKarmaCard {
    pub card: KarmaCard,
    pub remaining_turns: u32,
}
