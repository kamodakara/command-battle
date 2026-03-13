pub mod events;
pub mod logic;
pub mod ui;

use bevy::prelude::*;

pub struct InBattlePlugin;

impl Plugin for InBattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(logic::BattleLogicPlugin)
            .add_plugins(ui::BattleUiPlugin);
    }
}
