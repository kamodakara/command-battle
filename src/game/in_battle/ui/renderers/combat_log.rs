use bevy::prelude::*;
use super::super::resources::{CombatLog, CombatLogExpanded};
use super::super::super::logic::resources::BattlePhase;

// ─── コンポーネント定義 ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct UiMessage;

#[derive(Component)]
pub struct UiPhase;

#[derive(Component)]
pub struct UiCombatLogToggleButton;

#[derive(Component)]
pub struct UiCombatLogToggleButtonText;

// ─── 描画システム ────────────────────────────────────────────────────────────

pub fn render_message(
    log: Res<CombatLog>,
    expanded: Res<CombatLogExpanded>,
    mut msg_q: Query<&mut Text, With<UiMessage>>,
) {
    let Ok(mut msg) = msg_q.single_mut() else {
        return;
    };
    let max_lines = if expanded.0 { 15usize } else { 5usize };
    let start = if log.0.len() > max_lines {
        log.0.len() - max_lines
    } else {
        0
    };
    let mut s = String::new();
    for line in &log.0[start..] {
        s.push_str(line);
        s.push('\n');
    }
    msg.0 = s;
}

pub fn render_phase(
    phase: Res<BattlePhase>,
    mut phase_q: Query<&mut Text, With<UiPhase>>,
) {
    let Ok(mut phase_text) = phase_q.single_mut() else {
        return;
    };
    phase_text.0 = match *phase {
        BattlePhase::DecideEnemyConduct => "フェーズ: 敵の行動決定中".to_string(),
        BattlePhase::AwaitCommand => "フェーズ: 行動を選択してください".to_string(),
        BattlePhase::TurnEnd => "フェーズ: ターン終了".to_string(),
        BattlePhase::Finished => "フェーズ: 終了".to_string(),
    };
}

pub fn handle_combat_log_toggle(
    mut expanded: ResMut<CombatLogExpanded>,
    btn_q: Query<&Interaction, (Changed<Interaction>, With<UiCombatLogToggleButton>)>,
) {
    for interaction in btn_q.iter() {
        if *interaction == Interaction::Pressed {
            expanded.0 = !expanded.0;
        }
    }
}

pub fn render_combat_log_toggle_button(
    expanded: Res<CombatLogExpanded>,
    mut text_q: Query<&mut Text, With<UiCombatLogToggleButtonText>>,
) {
    if !expanded.is_changed() {
        return;
    }
    let Ok(mut text) = text_q.single_mut() else {
        return;
    };
    text.0 = if expanded.0 { "▲ 閉じる".to_string() } else { "▼ 開く".to_string() };
}
