use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use super::*;
use crate::fundamental::{
    Ability, AbilityScaling, AbilityType, Armor, ArmorKind, ArmorResistance, ArmorSlot, Art,
    ArtPerk, ArtPotency, ArtPotencyAttack, ArtPotencySupport, ArtPotencySupportRecover,
    ArtPotencySupportStatusCondition, ArtRank, ArtRequirement, ArtTarget, ArtType, ArtUsableWeapon,
    AttackPower, AttackPowerScaling, DefensePower, Equipment, EquipmentLoadPerformanceStatus,
    GuardCutRate, StatusCondition, StatusConditionDuration, StatusConditionDurationTurn,
    StatusConditionPotency, StatusConditionResistance, SupportRecoverPotency,
    SupportRecoverPotencyHp, SupportRecoverPotencySp, Weapon, WeaponAbilityRequirement,
    WeaponAttackPower, WeaponAttackPowerAbilityScaling, WeaponBreakPower, WeaponGuard, WeaponKind,
    WeaponPerformance, WeaponSorceryPower,
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

#[derive(Component)]
pub struct ErrorMessageText;

// ================== Resources ==================

#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub enum MenuType {
    #[default]
    Status,
    Equipment,
    Arts,
    StartBattle,
}

#[derive(Resource, Default)]
pub struct PreparationState {
    pub initialized: bool, // 初期化済みフラグ（戦闘から戻った際にリセットしない）
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
    pub selected_arts: Vec<usize>, // 選択された技術のID (最大8つ)
    pub selecting_arts_slot: Option<usize>, // 選択中のスロット (0-7)
    pub selected_art_tab: ArtTypeTab, // 技術選択ダイアログの選択中タブ
    pub error_message: Option<String>,
    pub error_message_timer: f32,
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

// 技術データベース（仮データ）
#[derive(Resource)]
pub struct ArtsDatabase {
    pub arts: Vec<ArtsData>,
}

#[derive(Clone)]
pub struct ArtsData {
    pub id: usize,
    pub name: String,
    pub art: Art,
}

// ================== Plugin ==================
pub struct PreparationPlugin;

impl Plugin for PreparationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreparationState>()
            .insert_resource(create_equipment_database())
            .insert_resource(create_arts_database())
            .add_systems(OnEnter(GameState::Preparation), setup_preparation_screen)
            .add_systems(
                Update,
                (
                    menu_button_system,
                    update_content_panel,
                    status_allocation_system,
                    equipment_selection_system,
                    unequip_system,
                    arts_slot_button_system,
                    start_battle_system,
                    equipment_list_button_system,
                    arts_selection_dialog_system,
                    close_equipment_list_system,
                    close_arts_selection_dialog_system,
                    arts_tab_button_system,
                    update_error_message_system,
                    display_error_message_system,
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

    // 初期化（初回のみ。戦闘から戻った場合は能力・装備・技術を保持）
    if !prep_state.initialized {
        prep_state.initialized = true;
        prep_state.status_points = 100;
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
    }
    // UI状態は毎回リセット
    prep_state.current_menu = MenuType::Status;
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

                    // 技術ボタン
                    menu.spawn((
                        MenuButton {
                            menu_type: MenuType::Arts,
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
                            Text::new("技術"),
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
    arts_db: Res<ArtsDatabase>,
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
                    build_status_content(parent, font_clone.clone(), &prep_state, &equipment_db);
                }
                MenuType::Equipment => {
                    build_equipment_content(parent, font_clone.clone(), &prep_state, &equipment_db);
                }
                MenuType::Arts => {
                    build_arts_content(parent, font_clone.clone(), &prep_state, &arts_db);
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
    equipment_db: &EquipmentDatabase,
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

    // 現在の能力値からPlayerAbilityを作成
    let current_ability = Ability {
        vitality: prep_state.temp_vitality,
        spirit: prep_state.temp_spirit,
        endurance: prep_state.temp_endurance,
        agility: prep_state.temp_agility,
        strength: prep_state.temp_strength,
        dexterity: prep_state.temp_dexterity,
        intelligence: prep_state.temp_intelligence,
        faith: prep_state.temp_faith,
        arcane: prep_state.temp_arcane,
    };

    // プレイヤーステータスを計算
    let player_stats = current_ability.player_stats();

    // 基礎防御力を計算
    let defense_power = current_ability.base_defense_power();

    // 横並びのコンテナ（左：能力値、右：ステータスと防御力）
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(40.0),
            ..default()
        })
        .with_children(|row| {
            // 左側：能力値割り振り
            row.spawn(Node {
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|left_col| {
                // 能力値セクションのヘッダー
                left_col.spawn((
                    Text::new("■ 能力値"),
                    TextFont {
                        font: font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 1.0, 0.8)),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
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
                    left_col
                        .spawn(Node {
                            width: Val::Px(350.0),
                            height: Val::Px(45.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(15.0),
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        })
                        .with_children(|stat_row| {
                            // ステータス名
                            stat_row.spawn((
                                Text::new(name),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    width: Val::Px(100.0),
                                    ..default()
                                },
                            ));

                            // 減少ボタン
                            stat_row
                                .spawn((
                                    StatButton {
                                        stat_type,
                                        is_increase: false,
                                    },
                                    Button,
                                    Node {
                                        width: Val::Px(35.0),
                                        height: Val::Px(35.0),
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
                                            font_size: 20.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });

                            // 値表示
                            stat_row.spawn((
                                Text::new(format!("{}", value)),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    width: Val::Px(50.0),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                            ));

                            // 増加ボタン
                            stat_row
                                .spawn((
                                    StatButton {
                                        stat_type,
                                        is_increase: true,
                                    },
                                    Button,
                                    Node {
                                        width: Val::Px(35.0),
                                        height: Val::Px(35.0),
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
                                            font_size: 20.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });
                        });
                }
            });

            // 右側：プレイヤーステータスと基礎防御力
            row.spawn(Node {
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|right_col| {
                // プレイヤーステータス表示
                right_col.spawn((
                    Text::new("■ プレイヤーステータス"),
                    TextFont {
                        font: font.clone(),
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 1.0, 0.8)),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));

                let player_stats_info = [
                    ("HP", player_stats.hp),
                    ("SP", player_stats.sp),
                    ("スタミナ", player_stats.stamina),
                    ("スタミナ回復", player_stats.stamina_recovery),
                    ("装備重量", player_stats.max_equipment_weight),
                ];

                for (name, value) in player_stats_info {
                    right_col.spawn((
                        Text::new(format!("  {}: {}", name, value)),
                        TextFont {
                            font: font.clone(),
                            font_size: 17.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                    ));
                }

                // 基礎防御力表示
                right_col.spawn((
                    Text::new("■ 基礎防御力"),
                    TextFont {
                        font: font.clone(),
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 1.0, 0.8)),
                    Node {
                        margin: UiRect {
                            top: Val::Px(15.0),
                            bottom: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    },
                ));

                let defense_info = [
                    ("斬撃", defense_power.slash),
                    ("打撃", defense_power.strike),
                    ("刺突", defense_power.thrust),
                    ("衝撃", defense_power.impact),
                    ("魔力", defense_power.magic),
                    ("炎", defense_power.fire),
                    ("雷", defense_power.lightning),
                    ("混濁", defense_power.chaos),
                ];

                for (name, value) in defense_info {
                    right_col.spawn((
                        Text::new(format!("  {}: {}", name, value)),
                        TextFont {
                            font: font.clone(),
                            font_size: 17.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                    ));
                }
            });

            // 右端：攻撃力と術力
            row.spawn(Node {
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|attack_col| {
                // 攻撃力表示
                attack_col.spawn((
                    Text::new("■ 攻撃力"),
                    TextFont {
                        font: font.clone(),
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 1.0, 0.8)),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));

                // 武器1の攻撃力を計算
                let weapon1_attack_power = if let Some(weapon_id) = prep_state.equipped_weapon1 {
                    if let Some(weapon_data) = equipment_db.weapons.get(weapon_id) {
                        let weapon_performance = weapon_data.weapon.performance(&current_ability);
                        println!(
                            "Weapon 1 Attack Power Calculation: {:?}",
                            weapon_performance.final_attack_power()
                        );
                        weapon_performance.final_attack_power().total_power()
                    } else {
                        WeaponPerformance::unarmed_weapon_performance()
                            .final_attack_power()
                            .total_power()
                    }
                } else {
                    WeaponPerformance::unarmed_weapon_performance()
                        .final_attack_power()
                        .total_power()
                };

                // 武器2の攻撃力を計算
                let weapon2_attack_power = if let Some(weapon_id) = prep_state.equipped_weapon2 {
                    if let Some(weapon_data) = equipment_db.weapons.get(weapon_id) {
                        let weapon_performance = weapon_data.weapon.performance(&current_ability);
                        weapon_performance.final_attack_power().total_power()
                    } else {
                        WeaponPerformance::unarmed_weapon_performance()
                            .final_attack_power()
                            .total_power()
                    }
                } else {
                    WeaponPerformance::unarmed_weapon_performance()
                        .final_attack_power()
                        .total_power()
                };

                attack_col.spawn((
                    Text::new(format!("  武器1: {}", weapon1_attack_power)),
                    TextFont {
                        font: font.clone(),
                        font_size: 17.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ));

                attack_col.spawn((
                    Text::new(format!("  武器2: {}", weapon2_attack_power)),
                    TextFont {
                        font: font.clone(),
                        font_size: 17.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ));

                // 術力表示
                attack_col.spawn((
                    Text::new("■ 術力"),
                    TextFont {
                        font: font.clone(),
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 1.0, 0.8)),
                    Node {
                        margin: UiRect {
                            top: Val::Px(15.0),
                            bottom: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    },
                ));

                // 武器1の術力を計算
                let weapon1_sorcery_power = if let Some(weapon_id) = prep_state.equipped_weapon1 {
                    if let Some(weapon_data) = equipment_db.weapons.get(weapon_id) {
                        let weapon_performance = weapon_data.weapon.performance(&current_ability);
                        weapon_performance.final_sorcery_power()
                    } else {
                        WeaponPerformance::unarmed_weapon_performance().final_sorcery_power()
                    }
                } else {
                    WeaponPerformance::unarmed_weapon_performance().final_sorcery_power()
                };

                // 武器2の術力を計算
                let weapon2_sorcery_power = if let Some(weapon_id) = prep_state.equipped_weapon2 {
                    if let Some(weapon_data) = equipment_db.weapons.get(weapon_id) {
                        let weapon_performance = weapon_data.weapon.performance(&current_ability);
                        weapon_performance.final_sorcery_power()
                    } else {
                        WeaponPerformance::unarmed_weapon_performance().final_sorcery_power()
                    }
                } else {
                    WeaponPerformance::unarmed_weapon_performance().final_sorcery_power()
                };

                attack_col.spawn((
                    Text::new(format!("  武器1: {}", weapon1_sorcery_power)),
                    TextFont {
                        font: font.clone(),
                        font_size: 17.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ));

                attack_col.spawn((
                    Text::new(format!("  武器2: {}", weapon2_sorcery_power)),
                    TextFont {
                        font: font.clone(),
                        font_size: 17.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ));
            });
        });
}

/// 武器種類を日本語に変換
fn weapon_kind_to_japanese(kind: &WeaponKind) -> &'static str {
    match kind {
        WeaponKind::StraightSword => "直剣",
        WeaponKind::Greatsword => "大剣",
        WeaponKind::Spear => "槍",
        WeaponKind::Axe => "斧",
        WeaponKind::Hammer => "ハンマー",
        WeaponKind::Bow => "弓",
        WeaponKind::Crossbow => "クロスボウ",
        WeaponKind::Staff => "杖",
        WeaponKind::Shield => "盾",
    }
}

/// 能力タイプを日本語に変換
fn ability_type_to_japanese(ability_type: &AbilityType) -> &'static str {
    match ability_type {
        AbilityType::Vitality => "生命力",
        AbilityType::Spirit => "精神力",
        AbilityType::Endurance => "持久力",
        AbilityType::Agility => "敏捷性",
        AbilityType::Strength => "筋力",
        AbilityType::Dexterity => "技量",
        AbilityType::Intelligence => "知力",
        AbilityType::Faith => "信仰",
        AbilityType::Arcane => "神秘",
    }
}

/// 攻撃力の値をフォーマットする（基礎値 + 能力補正、ペナルティがある場合は別表示）
fn format_attack_power_value(base: u32, ability_bonus: u32, penalty: Option<u32>) -> String {
    if let Some(penalty_value) = penalty {
        // ペナルティがある場合
        format!("{} (-{})", base, penalty_value)
    } else if ability_bonus > 0 {
        // 能力補正がある場合
        format!("{} (+{})", base, ability_bonus)
    } else {
        // 基礎値のみ
        format!("{}", base)
    }
}

/// 武器性能を表示するUI構築ヘルパー
fn build_weapon_performance_display(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    title: &str,
    weapon: &Weapon,
    performance: &WeaponPerformance,
) {
    // タイトル（右手武器/左手武器）
    parent.spawn((
        Text::new(format!("■ {}性能", title)),
        TextFont {
            font: font.clone(),
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 1.0, 0.8)),
        Node {
            margin: UiRect::bottom(Val::Px(5.0)),
            ..default()
        },
    ));

    // 武器名
    parent.spawn((
        Text::new(format!("  {}", weapon.name)),
        TextFont {
            font: font.clone(),
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.5)),
        Node {
            margin: UiRect::bottom(Val::Px(3.0)),
            ..default()
        },
    ));

    // 基本情報（重量、武器種）
    parent.spawn((
        Text::new(format!(
            "  重量: {}  武器種: {}",
            weapon.weight,
            weapon_kind_to_japanese(&weapon.kind)
        )),
        TextFont {
            font: font.clone(),
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            margin: UiRect::bottom(Val::Px(2.0)),
            ..default()
        },
    ));

    // 必要能力の表示
    let mut req_parts = vec![];
    if weapon.ability_requirement.strength > 0 {
        req_parts.push(format!("筋力{}", weapon.ability_requirement.strength));
    }
    if weapon.ability_requirement.dexterity > 0 {
        req_parts.push(format!("技量{}", weapon.ability_requirement.dexterity));
    }
    if weapon.ability_requirement.intelligence > 0 {
        req_parts.push(format!("知力{}", weapon.ability_requirement.intelligence));
    }
    if weapon.ability_requirement.faith > 0 {
        req_parts.push(format!("信仰{}", weapon.ability_requirement.faith));
    }
    if weapon.ability_requirement.arcane > 0 {
        req_parts.push(format!("神秘{}", weapon.ability_requirement.arcane));
    }
    if weapon.ability_requirement.agility > 0 {
        req_parts.push(format!("敏捷{}", weapon.ability_requirement.agility));
    }
    let req_text = if req_parts.is_empty() {
        "なし".to_string()
    } else {
        req_parts.join(" ")
    };

    // ペナルティがある場合、不足能力を赤で表示
    let (req_color, req_suffix) = if let Some(ref penalty) = performance.penalty {
        let not_enough: Vec<String> = penalty
            .not_enough_abilities
            .iter()
            .map(|a| ability_type_to_japanese(a).to_string())
            .collect();
        (
            Color::srgb(1.0, 0.4, 0.4),
            format!(" [不足: {}]", not_enough.join(", ")),
        )
    } else {
        (Color::WHITE, String::new())
    };

    parent.spawn((
        Text::new(format!("  必要能力: {}{}", req_text, req_suffix)),
        TextFont {
            font: font.clone(),
            font_size: 14.0,
            ..default()
        },
        TextColor(req_color),
        Node {
            margin: UiRect::bottom(Val::Px(3.0)),
            ..default()
        },
    ));

    // 攻撃力セクション
    parent.spawn((
        Text::new("  【攻撃力・術力】"),
        TextFont {
            font: font.clone(),
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.5)),
        Node {
            margin: UiRect::bottom(Val::Px(1.0)),
            ..default()
        },
    ));

    // 各属性攻撃力
    let attack_info = [
        (
            "斬撃",
            performance.attack_power.slash,
            performance.ability_attack_power.slash,
        ),
        (
            "打撃",
            performance.attack_power.strike,
            performance.ability_attack_power.strike,
        ),
        (
            "刺突",
            performance.attack_power.thrust,
            performance.ability_attack_power.thrust,
        ),
        (
            "衝撃",
            performance.attack_power.impact,
            performance.ability_attack_power.impact,
        ),
        (
            "魔力",
            performance.attack_power.magic,
            performance.ability_attack_power.magic,
        ),
        (
            "炎",
            performance.attack_power.fire,
            performance.ability_attack_power.fire,
        ),
        (
            "雷",
            performance.attack_power.lightning,
            performance.ability_attack_power.lightning,
        ),
        (
            "混濁",
            performance.attack_power.chaos,
            performance.ability_attack_power.chaos,
        ),
    ];

    // 攻撃力がある属性のみ表示
    let mut attack_texts = vec![];
    for (name, base, ability_bonus) in attack_info {
        if base > 0 || ability_bonus > 0 {
            let penalty_value = performance
                .penalty
                .as_ref()
                .map(|p| match name {
                    "斬撃" => p.penalty_attack_power.slash,
                    "打撃" => p.penalty_attack_power.strike,
                    "刺突" => p.penalty_attack_power.thrust,
                    "衝撃" => p.penalty_attack_power.impact,
                    "魔力" => p.penalty_attack_power.magic,
                    "炎" => p.penalty_attack_power.fire,
                    "雷" => p.penalty_attack_power.lightning,
                    "混濁" => p.penalty_attack_power.chaos,
                    _ => 0,
                })
                .filter(|&v| v > 0);

            attack_texts.push(format!(
                "{}: {}",
                name,
                format_attack_power_value(base, ability_bonus, penalty_value)
            ));
        }
    }

    if !attack_texts.is_empty() {
        parent.spawn((
            Text::new(format!("    {}", attack_texts.join("  "))),
            TextFont {
                font: font.clone(),
                font_size: 13.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
        ));
    }

    // 術力（攻撃力セクション内）
    let sorcery_text = if let Some(ref penalty) = performance.penalty {
        if penalty.penalty_sorcery_power > 0 {
            format!(
                "術力: {} (-{})",
                performance.sorcery_power, penalty.penalty_sorcery_power
            )
        } else if performance.ability_sorcery_power > 0 {
            format!(
                "術力: {} (+{})",
                performance.sorcery_power, performance.ability_sorcery_power
            )
        } else {
            format!("術力: {}", performance.sorcery_power)
        }
    } else if performance.ability_sorcery_power > 0 {
        format!(
            "術力: {} (+{})",
            performance.sorcery_power, performance.ability_sorcery_power
        )
    } else {
        format!("術力: {}", performance.sorcery_power)
    };

    parent.spawn((
        Text::new(format!("    {}", sorcery_text)),
        TextFont {
            font: font.clone(),
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            margin: UiRect::bottom(Val::Px(3.0)),
            ..default()
        },
    ));

    // ガード性能セクション
    parent.spawn((
        Text::new("  【ガード性能】"),
        TextFont {
            font: font.clone(),
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.8, 1.0)),
        Node {
            margin: UiRect::bottom(Val::Px(1.0)),
            ..default()
        },
    ));

    // ガード強度
    let guard_strength_text = if let Some(ref penalty) = performance.penalty {
        if penalty.penalty_guard_strength > 0 {
            format!(
                "ガード強度: {} (-{})",
                weapon.guard.guard_strength, penalty.penalty_guard_strength
            )
        } else {
            format!("ガード強度: {}", weapon.guard.guard_strength)
        }
    } else {
        format!("ガード強度: {}", weapon.guard.guard_strength)
    };

    parent.spawn((
        Text::new(format!("    {}", guard_strength_text)),
        TextFont {
            font: font.clone(),
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            margin: UiRect::bottom(Val::Px(2.0)),
            ..default()
        },
    ));

    // カット率
    let cut_rate = &weapon.guard.cut_rate;

    // カット率ラベル
    parent.spawn((
        Text::new("    カット率:"),
        TextFont {
            font: font.clone(),
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            margin: UiRect::bottom(Val::Px(1.0)),
            ..default()
        },
    ));

    // カット率1行目（物理系：斬、打、刺、衝）
    let cut_rate_line1 = format!(
        "      斬: {:.0}%  打: {:.0}%  刺: {:.0}%  衝: {:.0}%",
        (1.0 - cut_rate.slash) * 100.0,
        (1.0 - cut_rate.strike) * 100.0,
        (1.0 - cut_rate.thrust) * 100.0,
        (1.0 - cut_rate.impact) * 100.0
    );

    parent.spawn((
        Text::new(cut_rate_line1),
        TextFont {
            font: font.clone(),
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            margin: UiRect::bottom(Val::Px(1.0)),
            ..default()
        },
    ));

    // カット率2行目（属性系：魔、炎、雷、濁）
    let cut_rate_line2 = format!(
        "      魔: {:.0}%  炎: {:.0}%  雷: {:.0}%  濁: {:.0}%",
        (1.0 - cut_rate.magic) * 100.0,
        (1.0 - cut_rate.fire) * 100.0,
        (1.0 - cut_rate.lightning) * 100.0,
        (1.0 - cut_rate.chaos) * 100.0
    );

    parent.spawn((
        Text::new(cut_rate_line2),
        TextFont {
            font: font.clone(),
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            margin: UiRect::bottom(Val::Px(3.0)),
            ..default()
        },
    ));
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

    // 現在のプレイヤー能力値を作成
    let current_ability = Ability {
        vitality: prep_state.temp_vitality,
        spirit: prep_state.temp_spirit,
        endurance: prep_state.temp_endurance,
        agility: prep_state.temp_agility,
        strength: prep_state.temp_strength,
        dexterity: prep_state.temp_dexterity,
        intelligence: prep_state.temp_intelligence,
        faith: prep_state.temp_faith,
        arcane: prep_state.temp_arcane,
    };

    // 装備スロット一覧（武器の場合は武器データも保持）
    let weapon1_data = prep_state
        .equipped_weapon1
        .and_then(|id| equipment_db.weapons.iter().find(|w| w.id == id));
    let weapon2_data = prep_state
        .equipped_weapon2
        .and_then(|id| equipment_db.weapons.iter().find(|w| w.id == id));

    let equipment_slots: Vec<(&str, EquipmentSlot, Option<String>, Option<&WeaponData>)> = vec![
        (
            "右手武器",
            EquipmentSlot::Weapon1,
            weapon1_data.map(|w| w.name.clone()),
            weapon1_data,
        ),
        (
            "左手武器",
            EquipmentSlot::Weapon2,
            weapon2_data.map(|w| w.name.clone()),
            weapon2_data,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
        ),
    ];

    // メインコンテナ：装備スロット（左）と武器性能（右）を横並びで表示
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(30.0),
            ..default()
        })
        .with_children(|main_row| {
            // 左側：装備スロット一覧
            main_row
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(380.0),
                    ..default()
                })
                .with_children(|left_col| {
                    for (name, slot, equipped_name, weapon_data) in equipment_slots {
                        left_col
                            .spawn(Node {
                                width: Val::Px(370.0),
                                height: Val::Px(40.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                margin: UiRect::bottom(Val::Px(5.0)),
                                padding: UiRect::all(Val::Px(8.0)),
                                border: UiRect::all(Val::Px(1.0)),
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
                                // 装備があるかどうかを先に確認
                                let has_equipment = equipped_name.is_some();

                                // スロット名と装備名
                                row.spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(1.0),
                                    ..default()
                                })
                                .with_children(|col| {
                                    col.spawn((
                                        Text::new(name),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));

                                    if let Some(eq_name) = equipped_name {
                                        // 武器の場合、使用可能かチェック
                                        let is_usable = if let Some(weapon_data) = weapon_data {
                                            let not_enough_abilities = weapon_data
                                                .weapon
                                                .not_enough_abilities(&current_ability);
                                            not_enough_abilities.is_empty()
                                        } else {
                                            true // 防具の場合は常にtrue
                                        };

                                        // 武器名の表示（使用不可の場合は「×」を追加）
                                        let display_name = if is_usable {
                                            eq_name
                                        } else {
                                            format!("{} ×", eq_name)
                                        };

                                        col.spawn((
                                            Text::new(display_name),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.7, 0.9, 1.0)),
                                        ));
                                    }
                                });

                                // ボタンコンテナ
                                row.spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(5.0),
                                    ..default()
                                })
                                .with_children(|btn_row| {
                                    // 変更ボタン
                                    btn_row
                                        .spawn((
                                            EquipmentButton { slot },
                                            Button,
                                            Node {
                                                width: Val::Px(50.0),
                                                height: Val::Px(28.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                border: UiRect::all(Val::Px(1.0)),
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
                                                    font_size: 12.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                            ));
                                        });

                                    // 外すボタン（装備がある場合のみ表示）
                                    if has_equipment {
                                        btn_row
                                            .spawn((
                                                UnequipButton { slot },
                                                Button,
                                                Node {
                                                    width: Val::Px(50.0),
                                                    height: Val::Px(28.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    border: UiRect::all(Val::Px(1.0)),
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
                                                    Text::new("外す"),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 12.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::WHITE),
                                                ));
                                            });
                                    }
                                });
                            });
                    }

                    // 装備重量・荷重表示
                    let current_equipment = Equipment {
                        weapon1: prep_state.equipped_weapon1.and_then(|id| {
                            equipment_db
                                .weapons
                                .iter()
                                .find(|w| w.id == id)
                                .map(|w| w.weapon.clone())
                        }),
                        weapon2: prep_state.equipped_weapon2.and_then(|id| {
                            equipment_db
                                .weapons
                                .iter()
                                .find(|w| w.id == id)
                                .map(|w| w.weapon.clone())
                        }),
                        armor1: prep_state.equipped_armor1.and_then(|id| {
                            equipment_db
                                .armors
                                .iter()
                                .find(|a| a.id == id)
                                .map(|a| a.armor.clone())
                        }),
                        armor2: prep_state.equipped_armor2.and_then(|id| {
                            equipment_db
                                .armors
                                .iter()
                                .find(|a| a.id == id)
                                .map(|a| a.armor.clone())
                        }),
                        armor3: prep_state.equipped_armor3.and_then(|id| {
                            equipment_db
                                .armors
                                .iter()
                                .find(|a| a.id == id)
                                .map(|a| a.armor.clone())
                        }),
                        armor4: prep_state.equipped_armor4.and_then(|id| {
                            equipment_db
                                .armors
                                .iter()
                                .find(|a| a.id == id)
                                .map(|a| a.armor.clone())
                        }),
                        armor5: prep_state.equipped_armor5.and_then(|id| {
                            equipment_db
                                .armors
                                .iter()
                                .find(|a| a.id == id)
                                .map(|a| a.armor.clone())
                        }),
                        armor6: prep_state.equipped_armor6.and_then(|id| {
                            equipment_db
                                .armors
                                .iter()
                                .find(|a| a.id == id)
                                .map(|a| a.armor.clone())
                        }),
                        armor7: prep_state.equipped_armor7.and_then(|id| {
                            equipment_db
                                .armors
                                .iter()
                                .find(|a| a.id == id)
                                .map(|a| a.armor.clone())
                        }),
                        armor8: prep_state.equipped_armor8.and_then(|id| {
                            equipment_db
                                .armors
                                .iter()
                                .find(|a| a.id == id)
                                .map(|a| a.armor.clone())
                        }),
                    };

                    let player_stats = current_ability.player_stats();
                    let load_perf =
                        current_equipment.load_performance(player_stats.max_equipment_weight);

                    let load_status_text = match load_perf.status {
                        EquipmentLoadPerformanceStatus::Light => "軽量",
                        EquipmentLoadPerformanceStatus::Medium => "中量",
                        EquipmentLoadPerformanceStatus::Heavy => "重量",
                        EquipmentLoadPerformanceStatus::SuperHeavy => "重量過多",
                    };

                    let load_status_color = match load_perf.status {
                        EquipmentLoadPerformanceStatus::Light => Color::srgb(0.4, 1.0, 0.4),
                        EquipmentLoadPerformanceStatus::Medium => Color::srgb(1.0, 1.0, 0.4),
                        EquipmentLoadPerformanceStatus::Heavy => Color::srgb(1.0, 0.6, 0.2),
                        EquipmentLoadPerformanceStatus::SuperHeavy => Color::srgb(1.0, 0.2, 0.2),
                    };

                    // 装備重量セクション
                    left_col
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            margin: UiRect::top(Val::Px(15.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            width: Val::Px(370.0),
                            ..default()
                        })
                        .insert(BackgroundColor(Color::from(LinearRgba {
                            red: 0.15,
                            green: 0.15,
                            blue: 0.2,
                            alpha: 1.0,
                        })))
                        .insert(BorderColor::all(Color::srgb(0.4, 0.4, 0.5)))
                        .with_children(|weight_section| {
                            // タイトル
                            weight_section.spawn((
                                Text::new("装備荷重"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    margin: UiRect::bottom(Val::Px(8.0)),
                                    ..default()
                                },
                            ));

                            // 装備重量: 現在値 / 最大値
                            weight_section
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::bottom(Val::Px(4.0)),
                                    ..default()
                                })
                                .with_children(|row| {
                                    row.spawn((
                                        Text::new("装備重量: "),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                    ));
                                    row.spawn((
                                        Text::new(format!(
                                            "{} / {}",
                                            load_perf.total_weight, load_perf.max_equipment_weight
                                        )),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });

                            // 荷重状態
                            weight_section
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    ..default()
                                })
                                .with_children(|row| {
                                    row.spawn((
                                        Text::new("荷重状態: "),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                    ));
                                    row.spawn((
                                        Text::new(load_status_text),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(load_status_color),
                                    ));
                                });
                        });
                });

            // 右側：武器性能と防御力表示
            main_row
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    ..default()
                })
                .with_children(|right_section| {
                    // 武器性能表示（横並び）
                    right_section
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            ..default()
                        })
                        .with_children(|weapons_row| {
                            // 右手武器の性能表示
                            weapons_row
                                .spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    width: Val::Px(320.0),
                                    ..default()
                                })
                                .with_children(|weapon_col| {
                                    if let Some(weapon_data) = weapon1_data {
                                        let performance =
                                            weapon_data.weapon.performance(&current_ability);
                                        build_weapon_performance_display(
                                            weapon_col,
                                            font.clone(),
                                            "右手武器",
                                            &weapon_data.weapon,
                                            &performance,
                                        );
                                    } else {
                                        weapon_col.spawn((
                                            Text::new("■ 右手武器性能"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.5, 1.0, 0.8)),
                                            Node {
                                                margin: UiRect::bottom(Val::Px(5.0)),
                                                ..default()
                                            },
                                        ));
                                        weapon_col.spawn((
                                            Text::new("  未装備"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.6, 0.6, 0.6)),
                                        ));
                                    }
                                });

                            // 左手武器の性能表示
                            weapons_row
                                .spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    width: Val::Px(320.0),
                                    ..default()
                                })
                                .with_children(|weapon_col| {
                                    if let Some(weapon_data) = weapon2_data {
                                        let performance =
                                            weapon_data.weapon.performance(&current_ability);
                                        build_weapon_performance_display(
                                            weapon_col,
                                            font.clone(),
                                            "左手武器",
                                            &weapon_data.weapon,
                                            &performance,
                                        );
                                    } else {
                                        weapon_col.spawn((
                                            Text::new("■ 左手武器性能"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.5, 1.0, 0.8)),
                                            Node {
                                                margin: UiRect::bottom(Val::Px(5.0)),
                                                ..default()
                                            },
                                        ));
                                        weapon_col.spawn((
                                            Text::new("  未装備"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.6, 0.6, 0.6)),
                                        ));
                                    }
                                });
                        });

                    // 防御力表示セクション
                    // 基礎防御力（能力値から算出）
                    let base_defense = current_ability.base_defense_power();

                    // 装備防御力を計算
                    let mut equipment_defense = DefensePower::default();
                    let armor_ids = [
                        prep_state.equipped_armor1,
                        prep_state.equipped_armor2,
                        prep_state.equipped_armor3,
                        prep_state.equipped_armor4,
                        prep_state.equipped_armor5,
                        prep_state.equipped_armor6,
                        prep_state.equipped_armor7,
                        prep_state.equipped_armor8,
                    ];
                    for armor_id in armor_ids.iter().flatten() {
                        if let Some(armor_data) =
                            equipment_db.armors.iter().find(|a| a.id == *armor_id)
                        {
                            equipment_defense.add(&armor_data.armor.defense);
                        }
                    }

                    right_section
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            margin: UiRect::top(Val::Px(15.0)),
                            ..default()
                        })
                        .with_children(|defense_section| {
                            // セクションヘッダー
                            defense_section.spawn((
                                Text::new("■ 防御力"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 1.0, 0.8)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(5.0)),
                                    ..default()
                                },
                            ));

                            // 防御力テーブル
                            defense_section
                                .spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(2.0),
                                    ..default()
                                })
                                .with_children(|table| {
                                    // ヘッダー行
                                    table
                                        .spawn(Node {
                                            flex_direction: FlexDirection::Row,
                                            column_gap: Val::Px(8.0),
                                            margin: UiRect::bottom(Val::Px(3.0)),
                                            ..default()
                                        })
                                        .with_children(|header_row| {
                                            header_row.spawn((
                                                Text::new("属性"),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: 13.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.8, 0.8, 0.5)),
                                                Node {
                                                    width: Val::Px(50.0),
                                                    ..default()
                                                },
                                            ));
                                            header_row.spawn((
                                                Text::new("能力"),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: 13.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.8, 0.8, 0.5)),
                                                Node {
                                                    width: Val::Px(40.0),
                                                    justify_content: JustifyContent::End,
                                                    ..default()
                                                },
                                            ));
                                            header_row.spawn((
                                                Text::new("+"),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: 13.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.8, 0.8, 0.5)),
                                                Node {
                                                    width: Val::Px(12.0),
                                                    justify_content: JustifyContent::Center,
                                                    ..default()
                                                },
                                            ));
                                            header_row.spawn((
                                                Text::new("装備"),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: 13.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.8, 0.8, 0.5)),
                                                Node {
                                                    width: Val::Px(40.0),
                                                    justify_content: JustifyContent::End,
                                                    ..default()
                                                },
                                            ));
                                            header_row.spawn((
                                                Text::new("="),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: 13.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.8, 0.8, 0.5)),
                                                Node {
                                                    width: Val::Px(12.0),
                                                    justify_content: JustifyContent::Center,
                                                    ..default()
                                                },
                                            ));
                                            header_row.spawn((
                                                Text::new("合計"),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: 13.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.8, 0.8, 0.5)),
                                                Node {
                                                    width: Val::Px(40.0),
                                                    justify_content: JustifyContent::End,
                                                    ..default()
                                                },
                                            ));
                                        });

                                    // 各属性の防御力を表示
                                    let defense_data = [
                                        ("斬撃", base_defense.slash, equipment_defense.slash),
                                        ("打撃", base_defense.strike, equipment_defense.strike),
                                        ("刺突", base_defense.thrust, equipment_defense.thrust),
                                        ("衝撃", base_defense.impact, equipment_defense.impact),
                                        ("魔力", base_defense.magic, equipment_defense.magic),
                                        ("炎", base_defense.fire, equipment_defense.fire),
                                        ("雷", base_defense.lightning, equipment_defense.lightning),
                                        ("混濁", base_defense.chaos, equipment_defense.chaos),
                                    ];

                                    for (name, base, equip) in defense_data {
                                        let total = base + equip;
                                        table
                                            .spawn(Node {
                                                flex_direction: FlexDirection::Row,
                                                column_gap: Val::Px(8.0),
                                                ..default()
                                            })
                                            .with_children(|data_row| {
                                                // 属性名
                                                data_row.spawn((
                                                    Text::new(name),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 13.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::WHITE),
                                                    Node {
                                                        width: Val::Px(50.0),
                                                        ..default()
                                                    },
                                                ));
                                                // 能力値による防御力
                                                data_row.spawn((
                                                    Text::new(format!("{}", base)),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 13.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::srgb(0.7, 0.9, 1.0)),
                                                    Node {
                                                        width: Val::Px(40.0),
                                                        justify_content: JustifyContent::End,
                                                        ..default()
                                                    },
                                                ));
                                                // プラス記号
                                                data_row.spawn((
                                                    Text::new("+"),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 13.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::WHITE),
                                                    Node {
                                                        width: Val::Px(12.0),
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                ));
                                                // 装備による防御力
                                                data_row.spawn((
                                                    Text::new(format!("{}", equip)),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 13.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::srgb(0.5, 1.0, 0.5)),
                                                    Node {
                                                        width: Val::Px(40.0),
                                                        justify_content: JustifyContent::End,
                                                        ..default()
                                                    },
                                                ));
                                                // イコール記号
                                                data_row.spawn((
                                                    Text::new("="),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 13.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::WHITE),
                                                    Node {
                                                        width: Val::Px(12.0),
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                ));
                                                // 合計
                                                data_row.spawn((
                                                    Text::new(format!("{}", total)),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 13.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::srgb(1.0, 1.0, 0.5)),
                                                    Node {
                                                        width: Val::Px(40.0),
                                                        justify_content: JustifyContent::End,
                                                        ..default()
                                                    },
                                                ));
                                            });
                                    }
                                });
                        });
                });
        });
}

