use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use super::*;

// ================== Components ==================

#[derive(Component)]
pub struct PreparationScreen;

#[derive(Component)]
pub struct MenuPanel;

#[derive(Component)]
pub struct ContentPanel;

#[derive(Component)]
pub struct MenuButton {
    pub menu_type: MenuType,
}

#[derive(Component)]
pub struct StartBattleButton;

// ================== Resources ==================

#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub enum MenuType {
    #[default]
    Status,
    Equipment,
    StartBattle,
}

#[derive(Resource, Default)]
pub struct PreparationState {
    pub current_menu: MenuType,
    pub status_points: u32,
    pub temp_vitality: u32,
    pub temp_spirit: u32,
    pub temp_endurance: u32,
    pub temp_agility: u32,
    pub temp_strength: u32,
    pub temp_dexterity: u32,
    pub temp_intelligence: u32,
    pub temp_faith: u32,
    pub temp_arcane: u32,
}

// ================== Plugin ==================
pub struct PreparationPlugin;

impl Plugin for PreparationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreparationState>()
            .add_systems(OnEnter(GameState::Preparation), setup_preparation_screen)
            .add_systems(
                Update,
                (
                    menu_button_system,
                    update_content_panel,
                    status_allocation_system,
                    equipment_selection_system,
                    start_battle_system,
                )
                    .run_if(in_state(GameState::Preparation)),
            )
            .add_systems(OnExit(GameState::Preparation), cleanup_preparation_screen);
    }
}

// ================== Systems ==================

