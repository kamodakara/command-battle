use bevy::prelude::*;
use crate::fundamental::Art;
use std::sync::Arc;
use super::super::events::ConsecutiveCommandEntry;

/// 戦闘ログ（BattleLogEventで更新、UI専有）
#[derive(Resource, Default)]
pub struct CombatLog(pub Vec<String>);

/// メッセージ送りキュー（UI専有）
/// BattleLogEvent を一旦ここに積み、タイマーで1件ずつ CombatLog へ移す
#[derive(Resource, Default)]
pub struct MessageQueue {
    pub pending: std::collections::VecDeque<String>,
    /// 次のメッセージを表示するまでの残り秒数
    pub timer: f32,
}

/// 敵ダメージポップアップ（DamageHpEventで更新、UI専有）
#[derive(Resource, Default)]
pub struct EnemyDamagePopup {
    pub amount: i32,
    pub timer: f32,
}

/// カルマカードUIの再描画フラグ（UI専有）
#[derive(Resource, Default)]
pub struct KarmaCardsNeedsRedraw(pub bool);

/// ログ展開状態（UI専有）
#[derive(Resource, Default)]
pub struct CombatLogExpanded(pub bool);

/// カルマダイアログの表示状態（UI専有）
#[derive(Resource, Default, PartialEq, Clone)]
pub enum KarmaDialogState {
    #[default]
    Closed,
    DrawPile,
    DiscardPile,
}

/// 連続コマンドキュー（UI専有、確定後にLogicへイベントで渡す）
#[derive(Resource, Default)]
pub struct ConsecutiveCommands {
    pub commands: Vec<ConsecutiveCommandEntry>,
}

// ─── メニューUI状態 ──────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Eq)]
pub enum ActionMenuState {
    ConsecutiveInput,
    ConsecutiveBasicArts,
    ConsecutiveWeaponArts { weapon_idx: usize },
    ConfirmAllCommands,
    AutoExecuting,
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
    pub fn confirm_all(&mut self) {
        self.menu_state = ActionMenuState::ConfirmAllCommands;
    }
    pub fn auto_executing(&mut self) {
        self.menu_state = ActionMenuState::AutoExecuting;
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

/// ターン行動ボード（UI専有）
/// 3スロット分のプレイヤー・敵行動名と実行済みフラグを保持する
#[derive(Resource, Default)]
pub struct TurnActionBoard {
    pub player_actions: [Option<String>; 3],
    /// None = まだ行動していない（"？？？"表示）
    pub enemy_actions: [Option<String>; 3],
    pub executed: [bool; 3],
    slot_conduct_counts: [u32; 3],
    /// メッセージ送り完了後に適用するリセット予約（ヒントスロット, ヒント行動名）
    pending_reset: Option<(usize, String)>,
    /// 行動セットに設定されたヒント文字列（ブレイクゲージ下に表示）
    pub enemy_hint: Option<String>,
}

impl TurnActionBoard {
    pub fn reset(&mut self) {
        *self = TurnActionBoard::default();
    }

    /// ターン開始時に呼ぶ。メッセージ送り完了後にリセット＋ヒント表示を予約する。
    pub fn schedule_reset(&mut self, hint_idx: usize, hint_name: String, enemy_hint: Option<String>) {
        self.pending_reset = Some((hint_idx, hint_name));
        self.enemy_hint = enemy_hint;
    }

    /// メッセージ送りが完了したタイミングで呼ぶ。予約があればリセットを適用する。
    pub fn apply_pending_reset_if_any(&mut self) {
        if let Some((hint_idx, hint_name)) = self.pending_reset.take() {
            let saved_hint = self.enemy_hint.take();
            self.reset();
            self.enemy_actions[hint_idx] = Some(hint_name);
            self.enemy_hint = saved_hint;
        }
    }

    /// 1conductイベントが来たときに呼ぶ。
    /// `enemy_art_name` は敵が行動者のときのみ Some を渡す。
    pub fn on_conduct(&mut self, action_index: usize, enemy_art_name: Option<&str>) {
        if action_index >= 3 {
            return;
        }
        if let Some(name) = enemy_art_name {
            self.enemy_actions[action_index] = Some(name.to_string());
        }
        self.slot_conduct_counts[action_index] += 1;
        if self.slot_conduct_counts[action_index] >= 2 {
            self.executed[action_index] = true;
        }
    }
}

/// 武器アーツ選択時のアーツ情報保持（UIのみで使用）
#[derive(Clone)]
pub struct PendingWeaponArt {
    pub art: Arc<Art>,
    pub weapon_index: usize,
}