/// アーツの必要能力を満たしているかチェック
fn check_art_requirement(prep_state: &PreparationState, requirement: &ArtRequirement) -> bool {
    prep_state.temp_strength >= requirement.strength
        && prep_state.temp_dexterity >= requirement.dexterity
        && prep_state.temp_intelligence >= requirement.intelligence
        && prep_state.temp_faith >= requirement.faith
        && prep_state.temp_arcane >= requirement.arcane
        && prep_state.temp_agility >= requirement.agility
}

/// 技術設定画面のコンテンツを構築
fn build_arts_content(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    prep_state: &PreparationState,
    arts_db: &ArtsDatabase,
) {
    parent.spawn((
        Text::new("技術設定"),
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

    parent.spawn((
        Text::new("戦闘で使用する技術を4つまで設定できます"),
        TextFont {
            font: font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
    ));

    // 設定中の技術と利用可能な技術を横並びで表示
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|row_parent| {
            // 左側：設定中の技術（幅30%）
            row_parent
                .spawn(Node {
                    width: Val::Percent(30.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|slots_parent| {
                    slots_parent.spawn((
                        Text::new("■ 設定中の技術"),
                        TextFont {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 1.0, 0.8)),
                        Node {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                    ));

                    // 8つのスロット
                    for slot_index in 0..8 {
                        let selected_art = prep_state
                            .selected_arts
                            .get(slot_index)
                            .and_then(|&art_id| arts_db.arts.iter().find(|a| a.id == art_id));

                        slots_parent
                            .spawn((
                                ArtsSlotButton { slot_index },
                                Button,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(50.0),
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::all(Val::Px(8.0)),
                                    border: UiRect::all(Val::Px(2.0)),
                                    justify_content: JustifyContent::Center,
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
                            .with_children(|slot| {
                                if let Some(art) = selected_art {
                                    // 必要能力を満たしているかチェック
                                    let meets_requirement =
                                        check_art_requirement(prep_state, &art.art.requirement);

                                    // 技術名（必要能力が足りない場合は×を表示）
                                    let name_text = if meets_requirement {
                                        format!("{}. {}", slot_index + 1, art.name)
                                    } else {
                                        format!("{}. {} ×", slot_index + 1, art.name)
                                    };
                                    let text_color = if meets_requirement {
                                        Color::WHITE
                                    } else {
                                        Color::srgb(1.0, 0.4, 0.4) // 赤っぽい色で警告
                                    };
                                    slot.spawn((
                                        Text::new(name_text),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(text_color),
                                    ));

                                    // 技術情報
                                    let info = format!(
                                        "SP:{} / ST:{}",
                                        art.art.sp_cost, art.art.stamina_cost
                                    );
                                    slot.spawn((
                                        Text::new(info),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                    ));
                                } else {
                                    slot.spawn((
                                        Text::new(format!("{}. [空き]", slot_index + 1)),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                    ));
                                }
                            });
                    }
                });

            // 右側：利用可能な技術（幅70%）
            row_parent
                .spawn(Node {
                    width: Val::Percent(70.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|available_parent| {
                    available_parent.spawn((
                        Text::new("■ 利用可能な技術"),
                        TextFont {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 1.0, 0.8)),
                        Node {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                    ));

                    // 技術一覧をグリッド表示（横3列）
                    available_parent
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            column_gap: Val::Px(10.0),
                            row_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|grid| {
                            for art_data in &arts_db.arts {
                                grid.spawn(Node {
                                    width: Val::Percent(32.0), // 3列表示（100% / 3 ≈ 33%、ギャップを考慮して32%）
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::all(Val::Px(12.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                })
                                .with_children(|art_panel| {
                                    // 技術名
                                    art_panel.spawn((
                                        Text::new(&art_data.name),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(1.0, 0.9, 0.6)),
                                    ));

                                    // コスト情報
                                    art_panel.spawn((
                                        Text::new(format!(
                                            "SP: {} / スタミナ: {}",
                                            art_data.art.sp_cost, art_data.art.stamina_cost
                                        )),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                    ));

                                    // 必要能力
                                    let req = &art_data.art.requirement;
                                    let mut req_parts = Vec::new();
                                    if req.strength > 0 {
                                        req_parts.push(format!("筋力{}", req.strength));
                                    }
                                    if req.dexterity > 0 {
                                        req_parts.push(format!("技量{}", req.dexterity));
                                    }
                                    if req.intelligence > 0 {
                                        req_parts.push(format!("知力{}", req.intelligence));
                                    }
                                    if req.faith > 0 {
                                        req_parts.push(format!("信仰{}", req.faith));
                                    }
                                    if req.arcane > 0 {
                                        req_parts.push(format!("神秘{}", req.arcane));
                                    }
                                    if req.agility > 0 {
                                        req_parts.push(format!("敏捷{}", req.agility));
                                    }

                                    if !req_parts.is_empty() {
                                        art_panel.spawn((
                                            Text::new(format!("必要: {}", req_parts.join(", "))),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.6, 0.8, 1.0)),
                                        ));
                                    }
                                });
                            }
                        });
                });
        });
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

#[derive(Component, Clone, Copy)]
pub struct UnequipButton {
    pub slot: EquipmentSlot,
}

#[derive(Component, Clone, Copy)]
pub struct ArtsSlotButton {
    pub slot_index: usize,
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

/// 装備を外すシステム
pub fn unequip_system(
    mut interaction_query: Query<
        (&Interaction, &UnequipButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut prep_state: ResMut<PreparationState>,
) {
    for (interaction, unequip_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // 選択されたスロットの装備を外す
                match unequip_button.slot {
                    EquipmentSlot::Weapon1 => prep_state.equipped_weapon1 = None,
                    EquipmentSlot::Weapon2 => prep_state.equipped_weapon2 = None,
                    EquipmentSlot::Armor1 => prep_state.equipped_armor1 = None,
                    EquipmentSlot::Armor2 => prep_state.equipped_armor2 = None,
                    EquipmentSlot::Armor3 => prep_state.equipped_armor3 = None,
                    EquipmentSlot::Armor4 => prep_state.equipped_armor4 = None,
                    EquipmentSlot::Armor5 => prep_state.equipped_armor5 = None,
                    EquipmentSlot::Armor6 => prep_state.equipped_armor6 = None,
                    EquipmentSlot::Armor7 => prep_state.equipped_armor7 = None,
                    EquipmentSlot::Armor8 => prep_state.equipped_armor8 = None,
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
            name: "賢者の杖".to_string(),
            weapon: create_wizard_staff(),
        },
        // WeaponData {
        //     id: 1,
        //     name: "グレートソード".to_string(),
        //     weapon: create_greatsword(),
        // },
        // WeaponData {
        //     id: 2,
        //     name: "スピア".to_string(),
        //     weapon: create_spear(),
        // },
        // WeaponData {
        //     id: 3,
        //     name: "バトルアックス".to_string(),
        //     weapon: create_axe(),
        // },
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

// ================== 技術データベース生成 ==================

fn create_arts_database() -> ArtsDatabase {
    let arts = vec![
        ArtsData {
            id: 0,
            name: "なぎ払い".to_string(),
            art: Art {
                name: "なぎ払い".to_string(),
                sp_cost: 8,
                stamina_cost: 40,
                perks: vec![ArtPerk::Melee],
                requirement: ArtRequirement {
                    strength: 15,
                    dexterity: 10,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                usable_weapon: ArtUsableWeapon::Specific(vec![
                    WeaponKind::StraightSword,
                    WeaponKind::Greatsword,
                    WeaponKind::Staff,
                ]),
                art_type: ArtType::Skill,
                always_hits: false,
                priority: 0,
                rank1: ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Attack(ArtPotencyAttack {
                        attack_power: AttackPower {
                            slash: 50,
                            strike: 50,
                            thrust: 0,
                            impact: 0,
                            magic: 0,
                            fire: 0,
                            lightning: 0,
                            chaos: 0,
                        },
                        weapon_attack_power_scaling: AttackPowerScaling {
                            slash: 1.5,
                            strike: 1.5,
                            thrust: 0.0,
                            impact: 0.0,
                            magic: 1.0,
                            fire: 1.0,
                            lightning: 1.0,
                            chaos: 1.0,
                        },
                        break_power: 80,
                        weapon_break_power_scaling: 2.0,
                    }),
                },
                rank2: None,
                rank3: None,
            },
        },
        ArtsData {
            id: 1,
            name: "雷鳴剣".to_string(),
            art: Art {
                name: "雷鳴剣".to_string(),
                sp_cost: 40,
                stamina_cost: 30,
                perks: vec![],
                requirement: ArtRequirement {
                    strength: 15,
                    dexterity: 15,
                    intelligence: 0,
                    faith: 20,
                    arcane: 0,
                    agility: 0,
                },
                usable_weapon: ArtUsableWeapon::Specific(vec![
                    WeaponKind::StraightSword,
                    WeaponKind::Greatsword,
                ]),
                art_type: ArtType::Skill,
                always_hits: false,
                priority: 0,
                rank1: ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Attack(ArtPotencyAttack {
                        attack_power: AttackPower {
                            slash: 100,
                            strike: 50,
                            thrust: 0,
                            impact: 0,
                            magic: 0,
                            fire: 0,
                            lightning: 150,
                            chaos: 0,
                        },
                        weapon_attack_power_scaling: AttackPowerScaling {
                            slash: 2.0,
                            strike: 1.0,
                            thrust: 1.0,
                            impact: 1.0,
                            magic: 1.0,
                            fire: 0.0,
                            lightning: 2.0,
                            chaos: 0.0,
                        },
                        break_power: 100,
                        weapon_break_power_scaling: 3.0,
                    }),
                },
                rank2: None,
                rank3: None,
            },
        },
        ArtsData {
            id: 2,
            name: "マジックアロー".to_string(),
            art: Art {
                name: "マジックアロー".to_string(),
                sp_cost: 25,
                stamina_cost: 10,
                perks: vec![ArtPerk::Ranged],
                requirement: ArtRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 15,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                usable_weapon: ArtUsableWeapon::All,
                always_hits: false,
                art_type: ArtType::Sorcery,
                priority: 0,
                rank1: ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Attack(ArtPotencyAttack {
                        attack_power: AttackPower {
                            slash: 0,
                            strike: 0,
                            thrust: 100,
                            impact: 0,
                            magic: 150,
                            fire: 0,
                            lightning: 0,
                            chaos: 0,
                        },
                        weapon_attack_power_scaling: AttackPowerScaling::default(),
                        break_power: 50,
                        weapon_break_power_scaling: 0.0,
                    }),
                },
                rank2: Some(ArtRank {
                    threshold: 30,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Attack(ArtPotencyAttack {
                        attack_power: AttackPower {
                            slash: 0,
                            strike: 0,
                            thrust: 150,
                            impact: 0,
                            magic: 350,
                            fire: 0,
                            lightning: 0,
                            chaos: 0,
                        },
                        weapon_attack_power_scaling: AttackPowerScaling::default(),
                        break_power: 100,
                        weapon_break_power_scaling: 0.0,
                    }),
                }),
                rank3: Some(ArtRank {
                    threshold: 65,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Attack(ArtPotencyAttack {
                        attack_power: AttackPower {
                            slash: 0,
                            strike: 0,
                            thrust: 400,
                            impact: 0,
                            magic: 500,
                            fire: 0,
                            lightning: 0,
                            chaos: 0,
                        },
                        weapon_attack_power_scaling: AttackPowerScaling::default(),
                        break_power: 200,
                        weapon_break_power_scaling: 0.0,
                    }),
                }),
            },
        },
        ArtsData {
            id: 3,
            name: "雷撃".to_string(),
            art: Art {
                name: "雷撃".to_string(),
                sp_cost: 55,
                stamina_cost: 30,
                perks: vec![ArtPerk::Ranged],
                requirement: ArtRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 15,
                    faith: 18,
                    arcane: 0,
                    agility: 0,
                },
                usable_weapon: ArtUsableWeapon::All,
                always_hits: false,
                art_type: ArtType::Sorcery,
                priority: 0,
                rank1: ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Attack(ArtPotencyAttack {
                        attack_power: AttackPower {
                            slash: 0,
                            strike: 0,
                            thrust: 0,
                            impact: 0,
                            magic: 0,
                            fire: 0,
                            lightning: 300,
                            chaos: 0,
                        },
                        weapon_attack_power_scaling: AttackPowerScaling::default(),
                        break_power: 100,
                        weapon_break_power_scaling: 0.0,
                    }),
                },
                rank2: Some(ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Attack(ArtPotencyAttack {
                        attack_power: AttackPower {
                            slash: 0,
                            strike: 0,
                            thrust: 0,
                            impact: 0,
                            magic: 0,
                            fire: 0,
                            lightning: 500,
                            chaos: 0,
                        },
                        weapon_attack_power_scaling: AttackPowerScaling::default(),
                        break_power: 150,
                        weapon_break_power_scaling: 0.0,
                    }),
                }),
                rank3: Some(ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Attack(ArtPotencyAttack {
                        attack_power: AttackPower {
                            slash: 0,
                            strike: 0,
                            thrust: 0,
                            impact: 0,
                            magic: 0,
                            fire: 0,
                            lightning: 700,
                            chaos: 0,
                        },
                        weapon_attack_power_scaling: AttackPowerScaling::default(),
                        break_power: 300,
                        weapon_break_power_scaling: 0.0,
                    }),
                }),
            },
        },
        ArtsData {
            id: 4,
            name: "渾身の一撃".to_string(),
            art: Art {
                name: "渾身の一撃".to_string(),
                sp_cost: 30,
                stamina_cost: 40,
                perks: vec![ArtPerk::Melee],
                requirement: ArtRequirement {
                    strength: 20,
                    dexterity: 8,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                usable_weapon: ArtUsableWeapon::Specific(vec![
                    WeaponKind::Greatsword,
                    WeaponKind::Axe,
                ]),
                always_hits: true,
                art_type: ArtType::Skill,
                priority: 0,
                rank1: ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Attack(ArtPotencyAttack {
                        attack_power: AttackPower {
                            slash: 250,
                            strike: 100,
                            thrust: 0,
                            impact: 50,
                            magic: 0,
                            fire: 0,
                            lightning: 0,
                            chaos: 0,
                        },
                        weapon_attack_power_scaling: AttackPowerScaling {
                            slash: 2.0,
                            strike: 1.5,
                            thrust: 0.0,
                            impact: 1.0,
                            magic: 0.0,
                            fire: 0.0,
                            lightning: 0.0,
                            chaos: 0.0,
                        },
                        break_power: 50,
                        weapon_break_power_scaling: 1.5,
                    }),
                },
                rank2: None,
                rank3: None,
            },
        },
        ArtsData {
            id: 5,
            name: "ヒール".to_string(),
            art: Art {
                name: "ヒール".to_string(),
                sp_cost: 25,
                stamina_cost: 10,
                perks: vec![ArtPerk::Ranged, ArtPerk::AtFeet],
                requirement: ArtRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 12,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                usable_weapon: ArtUsableWeapon::All,
                always_hits: false,
                art_type: ArtType::Sorcery,
                priority: 0,
                rank1: ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Support(ArtPotencySupport::Recover(
                        ArtPotencySupportRecover {
                            potencies: vec![SupportRecoverPotency::Hp(SupportRecoverPotencyHp {
                                hp_recover: 50,
                            })],
                        },
                    )),
                },
                rank2: Some(ArtRank {
                    threshold: 25,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Support(ArtPotencySupport::Recover(
                        ArtPotencySupportRecover {
                            potencies: vec![SupportRecoverPotency::Hp(SupportRecoverPotencyHp {
                                hp_recover: 70,
                            })],
                        },
                    )),
                }),
                rank3: None,
            },
        },
        // 基本タイプのアーツ
        ArtsData {
            id: 6,
            name: "回避".to_string(),
            art: Art {
                name: "回避".to_string(),
                sp_cost: 0,
                stamina_cost: 15,
                perks: vec![],
                requirement: ArtRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 10,
                },
                usable_weapon: ArtUsableWeapon::All,
                art_type: ArtType::Basic,
                always_hits: true,
                priority: 10,
                rank1: ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Support(ArtPotencySupport::StatusCondition(
                        ArtPotencySupportStatusCondition {
                            status_conditions: vec![StatusCondition {
                                potency: StatusConditionPotency::Evasion,
                                duration: StatusConditionDuration::Turn(
                                    StatusConditionDurationTurn { turns: 1 },
                                ),
                            }],
                        },
                    )),
                },
                rank2: None,
                rank3: None,
            },
        },
        ArtsData {
            id: 7,
            name: "瞑想".to_string(),
            art: Art {
                name: "瞑想".to_string(),
                sp_cost: 0,
                stamina_cost: 25,
                perks: vec![],
                requirement: ArtRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 0,
                },
                art_type: ArtType::Basic,
                usable_weapon: ArtUsableWeapon::All,
                always_hits: true,
                priority: 0,
                rank1: ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Support(ArtPotencySupport::Recover(
                        ArtPotencySupportRecover {
                            potencies: vec![SupportRecoverPotency::Sp(SupportRecoverPotencySp {
                                sp_recover: 30,
                            })],
                        },
                    )),
                },
                rank2: None,
                rank3: None,
            },
        },
        ArtsData {
            id: 8,
            name: "バックステップ".to_string(),
            art: Art {
                name: "バックステップ".to_string(),
                sp_cost: 0,
                stamina_cost: 5,
                perks: vec![],
                requirement: ArtRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 10,
                },
                usable_weapon: ArtUsableWeapon::All,
                art_type: ArtType::Basic,
                always_hits: true,
                priority: 5,
                rank1: ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Support(ArtPotencySupport::StatusCondition(
                        ArtPotencySupportStatusCondition {
                            status_conditions: vec![StatusCondition {
                                potency: StatusConditionPotency::Ranged,
                                duration: StatusConditionDuration::Turn(
                                    StatusConditionDurationTurn { turns: 1 },
                                ),
                            }],
                        },
                    )),
                },
                rank2: None,
                rank3: None,
            },
        },
        ArtsData {
            id: 9,
            name: "ジャンプ".to_string(),
            art: Art {
                name: "ジャンプ".to_string(),
                sp_cost: 0,
                stamina_cost: 5,
                perks: vec![],
                requirement: ArtRequirement {
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    faith: 0,
                    arcane: 0,
                    agility: 10,
                },
                usable_weapon: ArtUsableWeapon::All,
                art_type: ArtType::Basic,
                always_hits: true,
                priority: 5,
                rank1: ArtRank {
                    threshold: 0,
                    target: ArtTarget::Single,
                    potency: ArtPotency::Support(ArtPotencySupport::StatusCondition(
                        ArtPotencySupportStatusCondition {
                            status_conditions: vec![StatusCondition {
                                potency: StatusConditionPotency::Floating,
                                duration: StatusConditionDuration::Turn(
                                    StatusConditionDurationTurn { turns: 1 },
                                ),
                            }],
                        },
                    )),
                },
                rank2: None,
                rank3: None,
            },
        },
    ];

    ArtsDatabase { arts }
}

