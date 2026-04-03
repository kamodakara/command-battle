use super::*;
use crate::data::KarmaCardRepository;

// カルマカードを引く処理
// 山札からカルマカードを1枚引き、場に出す。
// TODO: インシデントの追加
pub fn karma_draw_card(battle: &mut Battle, card_repo: &KarmaCardRepository) {
    let player = &mut battle.player;
    let karma = if let Some(karma) = player.karma.as_mut() {
        karma
    } else {
        // TODO: エラーハンドリングを追加
        panic!("Player does not have karma");
    };

    // 山札からカードを引く処理
    let draw_card = if let Some(drawn_card) = karma.draw_pile.pop() {
        // 引いたカードを場に出す
        Some(drawn_card)
    } else {
        // 山札が空の場合、捨て札をシャッフルして山札に戻す
        karma.draw_pile.append(&mut karma.discard_pile);
        // 山札をシャッフルする（シャッフルロジックは省略）

        // TODO: シャッフルのインシデント

        // 再度カードを引く
        if let Some(drawn_card) = karma.draw_pile.pop() {
            Some(drawn_card)
        } else {
            // 山札も捨て札も空の場合は何もしない
            None
        }
    };

    // 場にカードを追加
    if let Some(deck_card) = draw_card {
        // カードプールからカードデータを解決する
        if let Some(record) = card_repo.find_by_id(deck_card.card_id.0) {
            let card = record.data.clone();
            let remaining_turns = card.max_turn;
            karma.field_cards.push(BattleKarmaCard {
                card_id: deck_card.card_id,
                card,
                remaining_turns,
            });
        }
    }

    // TODO: カードドローのインシデント
}
