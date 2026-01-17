mod battle;
mod game;
mod player;
mod types;

use crate::game::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PreparationPlugin)
        .add_plugins(InBattlePlugin)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}

// ================== Components & Resources ==================
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Preparation, // 準備画面
    Battle, // 戦闘画面
}

// ================== Setup ==================
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