// 武器生成関数（簡略化のため一部のみ実装）
fn create_longsword() -> Weapon {
    Weapon {
        name: "ロングソード".to_string(),
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
                strike: 50,
                thrust: 100,
                impact: 0,
                magic: 0,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            ability_scaling: WeaponAttackPowerAbilityScaling {
                slash: AbilityScaling {
                    strength: 3.0,
                    dexterity: 3.0,
                    intelligence: 0.0,
                    faith: 0.0,
                    arcane: 0.0,
                    agility: 0.0,
                },
                strike: AbilityScaling {
                    strength: 3.0,
                    dexterity: 0.0,
                    intelligence: 0.0,
                    faith: 0.0,
                    arcane: 0.0,
                    agility: 0.0,
                },
                thrust: AbilityScaling {
                    strength: 3.0,
                    dexterity: 0.0,
                    intelligence: 0.0,
                    faith: 0.0,
                    arcane: 0.0,
                    agility: 0.0,
                },
                impact: create_default_ability_scaling(),
                magic: create_default_ability_scaling(),
                fire: create_default_ability_scaling(),
                lightning: create_default_ability_scaling(),
                chaos: create_default_ability_scaling(),
            },
        },
        sorcery_power: create_default_sorcery_power(),
        break_power: WeaponBreakPower {
            base_power: 20,
            scaling: AbilityScaling {
                strength: 2.0,
                dexterity: 2.0,
                intelligence: 0.0,
                faith: 0.0,
                arcane: 0.0,
                agility: 0.0,
            },
        },
        guard: WeaponGuard {
            cut_rate: GuardCutRate {
                slash: 0.4,
                strike: 0.6,
                thrust: 0.7,
                impact: 0.7,
                magic: 0.9,
                fire: 0.9,
                lightning: 0.8,
                chaos: 0.9,
            },
            guard_strength: 30,
        },
    }
}

