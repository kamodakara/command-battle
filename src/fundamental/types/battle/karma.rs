use super::*;

#[derive(Debug)]
pub struct BattleKarma {
    // 山札（カードプールへの参照）
    pub draw_pile: Vec<KarmaDeckCard>,
    // 捨て札（カードプールへの参照）
    pub discard_pile: Vec<KarmaDeckCard>,

    // 場の札（ドロー時にカードデータを解決済み）
    pub field_cards: Vec<BattleKarmaCard>,
}

#[derive(Debug)]
pub struct BattleKarmaCard {
    pub card_id: KarmaCardId, // 捨て札へ戻す際に使用
    pub card: KarmaCard,      // ドロー時に解決したカードデータのスナップショット
    pub remaining_turns: u32,
}
