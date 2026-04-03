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
        .import_from_str(include_str!("../../assets/data/weapons.json"))
        .expect("weapons.json のパースに失敗しました");

    dm.armor
        .import_from_str(include_str!("../../assets/data/armors.json"))
        .expect("armors.json のパースに失敗しました");

    dm.art
        .import_from_str(include_str!("../../assets/data/arts.json"))
        .expect("arts.json のパースに失敗しました");

    dm.karma_card
        .import_from_str(include_str!("../../assets/data/karma_cards.json"))
        .expect("karma_cards.json のパースに失敗しました");

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