fn create_wizard_staff() -> Weapon {
    Weapon {
        name: "賢者の杖".to_string(),
        kind: WeaponKind::Staff,
        weight: 5,
        ability_requirement: WeaponAbilityRequirement {
            strength: 0,
            dexterity: 0,
            intelligence: 15,
            faith: 0,
            arcane: 0,
            agility: 0,
        },
        attack_power: WeaponAttackPower {
            base: AttackPower {
                slash: 0,
                strike: 10,
                thrust: 10,
                impact: 0,
                magic: 10,
                fire: 0,
                lightning: 0,
                chaos: 0,
            },
            ability_scaling: WeaponAttackPowerAbilityScaling {
                slash: create_default_ability_scaling(),
                strike: AbilityScaling {
                    strength: 3.0,
                    dexterity: 0.0,
                    intelligence: 0.0,
                    faith: 0.0,
                    arcane: 0.0,
                    agility: 0.0,
                },
                thrust: create_default_ability_scaling(),
                impact: create_default_ability_scaling(),
                magic: AbilityScaling {
                    strength: 0.0,
                    dexterity: 0.0,
                    intelligence: 5.0,
                    faith: 0.0,
                    arcane: 0.0,
                    agility: 0.0,
                },
                fire: create_default_ability_scaling(),
                lightning: create_default_ability_scaling(),
                chaos: create_default_ability_scaling(),
            },
        },
        sorcery_power: WeaponSorceryPower {
            base: 30,
            scaling: AbilityScaling {
                strength: 0.0,
                dexterity: 0.0,
                intelligence: 1.0,
                faith: 0.5,
                arcane: 0.0,
                agility: 0.0,
            },
        },
        break_power: WeaponBreakPower {
            base_power: 10,
            scaling: AbilityScaling {
                strength: 2.0,
                dexterity: 0.0,
                intelligence: 0.0,
                faith: 0.0,
                arcane: 0.0,
                agility: 0.0,
            },
        },
        guard: WeaponGuard {
            cut_rate: GuardCutRate {
                slash: 0.8,
                strike: 0.8,
                thrust: 0.9,
                impact: 0.9,
                magic: 0.5,
                fire: 0.7,
                lightning: 0.6,
                chaos: 0.5,
            },
            guard_strength: 30,
        },
    }
}

