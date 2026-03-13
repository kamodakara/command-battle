use bevy::prelude::*;
use super::resources::*;
use super::battle_factory::*;
use crate::game::{ArtsDatabase, EquipmentDatabase, PreparationState};

pub fn setup_battle_logic(
    mut commands: Commands,
    prep_state: Res<PreparationState>,
    equipment_db: Res<EquipmentDatabase>,
    arts_db: Res<ArtsDatabase>,
) {
    commands.insert_resource(BattlePhase::DecideEnemyConduct);
    commands.insert_resource(Turn(1));
    commands.insert_resource(EnemyPlannedAction(None));

    let (basic_arts, equipped_weapons, battle_weapons) =
        create_equipped_weapons_from_preparation(&prep_state, &equipment_db, &arts_db);
    commands.insert_resource(PlayerBasicArts(basic_arts));
    commands.insert_resource(equipped_weapons);
    commands.insert_resource(BattleResource(create_battle_from_preparation(
        &prep_state,
        &equipment_db,
        battle_weapons,
    )));
}
