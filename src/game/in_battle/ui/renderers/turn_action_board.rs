use super::super::resources::TurnActionBoard;
use bevy::prelude::*;

#[derive(Component)]
pub struct UiTurnActionBoard;

#[derive(Component)]
pub struct UiActionBoardPlayerText(pub usize);

#[derive(Component)]
pub struct UiActionBoardEnemyText(pub usize);

pub fn render(
    board: Res<TurnActionBoard>,
    mut player_texts: Query<
        (&UiActionBoardPlayerText, &mut Text, &mut TextColor),
        Without<UiActionBoardEnemyText>,
    >,
    mut enemy_texts: Query<
        (&UiActionBoardEnemyText, &mut Text, &mut TextColor),
        Without<UiActionBoardPlayerText>,
    >,
) {
    let gray = Color::from(LinearRgba {
        red: 0.40,
        green: 0.40,
        blue: 0.40,
        alpha: 1.0,
    });
    let white = Color::WHITE;
    let unknown = Color::from(LinearRgba {
        red: 0.70,
        green: 0.55,
        blue: 0.30,
        alpha: 1.0,
    });

    for (UiActionBoardPlayerText(idx), mut text, mut color) in player_texts.iter_mut() {
        text.0 = board.player_actions[*idx]
            .clone()
            .unwrap_or_else(|| "---".to_string());
        color.0 = if board.executed[*idx] { gray } else { white };
    }

    for (UiActionBoardEnemyText(idx), mut text, mut color) in enemy_texts.iter_mut() {
        match &board.enemy_actions[*idx] {
            Some(name) => {
                text.0 = name.clone();
                color.0 = if board.executed[*idx] { gray } else { white };
            }
            None => {
                text.0 = "？？？".to_string();
                color.0 = if board.executed[*idx] { gray } else { unknown };
            }
        }
    }
}