fn create_greatsword() -> Weapon {
    Weapon {
        name: "大剣".to_string(),
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
            ability_scaling: WeaponAttackPowerAbilityScaling {
                slash: AbilityScaling {
                    strength: 0.5,
                    dexterity: 0.5,
                    intelligence: 0.0,
                    faith: 0.0,
                    arcane: 0.0,
                    agility: 0.0,
                },
                strike: AbilityScaling {
                    strength: 0.5,
                    dexterity: 0.0,
                    intelligence: 0.0,
                    faith: 0.0,
                    arcane: 0.0,
                    agility: 0.0,
                },
                thrust: create_default_ability_scaling(),
                impact: create_default_ability_scaling(),
                magic: create_default_ability_scaling(),
                fire: create_default_ability_scaling(),
                lightning: create_default_ability_scaling(),
                chaos: create_default_ability_scaling(),
            },
        },
        sorcery_power: create_default_sorcery_power(),
        break_power: WeaponBreakPower {
            base_power: 40,
            scaling: AbilityScaling {
                strength: 1.0,
                dexterity: 0.0,
                intelligence: 0.0,
                faith: 0.0,
                arcane: 0.0,
                agility: 0.0,
            },
        },
        guard: create_default_guard(),
    }
}

fn create_spear() -> Weapon {
    Weapon {
        name: "槍".to_string(),
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
        name: "斧".to_string(),
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
        name: "盾".to_string(),
        kind: WeaponKind::Shield,
        weight: 10,
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
                slash: 0,
                strike: 35,
                thrust: 0,
                impact: 35,
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
            scaling: AbilityScaling {
                strength: 1.0,
                dexterity: 0.0,
                intelligence: 0.0,
                faith: 0.0,
                arcane: 0.0,
                agility: 0.0,
            },
        },
        guard: WeaponGuard {
            cut_rate: GuardCutRate {
                slash: 0.4,
                strike: 0.5,
                thrust: 0.4,
                impact: 0.6,
                magic: 0.6,
                fire: 0.6,
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
        defense: DefensePower {
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
        slots: vec![ArmorSlot::Head],
    }
}

fn create_iron_armor() -> Armor {
    Armor {
        kind: ArmorKind::ChestArmor,
        weight: 20,
        defense: DefensePower {
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
        slots: vec![ArmorSlot::Chest],
    }
}

fn create_iron_gauntlets() -> Armor {
    Armor {
        kind: ArmorKind::Gauntlets,
        weight: 6,
        defense: DefensePower {
            slash: 10,
            strike: 10,
            thrust: 10,
            impact: 8,
            magic: 5,
            fire: 7,
            lightning: 7,
            chaos: 5,
        },
        slots: vec![ArmorSlot::Arms],
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
        defense: DefensePower {
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
        slots: vec![ArmorSlot::Legs],
    }
}

fn create_leather_helmet() -> Armor {
    Armor {
        kind: ArmorKind::Helmet,
        weight: 3,
        defense: DefensePower {
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
        slots: vec![ArmorSlot::Head],
    }
}

fn create_leather_armor() -> Armor {
    Armor {
        kind: ArmorKind::ChestArmor,
        weight: 8,
        defense: DefensePower {
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
        slots: vec![ArmorSlot::Chest],
    }
}

fn create_leather_gauntlets() -> Armor {
    Armor {
        kind: ArmorKind::Gauntlets,
        weight: 2,
        defense: DefensePower {
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
        slots: vec![ArmorSlot::Arms],
    }
}

fn create_leather_leggings() -> Armor {
    Armor {
        kind: ArmorKind::LegArmor,
        weight: 5,
        defense: DefensePower {
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
        slots: vec![ArmorSlot::Legs],
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
    equipment_db: Res<EquipmentDatabase>,
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
                        EquipmentSlot::Armor1
                        | EquipmentSlot::Armor2
                        | EquipmentSlot::Armor3
                        | EquipmentSlot::Armor4
                        | EquipmentSlot::Armor5
                        | EquipmentSlot::Armor6
                        | EquipmentSlot::Armor7
                        | EquipmentSlot::Armor8 => {
                            // 選択された防具を取得
                            if let Some(armor_data) = equipment_db
                                .armors
                                .iter()
                                .find(|a| a.id == list_button.equipment_id)
                            {
                                // 現在の装備状態からEquipmentを構築
                                let current_equipment = Equipment {
                                    weapon1: prep_state.equipped_weapon1.and_then(|id| {
                                        equipment_db
                                            .weapons
                                            .iter()
                                            .find(|w| w.id == id)
                                            .map(|w| w.weapon.clone())
                                    }),
                                    weapon2: prep_state.equipped_weapon2.and_then(|id| {
                                        equipment_db
                                            .weapons
                                            .iter()
                                            .find(|w| w.id == id)
                                            .map(|w| w.weapon.clone())
                                    }),
                                    armor1: prep_state.equipped_armor1.and_then(|id| {
                                        equipment_db
                                            .armors
                                            .iter()
                                            .find(|a| a.id == id)
                                            .map(|a| a.armor.clone())
                                    }),
                                    armor2: prep_state.equipped_armor2.and_then(|id| {
                                        equipment_db
                                            .armors
                                            .iter()
                                            .find(|a| a.id == id)
                                            .map(|a| a.armor.clone())
                                    }),
                                    armor3: prep_state.equipped_armor3.and_then(|id| {
                                        equipment_db
                                            .armors
                                            .iter()
                                            .find(|a| a.id == id)
                                            .map(|a| a.armor.clone())
                                    }),
                                    armor4: prep_state.equipped_armor4.and_then(|id| {
                                        equipment_db
                                            .armors
                                            .iter()
                                            .find(|a| a.id == id)
                                            .map(|a| a.armor.clone())
                                    }),
                                    armor5: prep_state.equipped_armor5.and_then(|id| {
                                        equipment_db
                                            .armors
                                            .iter()
                                            .find(|a| a.id == id)
                                            .map(|a| a.armor.clone())
                                    }),
                                    armor6: prep_state.equipped_armor6.and_then(|id| {
                                        equipment_db
                                            .armors
                                            .iter()
                                            .find(|a| a.id == id)
                                            .map(|a| a.armor.clone())
                                    }),
                                    armor7: prep_state.equipped_armor7.and_then(|id| {
                                        equipment_db
                                            .armors
                                            .iter()
                                            .find(|a| a.id == id)
                                            .map(|a| a.armor.clone())
                                    }),
                                    armor8: prep_state.equipped_armor8.and_then(|id| {
                                        equipment_db
                                            .armors
                                            .iter()
                                            .find(|a| a.id == id)
                                            .map(|a| a.armor.clone())
                                    }),
                                };

                                // 装備可能かチェック
                                if current_equipment.is_equippable(&armor_data.armor) {
                                    match slot {
                                        EquipmentSlot::Armor1 => {
                                            prep_state.equipped_armor1 =
                                                Some(list_button.equipment_id);
                                        }
                                        EquipmentSlot::Armor2 => {
                                            prep_state.equipped_armor2 =
                                                Some(list_button.equipment_id);
                                        }
                                        EquipmentSlot::Armor3 => {
                                            prep_state.equipped_armor3 =
                                                Some(list_button.equipment_id);
                                        }
                                        EquipmentSlot::Armor4 => {
                                            prep_state.equipped_armor4 =
                                                Some(list_button.equipment_id);
                                        }
                                        EquipmentSlot::Armor5 => {
                                            prep_state.equipped_armor5 =
                                                Some(list_button.equipment_id);
                                        }
                                        EquipmentSlot::Armor6 => {
                                            prep_state.equipped_armor6 =
                                                Some(list_button.equipment_id);
                                        }
                                        EquipmentSlot::Armor7 => {
                                            prep_state.equipped_armor7 =
                                                Some(list_button.equipment_id);
                                        }
                                        EquipmentSlot::Armor8 => {
                                            prep_state.equipped_armor8 =
                                                Some(list_button.equipment_id);
                                        }
                                        _ => {}
                                    }
                                } else {
                                    // 装備できない場合はエラーメッセージを表示
                                    prep_state.error_message = Some("この防具は装備できません: すでに同じ装備箇所の防具が装備されています".to_string());
                                    prep_state.error_message_timer = 3.0;
                                }
                            }
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

// ================== エラーメッセージ表示 ==================

#[derive(Component)]
struct ErrorMessagePanel;

// ================== 技術選択ダイアログ ==================

/// 技術選択ダイアログのタブ種別
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ArtTypeTab {
    #[default]
    Basic, // 基本
    Skill,   // 技
    Sorcery, // 術
}

#[derive(Component)]
struct ArtsSelectionDialog;

#[derive(Component)]
struct ArtsTabButton {
    tab_type: ArtTypeTab,
}

#[derive(Component)]
struct ArtsListContainer;

#[derive(Component)]
struct ArtsListButton {
    arts_id: usize,
}

#[derive(Component)]
struct CloseArtsDialogButton;

#[derive(Component)]
struct RemoveArtsButton;

/// 技術スロットボタンのクリック処理
fn arts_slot_button_system(
    mut interaction_query: Query<
        (&Interaction, &ArtsSlotButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut prep_state: ResMut<PreparationState>,
    arts_db: Res<ArtsDatabase>,
) {
    for (interaction, slot_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                prep_state.selecting_arts_slot = Some(slot_button.slot_index);

                let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

                // 技術選択ダイアログを表示
                commands
                    .spawn((
                        ArtsSelectionDialog,
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
                                    width: Val::Px(700.0),
                                    height: Val::Px(600.0),
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
                                            Text::new("技術を選択"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 28.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));

                                        header
                                            .spawn(Node {
                                                flex_direction: FlexDirection::Row,
                                                column_gap: Val::Px(10.0),
                                                ..default()
                                            })
                                            .with_children(|buttons| {
                                                // 削除ボタン
                                                buttons
                                                    .spawn((
                                                        RemoveArtsButton,
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
                                                            red: 0.6,
                                                            green: 0.3,
                                                            blue: 0.3,
                                                            alpha: 1.0,
                                                        })),
                                                        BorderColor::all(Color::WHITE),
                                                    ))
                                                    .with_children(|btn| {
                                                        btn.spawn((
                                                            Text::new("削除"),
                                                            TextFont {
                                                                font: font.clone(),
                                                                font_size: 16.0,
                                                                ..default()
                                                            },
                                                            TextColor(Color::WHITE),
                                                        ));
                                                    });

                                                // 閉じるボタン
                                                buttons
                                                    .spawn((
                                                        CloseArtsDialogButton,
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
                                                            green: 0.4,
                                                            blue: 0.5,
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
                                    });

                                // タブ（基本・技・術）
                                dialog
                                    .spawn(Node {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Row,
                                        column_gap: Val::Px(5.0),
                                        margin: UiRect::bottom(Val::Px(10.0)),
                                        ..default()
                                    })
                                    .with_children(|tabs| {
                                        let tab_items = [
                                            (ArtTypeTab::Basic, "基本"),
                                            (ArtTypeTab::Skill, "技"),
                                            (ArtTypeTab::Sorcery, "術"),
                                        ];

                                        for (tab_type, label) in tab_items {
                                            let is_selected =
                                                prep_state.selected_art_tab == tab_type;
                                            tabs.spawn((
                                                ArtsTabButton { tab_type },
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
                                                    red: if is_selected { 0.4 } else { 0.25 },
                                                    green: if is_selected { 0.4 } else { 0.25 },
                                                    blue: if is_selected { 0.5 } else { 0.35 },
                                                    alpha: 1.0,
                                                })),
                                                BorderColor::all(if is_selected {
                                                    Color::srgb(1.0, 0.9, 0.6)
                                                } else {
                                                    Color::WHITE
                                                }),
                                            ))
                                            .with_children(|btn| {
                                                btn.spawn((
                                                    Text::new(label),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 18.0,
                                                        ..default()
                                                    },
                                                    TextColor(if is_selected {
                                                        Color::srgb(1.0, 0.9, 0.6)
                                                    } else {
                                                        Color::WHITE
                                                    }),
                                                ));
                                            });
                                        }
                                    });

                                // 技術リスト（スクロール可能）
                                dialog
                                    .spawn((
                                        ArtsListContainer,
                                        Node {
                                            width: Val::Percent(100.0),
                                            flex_grow: 1.0,
                                            flex_direction: FlexDirection::Column,
                                            row_gap: Val::Px(10.0),
                                            overflow: Overflow::clip_y(),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|list| {
                                        // 選択中のタブに合致するアーツのみを表示
                                        let target_art_type = match prep_state.selected_art_tab {
                                            ArtTypeTab::Basic => ArtType::Basic,
                                            ArtTypeTab::Skill => ArtType::Skill,
                                            ArtTypeTab::Sorcery => ArtType::Sorcery,
                                        };

                                        for art_data in arts_db
                                            .arts
                                            .iter()
                                            .filter(|a| a.art.art_type == target_art_type)
                                        {
                                            spawn_art_list_item(list, art_data, &font);
                                        }
                                    });
                            });
                    });
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

/// 技術リストボタンのクリック処理
fn arts_selection_dialog_system(
    mut interaction_query: Query<
        (&Interaction, &ArtsListButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<ArtsSelectionDialog>>,
    mut prep_state: ResMut<PreparationState>,
) {
    for (interaction, list_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(slot_index) = prep_state.selecting_arts_slot {
                    // 選択された技術をスロットに設定
                    if slot_index < prep_state.selected_arts.len() {
                        prep_state.selected_arts[slot_index] = list_button.arts_id;
                    } else {
                        prep_state.selected_arts.push(list_button.arts_id);
                    }

                    prep_state.selecting_arts_slot = None;

                    // ダイアログを閉じる
                    for entity in dialog_query.iter() {
                        commands.entity(entity).despawn();
                    }
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

/// 技術削除ボタンのクリック処理
fn close_arts_selection_dialog_system(
    mut interaction_query_close: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CloseArtsDialogButton>),
    >,
    mut interaction_query_remove: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<RemoveArtsButton>,
            Without<CloseArtsDialogButton>,
        ),
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<ArtsSelectionDialog>>,
    mut prep_state: ResMut<PreparationState>,
) {
    // 閉じるボタン
    for (interaction, mut color) in &mut interaction_query_close {
        match *interaction {
            Interaction::Pressed => {
                prep_state.selecting_arts_slot = None;
                for entity in dialog_query.iter() {
                    commands.entity(entity).despawn();
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.6,
                    alpha: 1.0,
                }));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.4,
                    green: 0.4,
                    blue: 0.5,
                    alpha: 1.0,
                }));
            }
        }
    }

    // 削除ボタン
    for (interaction, mut color) in &mut interaction_query_remove {
        match *interaction {
            Interaction::Pressed => {
                if let Some(slot_index) = prep_state.selecting_arts_slot {
                    // 該当スロットの技術を削除
                    if slot_index < prep_state.selected_arts.len() {
                        prep_state.selected_arts.remove(slot_index);
                    }
                }
                prep_state.selecting_arts_slot = None;
                for entity in dialog_query.iter() {
                    commands.entity(entity).despawn();
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.7,
                    green: 0.4,
                    blue: 0.4,
                    alpha: 1.0,
                }));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: 0.6,
                    green: 0.3,
                    blue: 0.3,
                    alpha: 1.0,
                }));
            }
        }
    }
}

