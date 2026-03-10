use bevy::prelude::*;

use crate::data::DataManager;

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

    commands.insert_resource(dm);
}
