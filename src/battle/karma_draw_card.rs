use super::*;

// カルマカードを引く処理
// 山札からカルマカードを1枚引き、場に出す。
// TODO: インシデントの追加
pub fn karma_draw_card(battle: &mut Battle) {
    let player = battle.players.first_mut().unwrap();
    let karma = if let Some(karma) = player.karma.as_mut() {
        karma
    } else {
        // TODO: エラーハンドリングを追加
        panic!("Player does not have karma");
    };

    // 山札からカードを引く処理
    let draw_card = if let Some(drawn_card) = karma.draw_pile.pop() {
        // 引いたカードを場に出す
        drawn_card
    } else {
        // 山札が空の場合、捨て札をシャッフルして山札に戻す
        karma.draw_pile.append(&mut karma.discard_pile);
        // 山札をシャッフルする（シャッフルロジックは省略）

        // TODO: シャッフルのインシデント

        // 再度カードを引く
        if let Some(drawn_card) = karma.draw_pile.pop() {
            drawn_card
        } else {
            // 山札も捨て札も空の場合は何もしない
            // TDO: エラーハンドリングを追加
            panic!("No cards available to draw");
        }
    };
    // 場にカードを追加
    karma.field_cards.push(draw_card);

    // TODO: カードドローのインシデント
}