/// 技術タブボタンのクリック処理
fn arts_tab_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &ArtsTabButton,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<ArtsSelectionDialog>>,
    mut prep_state: ResMut<PreparationState>,
    arts_db: Res<ArtsDatabase>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, tab_button, mut color, mut border) in &mut interaction_query {
        let is_selected = prep_state.selected_art_tab == tab_button.tab_type;

        match *interaction {
            Interaction::Pressed => {
                // タブ切り替え
                if prep_state.selected_art_tab != tab_button.tab_type {
                    prep_state.selected_art_tab = tab_button.tab_type;

                    // ダイアログを閉じて再生成
                    for entity in dialog_query.iter() {
                        commands.entity(entity).despawn();
                    }

                    // ダイアログを再生成
                    spawn_arts_selection_dialog(
                        &mut commands,
                        &asset_server,
                        &prep_state,
                        &arts_db,
                    );
                }
            }
            Interaction::Hovered => {
                if !is_selected {
                    *color = BackgroundColor(Color::from(LinearRgba {
                        red: 0.35,
                        green: 0.35,
                        blue: 0.45,
                        alpha: 1.0,
                    }));
                }
            }
            Interaction::None => {
                *color = BackgroundColor(Color::from(LinearRgba {
                    red: if is_selected { 0.4 } else { 0.25 },
                    green: if is_selected { 0.4 } else { 0.25 },
                    blue: if is_selected { 0.5 } else { 0.35 },
                    alpha: 1.0,
                }));
                *border = BorderColor::all(if is_selected {
                    Color::srgb(1.0, 0.9, 0.6)
                } else {
                    Color::WHITE
                });
            }
        }
    }
}

