use bevy::prelude::*;

/// 戻るボタンのマーカーコンポーネント
#[derive(Component)]
pub struct BackToPreparationButton;

pub fn back_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackToPreparationButton>),
    >,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(crate::GameState::Preparation);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.45,
                    green: 0.3,
                    blue: 0.3,
                    alpha: 0.9,
                }));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.3,
                    green: 0.2,
                    blue: 0.2,
                    alpha: 0.9,
                }));
            }
        }
    }
}
