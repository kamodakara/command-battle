use bevy::prelude::*;
use crate::fundamental::Art;
use std::sync::Arc;

// 戦闘画面全体のマーカー（クリーンアップ用）
#[derive(Component)]
pub struct BattleScreen;

// 戻るボタン
#[derive(Component)]
pub struct BackToPreparationButton;

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct UiStatus;

#[derive(Component)]
pub struct UiPhase;

#[derive(Component)]
pub struct UiLog;

// 有効値（コマンド別表示用）
#[derive(Component)]
pub struct UiEffAttack;
#[derive(Component)]
pub struct UiEffSkill;
#[derive(Component)]
pub struct UiEffHeal;
#[derive(Component)]
pub struct UiEffDefend;

//
#[derive(Component)]
pub struct UiBackground;

#[derive(Component)]
pub struct UiPlayerStatus;
#[derive(Component)]
pub struct UiHpText;
#[derive(Component)]
pub struct UiHpGaugeFill;
#[derive(Component)]
pub struct UiStaText;
#[derive(Component)]
pub struct UiStaGaugeFill;
#[derive(Component)]
pub struct UiSpText;
#[derive(Component)]
pub struct UiSpGaugeFill;
#[derive(Component)]
pub struct UiTranceText;
#[derive(Component)]
pub struct UiTranceGaugeFill;
#[derive(Component)]
pub struct UiTranceLevelText;
#[derive(Component)]
pub struct UiTranceEffectText;
#[derive(Component)]
pub struct UiKarmaCardsContainer;

#[derive(Component)]
pub struct UiEnemy;
#[derive(Component)]
pub struct UiEnemyStatus;

// UiEnemy 内部の更新ターゲット
#[derive(Component)]
pub struct UiEnemyHpGaugeFill;
#[derive(Component)]
pub struct UiEnemyBreakGaugeFill;
#[derive(Component)]
pub struct UiEnemyBreakLabel; // 「ブレイク中」表示用
#[derive(Component)]
pub struct UiEnemyNextActionText; // 「次の行動: ...」

// 敵ダメージ表示テキスト（HPゲージの横に一時表示）
#[derive(Component)]
pub struct UiEnemyDamageText;
#[derive(Component)]
pub struct UiMessage;

// 行動選択メニュー用コンポーネント
#[derive(Component)]
pub struct UiActionMenu;
#[derive(Component)]
pub struct UiActionMenuContainer;
// メニューアイテム（ボタン）
#[derive(Component)]
pub struct ActionMenuItem {
    pub item_type: ActionMenuItemType,
}
#[derive(Clone)]
pub enum ActionMenuItemType {
    Category(ActionMenuCategory),
    Art(Arc<Art>),
    ConsecutiveAction(ConsecutiveActionType), // 連続コマンド関連のアクション
}
#[derive(Clone, PartialEq, Eq)]
pub enum ActionMenuCategory {
    Basic,
    Weapon(usize), // 武器インデックス
    Back,          // 戻る
}
// 連続コマンド確認画面の選択肢
#[derive(Clone, PartialEq, Eq)]
pub enum ConsecutiveActionType {
    Execute,       // 連続コマンドを実行
    Reenter,       // コマンド入力しなおし
    FinishInput,   // 入力完了（1〜2ターン分で終了）
    ConfirmAll,    // 3つのコマンド選択を確定して実行
    ReselectThird, // 3つ目のコマンドを再選択
}

#[derive(Component)]
pub struct UiCommand;
#[derive(Component)]
pub struct UiCommandHelp;

// ================== Boss Slain Banner ==================
#[derive(Component)]
pub struct BossSlainText; // ボス撃破表示用

#[derive(Component)]
pub struct BossSlainBanner {
    pub elapsed: f32,
    pub phase: BannerPhase,
}

// バナー背面の黒帯（グラデーション）
#[derive(Component)]
pub struct BossSlainBackdrop;
#[derive(Component)]
pub struct BossSlainBackdropCenter; // 中央の帯（不透明）
#[derive(Component)]
pub struct BossSlainBackdropRow(pub u8); // グラデーション行（0=最上段）

pub enum BannerPhase {
    FadeIn,
    Hold,
    FadeOut,
}