/// 技術リストのアイテムをスポーンするヘルパー関数
fn spawn_art_list_item(list: &mut ChildSpawnerCommands, art_data: &ArtsData, font: &Handle<Font>) {
    list.spawn((
        ArtsListButton {
            arts_id: art_data.id,
        },
        Button,
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
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
    .with_children(|btn| {
        // 技術名
        btn.spawn((
            Text::new(&art_data.name),
            TextFont {
                font: font.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.9, 0.6)),
        ));

        // コスト
        btn.spawn((
            Text::new(format!(
                "SP: {} / スタミナ: {}",
                art_data.art.sp_cost, art_data.art.stamina_cost
            )),
            TextFont {
                font: font.clone(),
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));

        // 必要能力
        let req = &art_data.art.requirement;
        let mut req_parts = Vec::new();
        if req.strength > 0 {
            req_parts.push(format!("筋力{}", req.strength));
        }
        if req.dexterity > 0 {
            req_parts.push(format!("技量{}", req.dexterity));
        }
        if req.intelligence > 0 {
            req_parts.push(format!("知力{}", req.intelligence));
        }
        if req.faith > 0 {
            req_parts.push(format!("信仰{}", req.faith));
        }
        if req.arcane > 0 {
            req_parts.push(format!("神秘{}", req.arcane));
        }
        if req.agility > 0 {
            req_parts.push(format!("敏捷{}", req.agility));
        }

        if !req_parts.is_empty() {
            btn.spawn((
                Text::new(format!("必要: {}", req_parts.join(", "))),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.8, 1.0)),
            ));
        }

        // 使用可能武器種（Allの場合は表示しない）
        if let ArtUsableWeapon::Specific(weapon_kinds) = &art_data.art.usable_weapon {
            let weapon_names: Vec<&str> = weapon_kinds
                .iter()
                .map(|kind| weapon_kind_to_string(kind))
                .collect();
            btn.spawn((
                Text::new(format!("武器: {}", weapon_names.join(", "))),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.6, 1.0)),
            ));
        }
    });
}

