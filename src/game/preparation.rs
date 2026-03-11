use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use super::*;
use crate::data::DataManager;
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
    pub equipped_weapon1: Option<u32>,
    pub equipped_weapon2: Option<u32>,
    pub equipped_armor1: Option<u32>,
    pub equipped_armor2: Option<u32>,
    pub equipped_armor3: Option<u32>,
    pub equipped_armor4: Option<u32>,
    pub equipped_armor5: Option<u32>,
    pub equipped_armor6: Option<u32>,
    pub equipped_armor7: Option<u32>,
    pub equipped_armor8: Option<u32>,
    pub selecting_slot: Option<EquipmentSlot>,
    pub selected_arts: Vec<u32>, // 選択された技術のID (最大8つ)
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
    pub id: u32,
    pub name: String,
    pub weapon: Weapon,
}

#[derive(Clone)]
pub struct ArmorData {
    pub id: u32,
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
    pub id: u32,
    pub name: String,
    pub art: Art,
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
    dm: Res<DataManager>,
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
                    build_status_content(parent, font_clone.clone(), &prep_state, &dm);
                }
                MenuType::Equipment => {
                    build_equipment_content(parent, font_clone.clone(), &prep_state, &dm);
                }
                MenuType::Arts => {
                    build_arts_content(parent, font_clone.clone(), &prep_state, &dm);
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
    dm: &DataManager,
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
                    if let Some(weapon_record) = dm.weapon.find_by_id(weapon_id) {
                        let weapon_performance = weapon_record.data.performance(&current_ability);
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
                    if let Some(weapon_record) = dm.weapon.find_by_id(weapon_id) {
                        let weapon_performance = weapon_record.data.performance(&current_ability);
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
                    if let Some(weapon_record) = dm.weapon.find_by_id(weapon_id) {
                        let weapon_performance = weapon_record.data.performance(&current_ability);
                        weapon_performance.final_sorcery_power()
                    } else {
                        WeaponPerformance::unarmed_weapon_performance().final_sorcery_power()
                    }
                } else {
                    WeaponPerformance::unarmed_weapon_performance().final_sorcery_power()
                };

                // 武器2の術力を計算
                let weapon2_sorcery_power = if let Some(weapon_id) = prep_state.equipped_weapon2 {
                    if let Some(weapon_record) = dm.weapon.find_by_id(weapon_id) {
                        let weapon_performance = weapon_record.data.performance(&current_ability);
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
    dm: &DataManager,
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
        .and_then(|id| dm.weapon.find_by_id(id));
    let weapon2_data = prep_state
        .equipped_weapon2
        .and_then(|id| dm.weapon.find_by_id(id));

    use crate::data::WeaponRecord;
    let equipment_slots: Vec<(&str, EquipmentSlot, Option<String>, Option<&WeaponRecord>)> = vec![
        (
            "右手武器",
            EquipmentSlot::Weapon1,
            weapon1_data.map(|w| w.data.name.clone()),
            weapon1_data,
        ),
        (
            "左手武器",
            EquipmentSlot::Weapon2,
            weapon2_data.map(|w| w.data.name.clone()),
            weapon2_data,
        ),
        (
            "防具1",
            EquipmentSlot::Armor1,
            prep_state
                .equipped_armor1
                .and_then(|id| dm.armor.find_by_id(id).map(|a| a.data.name.clone())),
            None,
        ),
        (
            "防具2",
            EquipmentSlot::Armor2,
            prep_state
                .equipped_armor2
                .and_then(|id| dm.armor.find_by_id(id).map(|a| a.data.name.clone())),
            None,
        ),
        (
            "防具3",
            EquipmentSlot::Armor3,
            prep_state
                .equipped_armor3
                .and_then(|id| dm.armor.find_by_id(id).map(|a| a.data.name.clone())),
            None,
        ),
        (
            "防具4",
            EquipmentSlot::Armor4,
            prep_state
                .equipped_armor4
                .and_then(|id| dm.armor.find_by_id(id).map(|a| a.data.name.clone())),
            None,
        ),
        (
            "防具5",
            EquipmentSlot::Armor5,
            prep_state
                .equipped_armor5
                .and_then(|id| dm.armor.find_by_id(id).map(|a| a.data.name.clone())),
            None,
        ),
        (
            "防具6",
            EquipmentSlot::Armor6,
            prep_state
                .equipped_armor6
                .and_then(|id| dm.armor.find_by_id(id).map(|a| a.data.name.clone())),
            None,
        ),
        (
            "防具7",
            EquipmentSlot::Armor7,
            prep_state
                .equipped_armor7
                .and_then(|id| dm.armor.find_by_id(id).map(|a| a.data.name.clone())),
            None,
        ),
        (
            "防具8",
            EquipmentSlot::Armor8,
            prep_state
                .equipped_armor8
                .and_then(|id| dm.armor.find_by_id(id).map(|a| a.data.name.clone())),
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
                                        let is_usable = if let Some(weapon_record) = weapon_data {
                                            let not_enough_abilities = weapon_record
                                                .data
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
                        weapon1: prep_state
                            .equipped_weapon1
                            .and_then(|id| dm.weapon.find_by_id(id).map(|r| r.data.clone())),
                        weapon2: prep_state
                            .equipped_weapon2
                            .and_then(|id| dm.weapon.find_by_id(id).map(|r| r.data.clone())),
                        armor1: prep_state
                            .equipped_armor1
                            .and_then(|id| dm.armor.find_by_id(id).map(|r| r.data.clone())),
                        armor2: prep_state
                            .equipped_armor2
                            .and_then(|id| dm.armor.find_by_id(id).map(|r| r.data.clone())),
                        armor3: prep_state
                            .equipped_armor3
                            .and_then(|id| dm.armor.find_by_id(id).map(|r| r.data.clone())),
                        armor4: prep_state
                            .equipped_armor4
                            .and_then(|id| dm.armor.find_by_id(id).map(|r| r.data.clone())),
                        armor5: prep_state
                            .equipped_armor5
                            .and_then(|id| dm.armor.find_by_id(id).map(|r| r.data.clone())),
                        armor6: prep_state
                            .equipped_armor6
                            .and_then(|id| dm.armor.find_by_id(id).map(|r| r.data.clone())),
                        armor7: prep_state
                            .equipped_armor7
                            .and_then(|id| dm.armor.find_by_id(id).map(|r| r.data.clone())),
                        armor8: prep_state
                            .equipped_armor8
                            .and_then(|id| dm.armor.find_by_id(id).map(|r| r.data.clone())),
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
                                    if let Some(weapon_record) = weapon1_data {
                                        let performance =
                                            weapon_record.data.performance(&current_ability);
                                        build_weapon_performance_display(
                                            weapon_col,
                                            font.clone(),
                                            "右手武器",
                                            &weapon_record.data,
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
                                    if let Some(weapon_record) = weapon2_data {
                                        let performance =
                                            weapon_record.data.performance(&current_ability);
                                        build_weapon_performance_display(
                                            weapon_col,
                                            font.clone(),
                                            "左手武器",
                                            &weapon_record.data,
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
                        if let Some(armor_record) = dm.armor.find_by_id(*armor_id) {
                            equipment_defense.add(&armor_record.data.defense);
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
    dm: &DataManager,
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
                            .and_then(|&art_id| dm.art.find_by_id(art_id));

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
                                if let Some(art_record) = selected_art {
                                    // 必要能力を満たしているかチェック
                                    let meets_requirement = check_art_requirement(
                                        prep_state,
                                        &art_record.data.requirement,
                                    );

                                    // 技術名（必要能力が足りない場合は×を表示）
                                    let name_text = if meets_requirement {
                                        format!("{}. {}", slot_index + 1, art_record.data.name)
                                    } else {
                                        format!("{}. {} ×", slot_index + 1, art_record.data.name)
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
                                        art_record.data.sp_cost, art_record.data.stamina_cost
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
                            for art_record in dm.art.find_all() {
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
                                        Text::new(&art_record.data.name),
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
                                            art_record.data.sp_cost, art_record.data.stamina_cost
                                        )),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                    ));

                                    // 必要能力
                                    let req = &art_record.data.requirement;
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
    dm: Res<DataManager>,
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
                        build_equipment_selection_dialog(parent, font, equipment_button.slot, &dm);
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

// ================== 装備選択ダイアログ ==================

#[derive(Component)]
struct EquipmentSelectionDialog;

#[derive(Component)]
struct EquipmentListButton {
    equipment_id: u32,
}

#[derive(Component)]
struct CloseDialogButton;

fn build_equipment_selection_dialog(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    slot: EquipmentSlot,
    dm: &DataManager,
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
                                    for weapon_record in dm.weapon.find_all() {
                                        list.spawn((
                                            EquipmentListButton {
                                                equipment_id: weapon_record.id,
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
                                                    Text::new(&weapon_record.data.name),
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
                                    for armor_record in dm.armor.find_all() {
                                        list.spawn((
                                            EquipmentListButton {
                                                equipment_id: armor_record.id,
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
                                                    Text::new(&armor_record.data.name),
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
    dm: Res<DataManager>,
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
                            if let Some(armor_record) =
                                dm.armor.find_by_id(list_button.equipment_id)
                            {
                                // 現在の装備状態からEquipmentを構築
                                let current_equipment = Equipment {
                                    weapon1: prep_state.equipped_weapon1.and_then(|id| {
                                        dm.weapon.find_by_id(id).map(|r| r.data.clone())
                                    }),
                                    weapon2: prep_state.equipped_weapon2.and_then(|id| {
                                        dm.weapon.find_by_id(id).map(|r| r.data.clone())
                                    }),
                                    armor1: prep_state.equipped_armor1.and_then(|id| {
                                        dm.armor.find_by_id(id).map(|r| r.data.clone())
                                    }),
                                    armor2: prep_state.equipped_armor2.and_then(|id| {
                                        dm.armor.find_by_id(id).map(|r| r.data.clone())
                                    }),
                                    armor3: prep_state.equipped_armor3.and_then(|id| {
                                        dm.armor.find_by_id(id).map(|r| r.data.clone())
                                    }),
                                    armor4: prep_state.equipped_armor4.and_then(|id| {
                                        dm.armor.find_by_id(id).map(|r| r.data.clone())
                                    }),
                                    armor5: prep_state.equipped_armor5.and_then(|id| {
                                        dm.armor.find_by_id(id).map(|r| r.data.clone())
                                    }),
                                    armor6: prep_state.equipped_armor6.and_then(|id| {
                                        dm.armor.find_by_id(id).map(|r| r.data.clone())
                                    }),
                                    armor7: prep_state.equipped_armor7.and_then(|id| {
                                        dm.armor.find_by_id(id).map(|r| r.data.clone())
                                    }),
                                    armor8: prep_state.equipped_armor8.and_then(|id| {
                                        dm.armor.find_by_id(id).map(|r| r.data.clone())
                                    }),
                                };

                                // 装備可能かチェック
                                if current_equipment.is_equippable(&armor_record.data) {
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
    arts_id: u32,
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
    dm: Res<DataManager>,
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

                                        for art_record in
                                            dm.art.find_many(|r| r.data.art_type == target_art_type)
                                        {
                                            spawn_art_list_item(list, art_record, &font);
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
    dm: Res<DataManager>,
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
                    spawn_arts_selection_dialog(&mut commands, &asset_server, &prep_state, &dm);
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
fn spawn_art_list_item(
    list: &mut ChildSpawnerCommands,
    art_record: &crate::data::ArtRecord,
    font: &Handle<Font>,
) {
    list.spawn((
        ArtsListButton {
            arts_id: art_record.id,
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
            Text::new(&art_record.data.name),
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
                art_record.data.sp_cost, art_record.data.stamina_cost
            )),
            TextFont {
                font: font.clone(),
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));

        // 必要能力
        let req = &art_record.data.requirement;
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
        if let ArtUsableWeapon::Specific(weapon_kinds) = &art_record.data.usable_weapon {
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
    dm: &Res<DataManager>,
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

                            for art_record in
                                dm.art.find_many(|r| r.data.art_type == target_art_type)
                            {
                                spawn_art_list_item(list, art_record, &font);
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
