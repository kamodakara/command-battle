use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use super::*;
use crate::types::{
    AbilityScaling, Armor, ArmorDefense, ArmorKind, ArmorResistance, AttackPower, GuardCutRate,
    Weapon, WeaponAbilityRequirement, WeaponAttackPower, WeaponAttackPowerAbilityScaling,
    WeaponBreakPower, WeaponGuard, WeaponKind, WeaponSorceryPower,
};

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
    pub equipped_weapon1: Option<usize>,
    pub equipped_weapon2: Option<usize>,
    pub equipped_armor1: Option<usize>,
    pub equipped_armor2: Option<usize>,
    pub equipped_armor3: Option<usize>,
    pub equipped_armor4: Option<usize>,
    pub equipped_armor5: Option<usize>,
    pub equipped_armor6: Option<usize>,
    pub equipped_armor7: Option<usize>,
    pub equipped_armor8: Option<usize>,
    pub selecting_slot: Option<EquipmentSlot>,
}

// 装備データベース（仮データ）
#[derive(Resource)]
pub struct EquipmentDatabase {
    pub weapons: Vec<WeaponData>,
    pub armors: Vec<ArmorData>,
}

#[derive(Clone)]
pub struct WeaponData {
    pub id: usize,
    pub name: String,
    pub weapon: Weapon,
}

#[derive(Clone)]
pub struct ArmorData {
    pub id: usize,
    pub name: String,
    pub armor: Armor,
}

// ================== Plugin ==================
pub struct PreparationPlugin;

