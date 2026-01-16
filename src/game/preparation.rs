use bevy::prelude::*;

use super::*;

// ================== Components ==================

#[derive(Component)]
pub struct PreparationScreen;

#[derive(Component)]
pub struct StartBattleButton;

// ================== Plugin ==================
pub struct PreparationPlugin;

impl Plugin for PreparationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Preparation), setup_preparation_screen)
            .add_systems(
                Update,
                preparation_button_system.run_if(in_state(GameState::Preparation)),
            )
            .add_systems(OnExit(GameState::Preparation), cleanup_preparation_screen);
    }
}

// ================== Systems ==================

/// 戦闘準備画面のセットアップ
pub fn setup_preparation_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    commands
        .spawn((
            PreparationScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(40.0),
                ..default()
            },
            BackgroundColor(Color::from(LinearRgba {
                red: 0.1,
                green: 0.1,
                blue: 0.15,
                alpha: 1.0,
            })),
        ))
        .with_children(|parent| {
            // タイトル
            parent.spawn((
                Text::new("戦闘準備"),
                TextFont {
                    font: font.clone(),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // 戦闘開始ボタン
            parent
                .spawn((
                    StartBattleButton,
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.25,
                        green: 0.25,
                        blue: 0.35,
                        alpha: 1.0,
                    })),
                    BorderColor::all(Color::WHITE),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("戦闘開始"),
                        TextFont {
                            font: font.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

/// 準備画面のクリーンアップ
pub fn cleanup_preparation_screen(
    mut commands: Commands,
    query: Query<Entity, With<PreparationScreen>>,
) {
    for entity in query.iter() {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn();
        }
    }
}

/// ボタンクリックで戦闘開始
pub fn preparation_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartBattleButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Battle);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.35,
                    green: 0.35,
                    blue: 0.45,
                    alpha: 1.0,
                }));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.25,
                    green: 0.25,
                    blue: 0.35,
                    alpha: 1.0,
                }));
            }
        }
    }
}