/// 武器種を日本語に変換
fn weapon_kind_to_string(kind: &WeaponKind) -> &'static str {
    match kind {
        WeaponKind::StraightSword => "直剣",
        WeaponKind::Greatsword => "大剣",
        WeaponKind::Spear => "槍",
        WeaponKind::Axe => "斧",
        WeaponKind::Hammer => "ハンマー",
        WeaponKind::Bow => "弓",
        WeaponKind::Crossbow => "クロスボウ",
        WeaponKind::Staff => "杖",
        WeaponKind::Shield => "盾",
    }
}

/// 技術選択ダイアログをスポーンするヘルパー関数
fn spawn_arts_selection_dialog(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    prep_state: &ResMut<PreparationState>,
    arts_db: &Res<ArtsDatabase>,
) {
    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    commands
        .spawn((
            ArtsSelectionDialog,
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
                        width: Val::Px(700.0),
                        height: Val::Px(600.0),
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
                                Text::new("技術を選択"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 28.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));

                            header
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(10.0),
                                    ..default()
                                })
                                .with_children(|buttons| {
                                    // 削除ボタン
                                    buttons
                                        .spawn((
                                            RemoveArtsButton,
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
                                                red: 0.6,
                                                green: 0.3,
                                                blue: 0.3,
                                                alpha: 1.0,
                                            })),
                                            BorderColor::all(Color::WHITE),
                                        ))
                                        .with_children(|btn| {
                                            btn.spawn((
                                                Text::new("削除"),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                            ));
                                        });

                                    // 閉じるボタン
                                    buttons
                                        .spawn((
                                            CloseArtsDialogButton,
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
                                                green: 0.4,
                                                blue: 0.5,
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
                        });

                    // タブ（基本・技・術）
                    dialog
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(5.0),
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        })
                        .with_children(|tabs| {
                            let tab_items = [
                                (ArtTypeTab::Basic, "基本"),
                                (ArtTypeTab::Skill, "技"),
                                (ArtTypeTab::Sorcery, "術"),
                            ];

                            for (tab_type, label) in tab_items {
                                let is_selected = prep_state.selected_art_tab == tab_type;
                                tabs.spawn((
                                    ArtsTabButton { tab_type },
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
                                        red: if is_selected { 0.4 } else { 0.25 },
                                        green: if is_selected { 0.4 } else { 0.25 },
                                        blue: if is_selected { 0.5 } else { 0.35 },
                                        alpha: 1.0,
                                    })),
                                    BorderColor::all(if is_selected {
                                        Color::srgb(1.0, 0.9, 0.6)
                                    } else {
                                        Color::WHITE
                                    }),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new(label),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(if is_selected {
                                            Color::srgb(1.0, 0.9, 0.6)
                                        } else {
                                            Color::WHITE
                                        }),
                                    ));
                                });
                            }
                        });

                    // 技術リスト（スクロール可能）
                    dialog
                        .spawn((
                            ArtsListContainer,
                            Node {
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(10.0),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                        ))
                        .with_children(|list| {
                            // 選択中のタブに合致するアーツのみを表示
                            let target_art_type = match prep_state.selected_art_tab {
                                ArtTypeTab::Basic => ArtType::Basic,
                                ArtTypeTab::Skill => ArtType::Skill,
                                ArtTypeTab::Sorcery => ArtType::Sorcery,
                            };

                            for art_data in arts_db
                                .arts
                                .iter()
                                .filter(|a| a.art.art_type == target_art_type)
                            {
                                spawn_art_list_item(list, art_data, &font);
                            }
                        });
                });
        });
}

/// エラーメッセージを更新するシステム
fn update_error_message_system(mut prep_state: ResMut<PreparationState>, time: Res<Time>) {
    if prep_state.error_message.is_some() {
        prep_state.error_message_timer -= time.delta_secs();
        if prep_state.error_message_timer <= 0.0 {
            prep_state.error_message = None;
            prep_state.error_message_timer = 0.0;
        }
    }
}

/// エラーメッセージを画面に表示するシステム
fn display_error_message_system(
    mut commands: Commands,
    prep_state: Res<PreparationState>,
    existing_panel: Query<Entity, With<ErrorMessagePanel>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/x12y16pxMaruMonica.ttf");

    if prep_state.error_message.is_some() && !prep_state.is_changed() {
        // メッセージが変わっていない場合は何もしない
        return;
    }

    // 既存のパネルを削除
    for entity in existing_panel.iter() {
        commands.entity(entity).despawn();
    }

    // メッセージがある場合は表示
    if let Some(ref message) = prep_state.error_message {
        commands
            .spawn((
                ErrorMessagePanel,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(600.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::from(LinearRgba {
                    red: 0.8,
                    green: 0.2,
                    blue: 0.2,
                    alpha: 0.95,
                })),
                BorderColor::all(Color::WHITE),
                ZIndex(200),
                Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            ))
            .with_children(|panel| {
                panel.spawn((
                    ErrorMessageText,
                    Text::new(message.clone()),
                    TextFont {
                        font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
    }
}