impl Plugin for PreparationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreparationState>()
            .insert_resource(create_equipment_database())
            .add_systems(OnEnter(GameState::Preparation), setup_preparation_screen)
            .add_systems(
                Update,
                (
                    menu_button_system,
                    update_content_panel,
                    status_allocation_system,
                    equipment_selection_system,
                    start_battle_system,
                    equipment_list_button_system,
                    close_equipment_list_system,
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
    prep_state.equipped_weapon1 = None;
    prep_state.equipped_weapon2 = None;
    prep_state.equipped_armor1 = None;
    prep_state.equipped_armor2 = None;
    prep_state.equipped_armor3 = None;
    prep_state.equipped_armor4 = None;
    prep_state.equipped_armor5 = None;
    prep_state.equipped_armor6 = None;
    prep_state.equipped_armor7 = None;
    prep_state.equipped_armor8 = None;
    prep_state.selecting_slot = None;

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
    equipment_db: Res<EquipmentDatabase>,
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
                    build_equipment_content(parent, font_clone.clone(), &prep_state, &equipment_db);
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
fn build_equipment_content(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    prep_state: &PreparationState,
    equipment_db: &EquipmentDatabase,
) {
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
        (
            "右手武器",
            EquipmentSlot::Weapon1,
            prep_state
                .equipped_weapon1
                .map(|id| {
                    equipment_db
                        .weapons
                        .iter()
                        .find(|w| w.id == id)
                        .map(|w| w.name.clone())
                })
                .flatten(),
        ),
        (
            "左手武器",
            EquipmentSlot::Weapon2,
            prep_state
                .equipped_weapon2
                .map(|id| {
                    equipment_db
                        .weapons
                        .iter()
                        .find(|w| w.id == id)
                        .map(|w| w.name.clone())
                })
                .flatten(),
        ),
        (
            "防具1",
            EquipmentSlot::Armor1,
            prep_state
                .equipped_armor1
                .map(|id| {
                    equipment_db
                        .armors
                        .iter()
                        .find(|a| a.id == id)
                        .map(|a| a.name.clone())
                })
                .flatten(),
        ),
        (
            "防具2",
            EquipmentSlot::Armor2,
            prep_state
                .equipped_armor2
                .map(|id| {
                    equipment_db
                        .armors
                        .iter()
                        .find(|a| a.id == id)
                        .map(|a| a.name.clone())
                })
                .flatten(),
        ),
        (
            "防具3",
            EquipmentSlot::Armor3,
            prep_state
                .equipped_armor3
                .map(|id| {
                    equipment_db
                        .armors
                        .iter()
                        .find(|a| a.id == id)
                        .map(|a| a.name.clone())
                })
                .flatten(),
        ),
        (
            "防具4",
            EquipmentSlot::Armor4,
            prep_state
                .equipped_armor4
                .map(|id| {
                    equipment_db
                        .armors
                        .iter()
                        .find(|a| a.id == id)
                        .map(|a| a.name.clone())
                })
                .flatten(),
        ),
        (
            "防具5",
            EquipmentSlot::Armor5,
            prep_state
                .equipped_armor5
                .map(|id| {
                    equipment_db
                        .armors
                        .iter()
                        .find(|a| a.id == id)
                        .map(|a| a.name.clone())
                })
                .flatten(),
        ),
        (
            "防具6",
            EquipmentSlot::Armor6,
            prep_state
                .equipped_armor6
                .map(|id| {
                    equipment_db
                        .armors
                        .iter()
                        .find(|a| a.id == id)
                        .map(|a| a.name.clone())
                })
                .flatten(),
        ),
        (
            "防具7",
            EquipmentSlot::Armor7,
            prep_state
                .equipped_armor7
                .map(|id| {
                    equipment_db
                        .armors
                        .iter()
                        .find(|a| a.id == id)
                        .map(|a| a.name.clone())
                })
                .flatten(),
        ),
        (
            "防具8",
            EquipmentSlot::Armor8,
            prep_state
                .equipped_armor8
                .map(|id| {
                    equipment_db
                        .armors
                        .iter()
                        .find(|a| a.id == id)
                        .map(|a| a.name.clone())
                })
                .flatten(),
        ),
    ];

    for (name, slot, equipped_name) in equipment_slots {
        parent
            .spawn(Node {
                width: Val::Px(600.0),
                height: Val::Px(45.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect::bottom(Val::Px(8.0)),
                padding: UiRect::all(Val::Px(10.0)),
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
                // スロット名と装備名
                row.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(2.0),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Text::new(name),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    if let Some(eq_name) = equipped_name {
                        col.spawn((
                            Text::new(eq_name),
                            TextFont {
                                font: font.clone(),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.9, 1.0)),
                        ));
                    }
                });

                // 変更ボタン
                row.spawn((
                    EquipmentButton { slot },
                    Button,
                    Node {
                        width: Val::Px(90.0),
                        height: Val::Px(35.0),
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
                            font_size: 16.0,
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

                    // 初期値（10）より下がらないようにチェック
                    if current_value > 10 {
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
    mut prep_state: ResMut<PreparationState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    equipment_db: Res<EquipmentDatabase>,
    screen_query: Query<Entity, With<PreparationScreen>>,
) {
    for (interaction, equipment_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                prep_state.selecting_slot = Some(equipment_button.slot);

                // 装備選択ダイアログを表示
                if let Ok(screen_entity) = screen_query.single() {
                    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

                    commands.entity(screen_entity).with_children(|parent| {
                        build_equipment_selection_dialog(
                            parent,
                            font,
                            equipment_button.slot,
                            &equipment_db,
                        );
                    });
                }
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

// ================== 装備データベース生成 ==================

fn create_equipment_database() -> EquipmentDatabase {
    let weapons = vec![
        WeaponData {
            id: 0,
            name: "ロングソード".to_string(),
            weapon: create_longsword(),
        },
        WeaponData {
            id: 1,
            name: "グレートソード".to_string(),
            weapon: create_greatsword(),
        },
        WeaponData {
            id: 2,
            name: "スピア".to_string(),
            weapon: create_spear(),
        },
        WeaponData {
            id: 3,
            name: "バトルアックス".to_string(),
            weapon: create_axe(),
        },
        WeaponData {
            id: 4,
            name: "ラウンドシールド".to_string(),
            weapon: create_shield(),
        },
    ];

    let armors = vec![
        ArmorData {
            id: 0,
            name: "鉄の兜".to_string(),
            armor: create_iron_helmet(),
        },
        ArmorData {
            id: 1,
            name: "鉄の鎧".to_string(),
            armor: create_iron_armor(),
        },
        ArmorData {
            id: 2,
            name: "鉄の籠手".to_string(),
            armor: create_iron_gauntlets(),
        },
        ArmorData {
            id: 3,
            name: "鉄の脚甲".to_string(),
            armor: create_iron_leggings(),
        },
        ArmorData {
            id: 4,
            name: "革の兜".to_string(),
            armor: create_leather_helmet(),
        },
        ArmorData {
            id: 5,
            name: "革の鎧".to_string(),
            armor: create_leather_armor(),
        },
        ArmorData {
            id: 6,
            name: "革の籠手".to_string(),
            armor: create_leather_gauntlets(),
        },
        ArmorData {
            id: 7,
            name: "革の脚甲".to_string(),
            armor: create_leather_leggings(),
        },
    ];

    EquipmentDatabase { weapons, armors }
}

// 武器生成関数（簡略化のため一部のみ実装）
fn create_longsword() -> Weapon {
    Weapon {
        kind: WeaponKind::StraightSword,
        weight: 10,
        ability_requirement: WeaponAbilityRequirement {
            strength: 10,
            dexterity: 10,
            intelligence: 0,
            faith: 0,
            arcane: 0,
            agility: 0,
        },
        attack_power: WeaponAttackPower {
            base: AttackPower {
                slash: 100,
                strike: 0,
                thrust: 50,
                impact: 0,
                magic: 0,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            ability_scaling: create_default_weapon_scaling(),
        },
        sorcery_power: create_default_sorcery_power(),
        break_power: WeaponBreakPower {
            base_power: 20,
            scaling: create_default_ability_scaling(),
        },
        guard: create_default_guard(),
    }
}

fn create_greatsword() -> Weapon {
    Weapon {
        kind: WeaponKind::Greatsword,
        weight: 20,
        ability_requirement: WeaponAbilityRequirement {
            strength: 20,
            dexterity: 10,
            intelligence: 0,
            faith: 0,
            arcane: 0,
            agility: 0,
        },
        attack_power: WeaponAttackPower {
            base: AttackPower {
                slash: 150,
                strike: 50,
                thrust: 80,
                impact: 0,
                magic: 0,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            ability_scaling: create_default_weapon_scaling(),
        },
        sorcery_power: create_default_sorcery_power(),
        break_power: WeaponBreakPower {
            base_power: 40,
            scaling: create_default_ability_scaling(),
        },
        guard: create_default_guard(),
    }
}

fn create_spear() -> Weapon {
    Weapon {
        kind: WeaponKind::Spear,
        weight: 12,
        ability_requirement: WeaponAbilityRequirement {
            strength: 12,
            dexterity: 15,
            intelligence: 0,
            faith: 0,
            arcane: 0,
            agility: 0,
        },
        attack_power: WeaponAttackPower {
            base: AttackPower {
                slash: 30,
                strike: 0,
                thrust: 120,
                impact: 0,
                magic: 0,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            ability_scaling: create_default_weapon_scaling(),
        },
        sorcery_power: create_default_sorcery_power(),
        break_power: WeaponBreakPower {
            base_power: 25,
            scaling: create_default_ability_scaling(),
        },
        guard: create_default_guard(),
    }
}

fn create_axe() -> Weapon {
    Weapon {
        kind: WeaponKind::Axe,
        weight: 18,
        ability_requirement: WeaponAbilityRequirement {
            strength: 18,
            dexterity: 8,
            intelligence: 0,
            faith: 0,
            arcane: 0,
            agility: 0,
        },
        attack_power: WeaponAttackPower {
            base: AttackPower {
                slash: 130,
                strike: 40,
                thrust: 20,
                impact: 0,
                magic: 0,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            ability_scaling: create_default_weapon_scaling(),
        },
        sorcery_power: create_default_sorcery_power(),
        break_power: WeaponBreakPower {
            base_power: 35,
            scaling: create_default_ability_scaling(),
        },
        guard: create_default_guard(),
    }
}

fn create_shield() -> Weapon {
    Weapon {
        kind: WeaponKind::Shield,
        weight: 8,
        ability_requirement: WeaponAbilityRequirement {
            strength: 10,
            dexterity: 0,
            intelligence: 0,
            faith: 0,
            arcane: 0,
            agility: 0,
        },
        attack_power: WeaponAttackPower {
            base: AttackPower {
                slash: 10,
                strike: 30,
                thrust: 0,
                impact: 0,
                magic: 0,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            ability_scaling: create_default_weapon_scaling(),
        },
        sorcery_power: create_default_sorcery_power(),
        break_power: WeaponBreakPower {
            base_power: 10,
            scaling: create_default_ability_scaling(),
        },
        guard: WeaponGuard {
            cut_rate: GuardCutRate {
                slash: 0.1,
                strike: 0.1,
                thrust: 0.1,
                impact: 0.3,
                magic: 0.5,
                fire: 0.5,
                lightning: 0.5,
                chaos: 0.5,
            },
            guard_strength: 60,
        },
    }
}

// 防具生成関数
fn create_iron_helmet() -> Armor {
    Armor {
        kind: ArmorKind::Helmet,
        weight: 8,
        defense: ArmorDefense {
            slash: 15,
            strike: 15,
            thrust: 15,
            impact: 12,
            magic: 8,
            fire: 10,
            lightning: 10,
            chaos: 8,
        },
        resistance: ArmorResistance {
            immunity: 10,
            robustness: 12,
            sanity: 8,
        },
    }
}

fn create_iron_armor() -> Armor {
    Armor {
        kind: ArmorKind::ChestArmor,
        weight: 20,
        defense: ArmorDefense {
            slash: 30,
            strike: 30,
            thrust: 30,
            impact: 25,
            magic: 15,
            fire: 20,
            lightning: 20,
            chaos: 15,
        },
        resistance: ArmorResistance {
            immunity: 20,
            robustness: 25,
            sanity: 15,
        },
    }
}

fn create_iron_gauntlets() -> Armor {
    Armor {
        kind: ArmorKind::Gauntlets,
        weight: 6,
        defense: ArmorDefense {
            slash: 10,
            strike: 10,
            thrust: 10,
            impact: 8,
            magic: 5,
            fire: 7,
            lightning: 7,
            chaos: 5,
        },
        resistance: ArmorResistance {
            immunity: 7,
            robustness: 8,
            sanity: 5,
        },
    }
}

fn create_iron_leggings() -> Armor {
    Armor {
        kind: ArmorKind::LegArmor,
        weight: 12,
        defense: ArmorDefense {
            slash: 18,
            strike: 18,
            thrust: 18,
            impact: 15,
            magic: 10,
            fire: 12,
            lightning: 12,
            chaos: 10,
        },
        resistance: ArmorResistance {
            immunity: 12,
            robustness: 15,
            sanity: 10,
        },
    }
}

fn create_leather_helmet() -> Armor {
    Armor {
        kind: ArmorKind::Helmet,
        weight: 3,
        defense: ArmorDefense {
            slash: 8,
            strike: 7,
            thrust: 8,
            impact: 6,
            magic: 5,
            fire: 6,
            lightning: 5,
            chaos: 5,
        },
        resistance: ArmorResistance {
            immunity: 8,
            robustness: 7,
            sanity: 10,
        },
    }
}

fn create_leather_armor() -> Armor {
    Armor {
        kind: ArmorKind::ChestArmor,
        weight: 8,
        defense: ArmorDefense {
            slash: 15,
            strike: 12,
            thrust: 15,
            impact: 10,
            magic: 10,
            fire: 12,
            lightning: 10,
            chaos: 10,
        },
        resistance: ArmorResistance {
            immunity: 15,
            robustness: 12,
            sanity: 20,
        },
    }
}

fn create_leather_gauntlets() -> Armor {
    Armor {
        kind: ArmorKind::Gauntlets,
        weight: 2,
        defense: ArmorDefense {
            slash: 5,
            strike: 4,
            thrust: 5,
            impact: 3,
            magic: 4,
            fire: 4,
            lightning: 4,
            chaos: 4,
        },
        resistance: ArmorResistance {
            immunity: 5,
            robustness: 4,
            sanity: 7,
        },
    }
}

fn create_leather_leggings() -> Armor {
    Armor {
        kind: ArmorKind::LegArmor,
        weight: 5,
        defense: ArmorDefense {
            slash: 10,
            strike: 8,
            thrust: 10,
            impact: 7,
            magic: 7,
            fire: 8,
            lightning: 7,
            chaos: 7,
        },
        resistance: ArmorResistance {
            immunity: 10,
            robustness: 8,
            sanity: 15,
        },
    }
}

// ヘルパー関数
fn create_default_ability_scaling() -> AbilityScaling {
    AbilityScaling {
        strength: 0.0,
        dexterity: 0.0,
        intelligence: 0.0,
        faith: 0.0,
        arcane: 0.0,
        agility: 0.0,
    }
}

fn create_default_weapon_scaling() -> WeaponAttackPowerAbilityScaling {
    WeaponAttackPowerAbilityScaling {
        slash: create_default_ability_scaling(),
        strike: create_default_ability_scaling(),
        thrust: create_default_ability_scaling(),
        impact: create_default_ability_scaling(),
        magic: create_default_ability_scaling(),
        fire: create_default_ability_scaling(),
        lightning: create_default_ability_scaling(),
        chaos: create_default_ability_scaling(),
    }
}

fn create_default_sorcery_power() -> WeaponSorceryPower {
    WeaponSorceryPower {
        base: 0,
        scaling: create_default_ability_scaling(),
    }
}

fn create_default_guard() -> WeaponGuard {
    WeaponGuard {
        cut_rate: GuardCutRate {
            slash: 0.8,
            strike: 0.8,
            thrust: 0.8,
            impact: 0.8,
            magic: 1.0,
            fire: 1.0,
            lightning: 1.0,
            chaos: 1.0,
        },
        guard_strength: 10,
    }
}

// ================== 装備選択ダイアログ ==================

#[derive(Component)]
struct EquipmentSelectionDialog;

#[derive(Component)]
struct EquipmentListButton {
    equipment_id: usize,
}

#[derive(Component)]
struct CloseDialogButton;

fn build_equipment_selection_dialog(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    slot: EquipmentSlot,
    equipment_db: &EquipmentDatabase,
) {
    parent
        .spawn((
            EquipmentSelectionDialog,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::from(LinearRgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.7,
            })),
            ZIndex(100),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: Val::Px(600.0),
                        height: Val::Px(500.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(15.0),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(Color::from(LinearRgba {
                        red: 0.15,
                        green: 0.15,
                        blue: 0.2,
                        alpha: 1.0,
                    })),
                    BorderColor::all(Color::WHITE),
                ))
                .with_children(|dialog| {
                    // ヘッダー
                    dialog
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        })
                        .with_children(|header| {
                            header.spawn((
                                Text::new("装備を選択"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 28.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));

                            // 閉じるボタン
                            header
                                .spawn((
                                    CloseDialogButton,
                                    Button,
                                    Node {
                                        width: Val::Px(80.0),
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
                                        Text::new("閉じる"),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });
                        });

                    // 装備リスト
                    dialog
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            overflow: Overflow::scroll_y(),
                            ..default()
                        })
                        .with_children(|list| {
                            match slot {
                                EquipmentSlot::Weapon1 | EquipmentSlot::Weapon2 => {
                                    // 武器リスト
                                    for weapon_data in &equipment_db.weapons {
                                        list.spawn((
                                            EquipmentListButton {
                                                equipment_id: weapon_data.id,
                                            },
                                            Button,
                                            Node {
                                                width: Val::Percent(100.0),
                                                height: Val::Px(50.0),
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
                                        .with_children(
                                            |btn| {
                                                btn.spawn((
                                                    Text::new(&weapon_data.name),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 20.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::WHITE),
                                                ));
                                            },
                                        );
                                    }
                                }
                                _ => {
                                    // 防具リスト
                                    for armor_data in &equipment_db.armors {
                                        list.spawn((
                                            EquipmentListButton {
                                                equipment_id: armor_data.id,
                                            },
                                            Button,
                                            Node {
                                                width: Val::Percent(100.0),
                                                height: Val::Px(50.0),
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
                                        .with_children(
                                            |btn| {
                                                btn.spawn((
                                                    Text::new(&armor_data.name),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 20.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::WHITE),
                                                ));
                                            },
                                        );
                                    }
                                }
                            }
                        });
                });
        });
}

/// 装備リストボタンのインタラクション
pub fn equipment_list_button_system(
    mut interaction_query: Query<
        (&Interaction, &EquipmentListButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut prep_state: ResMut<PreparationState>,
    mut commands: Commands,
    dialog_query: Query<Entity, With<EquipmentSelectionDialog>>,
) {
    for (interaction, list_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(slot) = prep_state.selecting_slot {
                    match slot {
                        EquipmentSlot::Weapon1 => {
                            prep_state.equipped_weapon1 = Some(list_button.equipment_id);
                        }
                        EquipmentSlot::Weapon2 => {
                            prep_state.equipped_weapon2 = Some(list_button.equipment_id);
                        }
                        EquipmentSlot::Armor1 => {
                            prep_state.equipped_armor1 = Some(list_button.equipment_id);
                        }
                        EquipmentSlot::Armor2 => {
                            prep_state.equipped_armor2 = Some(list_button.equipment_id);
                        }
                        EquipmentSlot::Armor3 => {
                            prep_state.equipped_armor3 = Some(list_button.equipment_id);
                        }
                        EquipmentSlot::Armor4 => {
                            prep_state.equipped_armor4 = Some(list_button.equipment_id);
                        }
                        EquipmentSlot::Armor5 => {
                            prep_state.equipped_armor5 = Some(list_button.equipment_id);
                        }
                        EquipmentSlot::Armor6 => {
                            prep_state.equipped_armor6 = Some(list_button.equipment_id);
                        }
                        EquipmentSlot::Armor7 => {
                            prep_state.equipped_armor7 = Some(list_button.equipment_id);
                        }
                        EquipmentSlot::Armor8 => {
                            prep_state.equipped_armor8 = Some(list_button.equipment_id);
                        }
                    }
                    prep_state.selecting_slot = None;
                }

                // ダイアログを閉じる
                for entity in dialog_query.iter() {
                    commands.entity(entity).despawn();
                }
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

/// ダイアログを閉じるシステム
pub fn close_equipment_list_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CloseDialogButton>),
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<EquipmentSelectionDialog>>,
    mut prep_state: ResMut<PreparationState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                prep_state.selecting_slot = None;
                for entity in dialog_query.iter() {
                    commands.entity(entity).despawn();
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.5,
                    green: 0.3,
                    blue: 0.3,
                    alpha: 1.0,
                }));
            }
            Interaction::None => {
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
