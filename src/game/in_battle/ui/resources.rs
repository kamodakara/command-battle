use bevy::prelude::*;
use crate::fundamental::Art;
use std::sync::Arc;
use super::super::events::ConsecutiveCommandEntry;

/// 戦闘ログ（BattleLogEventで更新、UI専有）
#[derive(Resource, Default)]
pub struct CombatLog(pub Vec<String>);

/// 敵ダメージポップアップ（DamageHpEventで更新、UI専有）
#[derive(Resource, Default)]
pub struct EnemyDamagePopup {
    pub amount: i32,
    pub timer: f32,
}

/// カルマカードUIの再描画フラグ（UI専有）
#[derive(Resource, Default)]
pub struct KarmaCardsNeedsRedraw(pub bool);

/// 敵の次の行動表示（EnemyActionPlannedEventで更新、UI専有）
#[derive(Resource, Default)]
pub struct EnemyNextActionDisplay(pub String);

/// 連続コマンドキュー（UI専有、確定後にLogicへイベントで渡す）
#[derive(Resource, Default)]
pub struct ConsecutiveCommands {
    pub commands: Vec<ConsecutiveCommandEntry>,
}

// ─── メニューUI状態 ──────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Eq)]
pub enum ActionMenuState {
    ConsecutiveConfirm,
    ConsecutiveInput,
    ConsecutiveBasicArts,
    ConsecutiveWeaponArts { weapon_idx: usize },
    ConfirmAllCommands,
}

/// メニュー選択リソース（UI専有）
#[derive(Resource)]
pub struct ActionMenuSelection {
    pub menu_state: ActionMenuState,
}

impl Default for ActionMenuSelection {
    fn default() -> Self {
        ActionMenuSelection {
            menu_state: ActionMenuState::ConsecutiveInput,
        }
    }
}

impl ActionMenuSelection {
    pub fn confirm(&mut self) {
        self.menu_state = ActionMenuState::ConsecutiveConfirm;
    }
    pub fn confirm_all(&mut self) {
        self.menu_state = ActionMenuState::ConfirmAllCommands;
    }
    pub fn input(&mut self) {
        self.menu_state = ActionMenuState::ConsecutiveInput;
    }
    pub fn select_category_basic(&mut self) {
        self.menu_state = ActionMenuState::ConsecutiveBasicArts;
    }
    pub fn select_category_weapon(&mut self, weapon_idx: usize) {
        self.menu_state = ActionMenuState::ConsecutiveWeaponArts { weapon_idx };
    }
}

/// 武器アーツ選択時のアーツ情報保持（UIのみで使用）
#[derive(Clone)]
pub struct PendingWeaponArt {
    pub art: Arc<Art>,
    pub weapon_index: usize,
}
