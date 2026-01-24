use super::*;

pub fn karma_for_turn_end(battle: &mut Battle) {
    let player = battle.players.first_mut().unwrap();
    let karma = if let Some(karma) = player.karma.as_mut() {
        karma
    } else {
        // TODO: エラーハンドリングを追加
        panic!("Player does not have karma");
    };

    // 場のカルマカードのターン経過処理
    for card in &mut karma.field_cards {
        if card.max_turn > 0 {
            card.max_turn -= 1;
        }
    }

    // ターン終了時にターン数が0になったカードを捨て札に移動
    let (to_discard, to_keep): (Vec<KarmaCard>, Vec<KarmaCard>) = karma
        .field_cards
        .drain(..)
        .partition(|card| card.max_turn == 0);

    // 捨て札に移動
    karma.discard_pile.extend(to_discard);
    // TODO: カルマカードが捨て札に移動したインシデントの追加

    // 残りのカードを場に戻す
    karma.field_cards = to_keep;

    // カルマコストを超えている場合ペナルティ処理
    // 場のカルマコストの総数
    let total_karma_cost: u32 = karma.field_cards.iter().map(|card| card.cost).sum();
    let player = &mut battle.players.first_mut().unwrap();
    let max_karma = player.max_karma();
    if total_karma_cost > max_karma {
        // HP、SPに最大値の4分の1のダメージを与える
        let penalty_damage = player.hp.max_hp / 4;
        let (before_hp, after_hp) = player.hp.damage(penalty_damage);
        let penalty_sp_damage = player.sp.max_sp / 4;
        let (before_sp, after_sp) = player.sp.damage(penalty_sp_damage);

        // TODO: カルマコスト超過ペナルティのインシデントの追加
    }
}
