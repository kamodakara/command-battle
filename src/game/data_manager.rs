use bevy::prelude::*;

use crate::data::DataManager;

use super::preparation::{ArmorData, ArtsData, ArtsDatabase, EquipmentDatabase, WeaponData};

impl Resource for DataManager {}

pub struct DataManagerPlugin;

impl Plugin for DataManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_data_manager);
    }
}

fn setup_data_manager(mut commands: Commands) {
    let mut dm = DataManager::default();

    dm.weapon
        .import_from_file("assets/data/weapons.json")
        .expect("assets/data/weapons.json の読み込みに失敗しました");

    dm.armor
        .import_from_file("assets/data/armors.json")
        .expect("assets/data/armors.json の読み込みに失敗しました");

    dm.art
        .import_from_file("assets/data/arts.json")
        .expect("assets/data/arts.json の読み込みに失敗しました");

    dm.karma_card
        .import_from_file("assets/data/karma_cards.json")
        .expect("assets/data/karma_cards.json の読み込みに失敗しました");

    // in_battle.rs との互換性のため、DataManager からレガシーリソースを構築して挿入する
    let equipment_db = EquipmentDatabase {
        weapons: dm
            .weapon
            .find_all()
            .into_iter()
            .map(|r| WeaponData {
                id: r.id,
                name: r.data.name.clone(),
                weapon: r.data.clone(),
            })
            .collect(),
        armors: dm
            .armor
            .find_all()
            .into_iter()
            .map(|r| ArmorData {
                id: r.id,
                name: r.data.name.clone(),
                armor: r.data.clone(),
            })
            .collect(),
    };

    let arts_db = ArtsDatabase {
        arts: dm
            .art
            .find_all()
            .into_iter()
            .map(|r| ArtsData {
                id: r.id,
                name: r.data.name.clone(),
                art: r.data.clone(),
            })
            .collect(),
    };

    commands.insert_resource(equipment_db);
    commands.insert_resource(arts_db);
    commands.insert_resource(dm);
}
