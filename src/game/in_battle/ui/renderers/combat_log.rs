use bevy::prelude::*;
use super::super::resources::CombatLog;
use super::super::super::logic::resources::BattlePhase;

// ─── コンポーネント定義 ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct UiMessage;

#[derive(Component)]
pub struct UiPhase;

// ─── 描画システム ────────────────────────────────────────────────────────────

pub fn render_message(
    log: Res<CombatLog>,
    mut msg_q: Query<&mut Text, With<UiMessage>>,
) {
    let Ok(mut msg) = msg_q.single_mut() else {
        return;
    };
    let max_lines = 20usize;
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