/// 戦闘準備画面のセットアップ
pub fn setup_preparation_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut prep_state: ResMut<PreparationState>,
) {
    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    // 初期化
    prep_state.current_menu = MenuType::Status;
    prep_state.status_points = 10;
    prep_state.temp_vitality = 10;
    prep_state.temp_spirit = 10;
    prep_state.temp_endurance = 10;
    prep_state.temp_agility = 10;
    prep_state.temp_strength = 10;
    prep_state.temp_dexterity = 10;
    prep_state.temp_intelligence = 10;
    prep_state.temp_faith = 10;
    prep_state.temp_arcane = 10;

    commands
        .spawn((
            PreparationScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
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
            // 左側メニュー
            parent
                .spawn((
                    MenuPanel,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(15.0),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.15,
                        green: 0.15,
                        blue: 0.2,
                        alpha: 1.0,
                    })),
                ))
                .with_children(|menu| {
                    // タイトル
                    menu.spawn((
                        Text::new("戦闘準備"),
                        TextFont {
                            font: font.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                    ));

                    // ステータスボタン
                    menu.spawn((
                        MenuButton {
                            menu_type: MenuType::Status,
                        },
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(60.0),
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
                            Text::new("ステータス"),
                            TextFont {
                                font: font.clone(),
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });

                    // 装備ボタン
                    menu.spawn((
                        MenuButton {
                            menu_type: MenuType::Equipment,
                        },
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(60.0),
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
                            Text::new("装備"),
                            TextFont {
                                font: font.clone(),
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });

                    // 戦闘開始ボタン
                    menu.spawn((
                        MenuButton {
                            menu_type: MenuType::StartBattle,
                        },
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(60.0),
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
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                });

            // 右側コンテンツエリア
            parent.spawn((
                ContentPanel,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(40.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            ));
        });
}

/// 準備画面のクリーンアップ
pub fn cleanup_preparation_screen(
    mut commands: Commands,
    query: Query<Entity, With<PreparationScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// メニューボタンのインタラクション
pub fn menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &MenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut prep_state: ResMut<PreparationState>,
) {
    for (interaction, menu_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                prep_state.current_menu = menu_button.menu_type;
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

/// コンテンツパネルの更新
pub fn update_content_panel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    prep_state: Res<PreparationState>,
    panel_query: Query<Entity, With<ContentPanel>>,
    children_query: Query<&Children>,
) {
    if !prep_state.is_changed() {
        return;
    }

    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    for panel_entity in panel_query.iter() {
        // 既存の子要素をすべて削除
        if let Ok(children) = children_query.get(panel_entity) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        // 新しいコンテンツを追加
        let current_menu = prep_state.current_menu;
        let font_clone = font.clone();

        commands
            .entity(panel_entity)
            .with_children(|parent| match current_menu {
                MenuType::Status => {
                    build_status_content(parent, font_clone.clone(), &prep_state);
                }
                MenuType::Equipment => {
                    build_equipment_content(parent, font_clone.clone());
                }
                MenuType::StartBattle => {
                    build_start_battle_content(parent, font_clone);
                }
            });
    }
}

/// ステータス画面のコンテンツを構築
fn build_status_content(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    prep_state: &PreparationState,
) {
    parent.spawn((
        Text::new("ステータス"),
        TextFont {
            font: font.clone(),
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            margin: UiRect::bottom(Val::Px(30.0)),
            ..default()
        },
    ));

    // 残りポイント表示
    parent.spawn((
        Text::new(format!("残りポイント: {}", prep_state.status_points)),
        TextFont {
            font: font.clone(),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.5)),
        Node {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
    ));

    // ステータス一覧
    let stats = [
        ("生命力", prep_state.temp_vitality, StatType::Vitality),
        ("精神力", prep_state.temp_spirit, StatType::Spirit),
        ("持久力", prep_state.temp_endurance, StatType::Endurance),
        ("敏捷性", prep_state.temp_agility, StatType::Agility),
        ("筋力", prep_state.temp_strength, StatType::Strength),
        ("技量", prep_state.temp_dexterity, StatType::Dexterity),
        ("知力", prep_state.temp_intelligence, StatType::Intelligence),
        ("信仰", prep_state.temp_faith, StatType::Faith),
        ("神秘", prep_state.temp_arcane, StatType::Arcane),
    ];

    for (name, value, stat_type) in stats {
        parent
            .spawn(Node {
                width: Val::Px(500.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            })
            .with_children(|row| {
                // ステータス名
                row.spawn((
                    Text::new(name),
                    TextFont {
                        font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        width: Val::Px(120.0),
                        ..default()
                    },
                ));

                // 減少ボタン
                row.spawn((
                    StatButton {
                        stat_type,
                        is_increase: false,
                    },
                    Button,
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.4,
                        green: 0.2,
                        blue: 0.2,
                        alpha: 1.0,
                    })),
                    BorderColor::all(Color::WHITE),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("-"),
                        TextFont {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

                // 値表示
                row.spawn((
                    Text::new(format!("{}", value)),
                    TextFont {
                        font: font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        width: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                ));

                // 増加ボタン
                row.spawn((
                    StatButton {
                        stat_type,
                        is_increase: true,
                    },
                    Button,
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.2,
                        green: 0.4,
                        blue: 0.2,
                        alpha: 1.0,
                    })),
                    BorderColor::all(Color::WHITE),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("+"),
                        TextFont {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            });
    }
}

/// 装備画面のコンテンツを構築
fn build_equipment_content(parent: &mut RelatedSpawnerCommands<ChildOf>, font: Handle<Font>) {
    parent.spawn((
        Text::new("装備"),
        TextFont {
            font: font.clone(),
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            margin: UiRect::bottom(Val::Px(30.0)),
            ..default()
        },
    ));

    // 装備スロット一覧
    let equipment_slots = [
        ("右手武器", EquipmentSlot::Weapon1),
        ("左手武器", EquipmentSlot::Weapon2),
        ("防具1", EquipmentSlot::Armor1),
        ("防具2", EquipmentSlot::Armor2),
        ("防具3", EquipmentSlot::Armor3),
        ("防具4", EquipmentSlot::Armor4),
        ("防具5", EquipmentSlot::Armor5),
        ("防具6", EquipmentSlot::Armor6),
        ("防具7", EquipmentSlot::Armor7),
        ("防具8", EquipmentSlot::Armor8),
    ];

    for (name, slot) in equipment_slots {
        parent
            .spawn(Node {
                width: Val::Px(600.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect::bottom(Val::Px(15.0)),
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            })
            .insert(BackgroundColor(Color::from(LinearRgba {
                red: 0.2,
                green: 0.2,
                blue: 0.25,
                alpha: 1.0,
            })))
            .insert(BorderColor::all(Color::srgb(0.5, 0.5, 0.5)))
            .with_children(|row| {
                // スロット名
                row.spawn((
                    Text::new(name),
                    TextFont {
                        font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // 変更ボタン
                row.spawn((
                    EquipmentButton { slot },
                    Button,
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.3,
                        green: 0.3,
                        blue: 0.4,
                        alpha: 1.0,
                    })),
                    BorderColor::all(Color::WHITE),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("変更"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            });
    }
}

/// 戦闘開始画面のコンテンツを構築
fn build_start_battle_content(parent: &mut RelatedSpawnerCommands<ChildOf>, font: Handle<Font>) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(30.0),
            ..default()
        })
        .with_children(|center| {
            center.spawn((
                Text::new("準備は完了しましたか？"),
                TextFont {
                    font: font.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            center
                .spawn((
                    StartBattleButton,
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.3,
                        green: 0.5,
                        blue: 0.3,
                        alpha: 1.0,
                    })),
                    BorderColor::all(Color::WHITE),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("戦闘開始!"),
                        TextFont {
                            font: font.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

// ================== Components for Status & Equipment ==================

#[derive(Component, Clone, Copy)]
pub struct StatButton {
    pub stat_type: StatType,
    pub is_increase: bool,
}

#[derive(Clone, Copy)]
pub enum StatType {
    Vitality,
    Spirit,
    Endurance,
    Agility,
    Strength,
    Dexterity,
    Intelligence,
    Faith,
    Arcane,
}

#[derive(Component, Clone, Copy)]
pub struct EquipmentButton {
    pub slot: EquipmentSlot,
}

#[derive(Clone, Copy)]
pub enum EquipmentSlot {
    Weapon1,
    Weapon2,
    Armor1,
    Armor2,
    Armor3,
    Armor4,
    Armor5,
    Armor6,
    Armor7,
    Armor8,
}

/// ステータス割り振りシステム
pub fn status_allocation_system(
    mut interaction_query: Query<
        (&Interaction, &StatButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut prep_state: ResMut<PreparationState>,
) {
    for (interaction, stat_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if stat_button.is_increase {
                    if prep_state.status_points > 0 {
                        match stat_button.stat_type {
                            StatType::Vitality => prep_state.temp_vitality += 1,
                            StatType::Spirit => prep_state.temp_spirit += 1,
                            StatType::Endurance => prep_state.temp_endurance += 1,
                            StatType::Agility => prep_state.temp_agility += 1,
                            StatType::Strength => prep_state.temp_strength += 1,
                            StatType::Dexterity => prep_state.temp_dexterity += 1,
                            StatType::Intelligence => prep_state.temp_intelligence += 1,
                            StatType::Faith => prep_state.temp_faith += 1,
                            StatType::Arcane => prep_state.temp_arcane += 1,
                        }
                        prep_state.status_points -= 1;
                    }
                } else {
                    let current_value = match stat_button.stat_type {
                        StatType::Vitality => prep_state.temp_vitality,
                        StatType::Spirit => prep_state.temp_spirit,
                        StatType::Endurance => prep_state.temp_endurance,
                        StatType::Agility => prep_state.temp_agility,
                        StatType::Strength => prep_state.temp_strength,
                        StatType::Dexterity => prep_state.temp_dexterity,
                        StatType::Intelligence => prep_state.temp_intelligence,
                        StatType::Faith => prep_state.temp_faith,
                        StatType::Arcane => prep_state.temp_arcane,
                    };

                    if current_value > 1 {
                        match stat_button.stat_type {
                            StatType::Vitality => prep_state.temp_vitality -= 1,
                            StatType::Spirit => prep_state.temp_spirit -= 1,
                            StatType::Endurance => prep_state.temp_endurance -= 1,
                            StatType::Agility => prep_state.temp_agility -= 1,
                            StatType::Strength => prep_state.temp_strength -= 1,
                            StatType::Dexterity => prep_state.temp_dexterity -= 1,
                            StatType::Intelligence => prep_state.temp_intelligence -= 1,
                            StatType::Faith => prep_state.temp_faith -= 1,
                            StatType::Arcane => prep_state.temp_arcane -= 1,
                        }
                        prep_state.status_points += 1;
                    }
                }
            }
            Interaction::Hovered => {
                if stat_button.is_increase {
                    *color = BackgroundColor(Color::from(LinearRgba {
                        red: 0.3,
                        green: 0.5,
                        blue: 0.3,
                        alpha: 1.0,
                    }));
                } else {
                    *color = BackgroundColor(Color::from(LinearRgba {
                        red: 0.5,
                        green: 0.3,
                        blue: 0.3,
                        alpha: 1.0,
                    }));
                }
            }
            Interaction::None => {
                if stat_button.is_increase {
                    *color = BackgroundColor(Color::from(LinearRgba {
                        red: 0.2,
                        green: 0.4,
                        blue: 0.2,
                        alpha: 1.0,
                    }));
                } else {
                    *color = BackgroundColor(Color::from(LinearRgba {
                        red: 0.4,
                        green: 0.2,
                        blue: 0.2,
                        alpha: 1.0,
                    }));
                }
            }
        }
    }
}

/// 装備選択システム
pub fn equipment_selection_system(
    mut interaction_query: Query<
        (&Interaction, &EquipmentButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, _equipment_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // TODO: 装備選択ダイアログを開く
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.4,
                    green: 0.4,
                    blue: 0.5,
                    alpha: 1.0,
                }));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.3,
                    green: 0.3,
                    blue: 0.4,
                    alpha: 1.0,
                }));
            }
        }
    }
}

/// 戦闘開始システム
pub fn start_battle_system(
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
                    red: 0.4,
                    green: 0.6,
                    blue: 0.4,
                    alpha: 1.0,
                }));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.3,
                    green: 0.5,
                    blue: 0.3,
                    alpha: 1.0,
                }));
            }
        }
    }
}
